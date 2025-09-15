use axum::{extract::State, response::Json, routing::get, Router};

use crate::config::Config;
use crate::db::DbPool;

pub fn router() -> Router<(DbPool, Config)> {
    Router::new().route("/daily", get(get_daily_logs))
}

async fn get_daily_logs(
    State((_pool, _config)): State<(DbPool, Config)>,
) -> Json<serde_json::Value> {
    // TODO: Implement daily logs retrieval
    Json(serde_json::json!({"logs": []}))
}
