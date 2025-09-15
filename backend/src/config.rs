use anyhow::Result;
use std::env;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub s3_endpoint: String,
    pub s3_bucket: String,
    pub s3_access_key: String,
    pub s3_secret_key: String,
    pub s3_region: String,
    pub redis_url: String,
    pub meilisearch_url: String,
    pub meilisearch_key: String,
    pub allow_dev_google_sso: bool,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        // Load .env file if it exists
        let _ = dotenvy::dotenv();
        
        Ok(Self {
            database_url: env::var("DATABASE_URL").unwrap(),
                // .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ipa_pronunciation_coach".to_string()),
            jwt_secret: env::var("JWT_SECRET").unwrap(),
                // .unwrap_or_else(|_| "dev_secret_change_me".to_string()),
            s3_endpoint: env::var("S3_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:9000".to_string()),
            s3_bucket: env::var("S3_BUCKET")
                .unwrap_or_else(|_| "ipa-media".to_string()),
            s3_access_key: env::var("S3_ACCESS_KEY")
                .unwrap_or_else(|_| "minioadmin".to_string()),
            s3_secret_key: env::var("S3_SECRET_KEY")
                .unwrap_or_else(|_| "minioadmin".to_string()),
            s3_region: env::var("S3_REGION")
                .unwrap_or_else(|_| "us-east-1".to_string()),
            redis_url: env::var("REDIS_URL").unwrap(),
                // .unwrap_or_else(|_| "redis://localhost:6379/0".to_string()),
            meilisearch_url: env::var("MEILISEARCH_URL")
                .unwrap_or_else(|_| "http://localhost:7700".to_string()),
            meilisearch_key: env::var("MEILISEARCH_KEY")
                .unwrap_or_else(|_| "masterKey".to_string()),
            allow_dev_google_sso: env::var("ALLOW_DEV_GOOGLE_SSO")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
        })
    }
}
