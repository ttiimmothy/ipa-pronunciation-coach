use super::{Dialect, PartOfSpeech};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Word {
    pub id: Uuid,
    pub text: String,
    pub language: String,
    pub pos: Option<PartOfSpeech>,
    pub difficulty: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DialectVariant {
    pub id: Uuid,
    pub word_id: Uuid,
    pub dialect: Dialect,
    pub ipa: String,
    pub audio_url: Option<String>,
    pub video_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Phoneme {
    pub id: Uuid,
    pub symbol: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WordPhoneme {
    pub word_id: Uuid,
    pub phoneme_id: Uuid,
    pub order_index: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MinimalPair {
    pub id: Uuid,
    pub word_a_id: Uuid,
    pub word_b_id: Uuid,
    pub phoneme_diff: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordWithVariants {
    #[serde(flatten)]
    pub word: Word,
    pub variants: Vec<DialectVariant>,
    pub phonemes: Vec<Phoneme>,
    pub minimal_pairs: Vec<MinimalPair>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWord {
    pub text: String,
    pub language: String,
    pub pos: Option<PartOfSpeech>,
    pub difficulty: i32,
    pub variants: Vec<CreateDialectVariant>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDialectVariant {
    pub dialect: Dialect,
    pub ipa: String,
    pub audio_url: Option<String>,
    pub video_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VocabSearchQuery {
    pub query: Option<String>,
    pub dialect: Option<Dialect>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}
