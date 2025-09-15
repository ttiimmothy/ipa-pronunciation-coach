use axum::{
    extract::DefaultBodyLimit,
    http::{HeaderValue, Method},
    routing::get,
    Router,
};
use sqlx::PgPool;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod auth;
mod config;
mod db;
mod jobs;
mod models;
mod routes;
mod services;

use config::Config;
use db::create_pool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ipa_backend=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env()?;

    // Create database pool
    let pool = create_pool(&config.database_url).await?;

    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    // Build application
    let app = create_app(pool, config).await?;

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8787));
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn create_app(pool: PgPool, config: Config) -> anyhow::Result<Router> {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any)
        .allow_origin([
            "http://localhost:4320".parse::<HeaderValue>()?,
            "http://localhost:3000".parse::<HeaderValue>()?, // Add your second origin here
        ]);

    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/auth", routes::auth::router())
        .nest("/vocab", routes::vocab::router())
        .nest("/practice", routes::practice::router())
        .nest("/logs", routes::logs::router())
        .nest("/media", routes::media::router())
        .nest("/live", routes::live::router())
        .nest("/ws", routes::ws::router())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(cors),
        )
        .with_state((pool, config))
        .layer(DefaultBodyLimit::max(50 * 1024 * 1024)); // 50MB limit for audio uploads

    Ok(app)
}

async fn health_check() -> &'static str {
    "OK"
}
