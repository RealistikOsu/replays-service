use std::env;
use pub_this::pub_this;

use anyhow::Result;

#[pub_this]
pub struct Config {
    s3_region: String,
    s3_endpoint: String,
    s3_access_key: String,
    s3_secret_key: String,
    s3_bucket: String,
    s3_retries: i32,

    http_host: String,
    http_port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            s3_region: env::var("S3_REGION")?,
            s3_endpoint: env::var("S3_ENDPOINT")?,
            s3_access_key: env::var("S3_ACCESS_KEY")?,
            s3_secret_key: env::var("S3_SECRET_KEY")?,
            s3_bucket: env::var("S3_BUCKET")?,
            s3_retries: env::var("S3_RETRIES")?.parse()?,
            http_host: env::var("HTTP_HOST")?,
            http_port: env::var("HTTP_PORT")?.parse()?,
        })
    }
}
