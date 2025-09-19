use crate::config::Config;
use crate::db::DbPool;
use axum::Router;

pub mod auth;
pub mod live;
pub mod logs;
pub mod media;
pub mod practice;
pub mod vocab;
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
