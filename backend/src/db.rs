use sqlx::{PgPool, Pool, Postgres};
use anyhow::Result;

pub async fn create_pool(database_url: &str) -> Result<PgPool> {
    let pool = PgPool::connect(database_url).await?;
    Ok(pool)
}

pub type DbPool = Pool<Postgres>;
