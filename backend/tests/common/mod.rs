use anyhow::Result;
use axum::{
    body::Body,
    http::Request,
    Router,
};
use sqlx::PgPool;

use ipa_backend::config::Config;
use ipa_backend::db::create_pool;
use ipa_backend::routes::router;

/// Create a test application
pub async fn create_test_app() -> Result<Router> {
    // Load test configuration
    let config = Config {
        database_url: "postgres://postgres:postgres@localhost:5432/ipa_test".to_string(),
        jwt_secret: "test_secret".to_string(),
        s3_endpoint: "http://localhost:9000".to_string(),
        s3_bucket: "test-bucket".to_string(),
        s3_access_key: "test".to_string(),
        s3_secret_key: "test".to_string(),
        s3_region: "us-east-1".to_string(),
        redis_url: "redis://localhost:6379/1".to_string(),
        meilisearch_url: "http://localhost:7700".to_string(),
        meilisearch_key: "test_key".to_string(),
        client_url: "http://localhost:4321".to_string(),
        allow_dev_google_sso: false,
    };
    
    // Create database pool
    let pool = create_pool(&config.database_url).await?;
    
    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;
    
    // Create app
    let app = router().with_state((pool, config));
    
    Ok(app)
}

/// Create a test request
#[allow(dead_code)]
pub fn create_test_request(uri: &str, method: &str, body: Option<Body>) -> Request<Body> {
    let body = body.unwrap_or_else(Body::empty);
    Request::builder()
        .uri(uri)
        .method(method)
        .body(body)
        .unwrap()
}

/// Setup test database
#[allow(dead_code)]
pub async fn setup_test_db() -> Result<PgPool> {
    let database_url = "postgres://postgres:postgres@localhost:5432/ipa_test";
    let pool = create_pool(database_url).await?;
    
    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;
    
    Ok(pool)
}

/// Cleanup test database
#[allow(dead_code)]
pub async fn cleanup_test_db(pool: &PgPool) -> Result<()> {
    // Drop all tables
    sqlx::query("DROP SCHEMA public CASCADE; CREATE SCHEMA public;")
        .execute(pool)
        .await?;
    
    Ok(())
}
