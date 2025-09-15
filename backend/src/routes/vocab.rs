use axum::{
    extract::{Path, Query, State},
    response::Json,
    routing::{get, post},
    Router,
};

use crate::config::Config;
use crate::db::DbPool;
use crate::models::{VocabSearchQuery, WordWithVariants};

pub fn router() -> Router<(DbPool, Config)> {
    Router::new()
        .route("/", get(search_vocab))
        .route("/:id", get(get_word))
        .route("/", post(create_word))
}

async fn search_vocab(
    State((_pool, _config)): State<(DbPool, Config)>,
    Query(_query): Query<VocabSearchQuery>,
) -> Json<Vec<WordWithVariants>> {
    // TODO: Implement vocabulary search with Meilisearch
    Json(vec![])
}

async fn get_word(
    State((_pool, _config)): State<(DbPool, Config)>,
    Path(_word_id): Path<uuid::Uuid>,
) -> Json<Option<WordWithVariants>> {
    // TODO: Implement word detail retrieval
    Json(None)
}

async fn create_word(
    State((_pool, _config)): State<(DbPool, Config)>,
    Json(word_data): Json<WordWithVariants>,
) -> Json<WordWithVariants> {
    // TODO: Implement word creation
    Json(word_data)
}
