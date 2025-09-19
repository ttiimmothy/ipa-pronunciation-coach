use axum::{
  extract::{Path, State},
  response::Json,
  routing::post,
  Router,
};

use crate::config::Config;
use crate::db::DbPool;
use crate::models::{CreateRoom, InviteRequest, RoomResponse};

pub fn router() -> Router<(DbPool, Config)> {
  Router::new()
    .route("/", post(create_room))
    .route("/:room_id/invite", post(invite_to_room))
}

async fn create_room(
  State((_pool, _config)): State<(DbPool, Config)>,
  Json(_payload): Json<CreateRoom>,
) -> Json<RoomResponse> {
  // TODO: Implement live room creation
  Json(RoomResponse {
    room_id: uuid::Uuid::new_v4(),
    ws_url: "ws://localhost:8787/ws".to_string(),
  })
}

async fn invite_to_room(
  State((_pool, _config)): State<(DbPool, Config)>,
  Path(_room_id): Path<uuid::Uuid>,
  Json(_payload): Json<InviteRequest>,
) -> Json<serde_json::Value> {
  // TODO: Implement room invitation
  Json(serde_json::json!({"status": "ok"}))
}
