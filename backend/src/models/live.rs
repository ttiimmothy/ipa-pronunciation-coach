use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LiveRoom {
  pub id: Uuid,
  pub host_id: Uuid,
  pub created_at: DateTime<Utc>,
  pub closed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LiveMember {
  pub room_id: Uuid,
  pub user_id: Uuid,
  pub joined_at: DateTime<Utc>,
  pub left_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Invite {
  pub id: Uuid,
  pub email: String,
  pub issuer_id: Uuid,
  pub token: String,
  pub expires_at: DateTime<Utc>,
  pub accepted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoom {
  pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomResponse {
  pub room_id: Uuid,
  pub ws_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InviteRequest {
  pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketMessage {
  pub event: String,
  pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreCompletedEvent {
  pub recording_id: Uuid,
  pub overall_pct: rust_decimal::Decimal,
  pub per_phoneme: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomUserJoinedEvent {
  pub room_id: Uuid,
  pub user_id: Uuid,
  pub user_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomUserLeftEvent {
  pub room_id: Uuid,
  pub user_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushToTalkEvent {
  pub room_id: Uuid,
  pub audio_data: Vec<u8>,
}
