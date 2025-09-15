use axum::Router;
use crate::db::DbPool;
use crate::config::Config;

pub mod auth;
pub mod vocab;
pub mod practice;
pub mod logs;
pub mod media;
pub mod live;
pub mod ws;

#[allow(dead_code)]
pub fn router() -> Router<(DbPool, Config)> {
    Router::new()
        .merge(auth::router())
        .merge(vocab::router())
        .merge(practice::router())
        .merge(logs::router())
        .merge(media::router())
        .merge(live::router())
        .merge(ws::router())
}
