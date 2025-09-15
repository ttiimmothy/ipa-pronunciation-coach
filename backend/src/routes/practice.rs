use axum::{
    extract::{Path, State},
    response::Json,
    routing::{get, post},
    Router,
};

use crate::config::Config;
use crate::db::DbPool;
use crate::models::{ScoreRequest, ScoreResponse, UsageTick, TimeCapSettings};

pub fn router() -> Router<(DbPool, Config)> {
    Router::new()
        .route("/score", post(score_recording))
        .route("/score/:recording_id", get(get_score))
        .route("/usage/tick", post(usage_tick))
        .route("/settings/timecap", post(update_timecap))
}

async fn score_recording(
    State((_pool, _config)): State<(DbPool, Config)>,
    Json(_payload): Json<ScoreRequest>,
) -> Json<ScoreResponse> {
    // TODO: Implement scoring job creation
    Json(ScoreResponse {
        status: crate::models::ScoreStatus::Pending,
        overall_pct: None,
        per_phoneme: None,
        job_id: Some("placeholder".to_string()),
    })
}

async fn get_score(
    State((_pool, _config)): State<(DbPool, Config)>,
    Path(_recording_id): Path<uuid::Uuid>,
) -> Json<ScoreResponse> {
    // TODO: Implement score retrieval
    Json(ScoreResponse {
        status: crate::models::ScoreStatus::Pending,
        overall_pct: None,
        per_phoneme: None,
        job_id: None,
    })
}

async fn usage_tick(
    State((_pool, _config)): State<(DbPool, Config)>,
    Json(_payload): Json<UsageTick>,
) -> Json<serde_json::Value> {
    // TODO: Implement usage tracking
    Json(serde_json::json!({"status": "ok"}))
}

async fn update_timecap(
    State((_pool, _config)): State<(DbPool, Config)>,
    Json(_payload): Json<TimeCapSettings>,
) -> Json<serde_json::Value> {
    // TODO: Implement time cap settings
    Json(serde_json::json!({"status": "ok"}))
}
