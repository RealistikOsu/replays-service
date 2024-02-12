use bytes::Bytes;
use std::path::Path;
use std::fs;
use std::time::Duration;

use anyhow::Result;
use anyhow::Error;
use s3::Bucket;
use s3::creds::Credentials;
use tokio::task;
use tokio::time;

/// Defines a storage adapter for a specific directory.
pub trait StorageAdapter {
    async fn save(&self, location: String, data: &Bytes) -> Result<()>;
    async fn load(&self, location: String) -> Result<Bytes>;
}

    pub struct LocalStorage {
    _location: String
}

impl LocalStorage {
    pub fn from_location(location: String) -> Result<Self> {
        let path = Path::new(&location);

        if !path.exists() {
            Err(Error::msg("The path provided does not exist."))
        } else {
            Ok(Self {
                _location: location,
            })
        }
    }
}

impl StorageAdapter for LocalStorage {
    async fn save(&self, location: String, data: &Bytes)  -> Result<()> {
        fs::write(location, data)?;

        Ok(())
    }

    async fn load(&self, location: String) -> Result<Bytes> {
        let bytes = Bytes::from(fs::read(location)?);

        Ok(bytes)
    }
}


pub struct S3Adapter {
    bucket: Bucket,
    credentials: Credentials,
    retries: usize
}

impl S3Adapter {
    pub fn from_credentials(
        credentials: Credentials,
        name: String,
        region: String,
        retries: usize,
    ) -> Result<Self> {
        let bucket = Bucket::new(
            name.as_str(),
            region.parse()?,
            credentials.clone(),
        )?;

        Ok(Self {
            bucket,
            credentials,
            retries,
        })
    }
}

async fn _save_retried(bucket: Bucket, location: String, data: Bytes, retries: usize) {
    for i in 0..retries {
        let result = bucket.put_object(
            location.clone(),
            &data,
        ).await;

        if result.is_ok() {
            return;
        }

        // Retry logic.
        time::sleep(Duration::from_millis(
            100 * (1.2_f32.powi(i as i32)) as u64
        )).await;
    }

    // TODO: logging
}

impl StorageAdapter for S3Adapter {
    async fn save(&self, location: String, data: &Bytes) -> Result<()> {
        // Asynchronously upload.
        task::spawn(_save_retried(
            self.bucket.clone(),
            location,
            data.clone(), // TODO: This copy sucks.
            self.retries,
        ));

        Ok(())
    }

    async fn load(&self, location: String) -> Result<Bytes> {
        let res = self.bucket.get_object(location).await?;

        Ok(res.bytes().to_owned())
    }
}
