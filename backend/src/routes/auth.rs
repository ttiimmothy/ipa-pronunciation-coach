use axum::{
  extract::State,
  http::StatusCode,
  response::Json,
  routing::{get, post},
  Router,
};

use crate::auth::{create_jwt, hash_password, verify_password};
use crate::config::Config;
use crate::db::DbPool;
use crate::models::{AuthResponse, CreateUser, LoginUser, User};

pub fn router() -> Router<(DbPool, Config)> {
  Router::new()
    .route("/register", post(register))
    .route("/login", post(login))
    .route("/me", get(get_me))
}

async fn register(
  State((pool, config)): State<(DbPool, Config)>,
  Json(payload): Json<CreateUser>,
) -> Result<Json<AuthResponse>, StatusCode> {
  // Check if user already exists
  let existing_user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
    .bind(&payload.email)
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

  if existing_user.is_some() {
    return Err(StatusCode::CONFLICT);
  }

  // Hash password
  let pass_hash =
    hash_password(&payload.password).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

  // Create user
  let user = sqlx::query_as::<_, User>(
    "INSERT INTO users (email, pass_hash, name, dialect) VALUES ($1, $2, $3, $4) RETURNING *",
  )
  .bind(&payload.email)
  .bind(&pass_hash)
  .bind(&payload.name)
  .bind(payload.dialect.unwrap_or(crate::models::Dialect::GA))
  .fetch_one(&pool)
  .await
  .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

  // Create JWT
  let access_token =
    create_jwt(user.id, &config.jwt_secret).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

  Ok(Json(AuthResponse { access_token, user }))
}

async fn login(
  State((pool, config)): State<(DbPool, Config)>,
  Json(payload): Json<LoginUser>,
) -> Result<Json<AuthResponse>, StatusCode> {
  // Find user
  let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
    .bind(&payload.email)
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::UNAUTHORIZED)?;

  // Verify password
  if !verify_password(&payload.password, &user.pass_hash)
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
  {
    return Err(StatusCode::UNAUTHORIZED);
  }

  // Create JWT
  let access_token =
    create_jwt(user.id, &config.jwt_secret).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

  Ok(Json(AuthResponse { access_token, user }))
}

async fn get_me(
  State((_pool, _config)): State<(DbPool, Config)>,
  user: User,
) -> Result<Json<User>, StatusCode> {
  Ok(Json(user))
}
