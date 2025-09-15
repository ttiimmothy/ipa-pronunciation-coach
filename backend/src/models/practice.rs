use super::Dialect;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PracticeSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub total_ms: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Recording {
    pub id: Uuid,
    pub user_id: Uuid,
    pub session_id: Option<Uuid>,
    pub word_id: Uuid,
    pub dialect: Dialect,
    pub media_url: String,
    pub duration_ms: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Score {
    pub id: Uuid,
    pub recording_id: Uuid,
    pub overall_pct: rust_decimal::Decimal,
    pub per_phoneme: serde_json::Value,
    pub latency_ms: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DailyUsage {
    pub user_id: Uuid,
    pub date: chrono::NaiveDate,
    pub active_ms: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRecording {
    pub word_id: Uuid,
    pub dialect: Dialect,
    pub duration_ms: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreRequest {
    pub recording_id: Uuid,
    pub word_id: Uuid,
    pub dialect: Dialect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreResponse {
    pub status: ScoreStatus,
    pub overall_pct: Option<rust_decimal::Decimal>,
    pub per_phoneme: Option<serde_json::Value>,
    pub job_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScoreStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageTick {
    pub active_ms: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeCapSettings {
    pub minutes: i32,
}
