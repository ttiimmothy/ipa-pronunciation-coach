use axum::{
  extract::{State, WebSocketUpgrade},
  response::Response,
  routing::get,
  Router,
};

use crate::config::Config;
use crate::db::DbPool;

pub fn router() -> Router<(DbPool, Config)> {
  Router::new().route("/", get(websocket_handler))
}

async fn websocket_handler(
  State((_pool, _config)): State<(DbPool, Config)>,
  ws: WebSocketUpgrade,
) -> Response {
  // TODO: Implement WebSocket handler for real-time features
  ws.on_upgrade(|_socket| async move {
    // Handle WebSocket connection
  })
}
