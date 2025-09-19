use axum::{extract::State, response::Json, routing::post, Router};

use crate::config::Config;
use crate::db::DbPool;

pub fn router() -> Router<(DbPool, Config)> {
  Router::new().route("/recordings", post(upload_recording))
}

async fn upload_recording(
  State((_pool, _config)): State<(DbPool, Config)>,
) -> Json<serde_json::Value> {
  // TODO: Implement recording upload to S3/MinIO
  Json(serde_json::json!({
      "recording_id": "placeholder",
      "media_url": "placeholder"
  }))
}
