use anyhow::Result;
use sqlx::{PgPool, Pool, Postgres};

pub async fn create_pool(database_url: &str) -> Result<PgPool> {
    let pool = PgPool::connect(database_url).await?;
    Ok(pool)
}

pub type DbPool = Pool<Postgres>;
