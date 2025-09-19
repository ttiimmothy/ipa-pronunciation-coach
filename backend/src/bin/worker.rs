use anyhow::Result;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Import from the parent crate
use ipa_backend::config::Config;
use ipa_backend::db::create_pool;
use ipa_backend::jobs::JobWorker;

#[tokio::main]
async fn main() -> Result<()> {
  // Initialize tracing
  tracing_subscriber::registry()
    .with(
      tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "ipa_backend=debug,jobs=debug".into()),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();

  // Load configuration
  let config = Config::from_env()?;

  // Create database pool
  let pool = create_pool(&config.database_url).await?;

  // Create and start job worker
  let mut worker = JobWorker::new(&config.redis_url, pool)?;

  // Handle shutdown signal
  tokio::spawn(async {
    tokio::signal::ctrl_c()
      .await
      .expect("Failed to listen for ctrl+c");
    tracing::info!("Shutdown signal received");
    std::process::exit(0);
  });

  // Start the worker
  worker.start().await?;

  Ok(())
}
