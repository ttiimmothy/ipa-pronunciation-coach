use anyhow::Result;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::json;
use tower::Service;

mod common;

use common::*;

#[tokio::test]
async fn test_health_check() -> Result<()> {
    let mut app = create_test_app().await?;

    let request = Request::builder()
        .uri("/health")
        .method("GET")
        .body(Body::empty())?;

    let response = app.call(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_user_registration() -> Result<()> {
    let mut app = create_test_app().await?;

    let user_data = json!({
        "email": "test@example.com",
        "password": "password123",
        "name": "Test User",
        "dialect": "GA"
    });

    let request = Request::builder()
        .uri("/api/auth/register")
        .method("POST")
        .header("content-type", "application/json")
        .body(Body::from(user_data.to_string()))?;

    let response = app.call(request).await?;
    // User registration should either succeed (200) or conflict if user exists (409)
    assert!(response.status() == StatusCode::OK || response.status() == StatusCode::CONFLICT);

    Ok(())
}

#[tokio::test]
async fn test_user_login() -> Result<()> {
    let mut app = create_test_app().await?;

    // First register a user
    let user_data = json!({
        "email": "login@example.com",
        "password": "password123",
        "name": "Login User"
    });

    let register_request = Request::builder()
        .uri("/api/auth/register")
        .method("POST")
        .header("content-type", "application/json")
        .body(Body::from(user_data.to_string()))?;

    let _response = app.clone().call(register_request).await?;

    // Then login
    let login_data = json!({
        "email": "login@example.com",
        "password": "password123"
    });

    let login_request = Request::builder()
        .uri("/api/auth/login")
        .method("POST")
        .header("content-type", "application/json")
        .body(Body::from(login_data.to_string()))?;

    let response = app.call(login_request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_vocabulary_search() -> Result<()> {
    let mut app = create_test_app().await?;

    let request = Request::builder()
        .uri("/api/vocab?query=hello")
        .method("GET")
        .body(Body::empty())?;

    let response = app.call(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_pronunciation_scoring() -> Result<()> {
    let mut app = create_test_app().await?;

    let score_data = json!({
        "recording_id": "123e4567-e89b-12d3-a456-426614174000",
        "word_id": "123e4567-e89b-12d3-a456-426614174001",
        "dialect": "GA"
    });

    let request = Request::builder()
        .uri("/api/practice/score")
        .method("POST")
        .header("content-type", "application/json")
        .body(Body::from(score_data.to_string()))?;

    let response = app.call(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_usage_tracking() -> Result<()> {
    let mut app = create_test_app().await?;

    let usage_data = json!({
        "active_ms": 5000
    });

    let request = Request::builder()
        .uri("/api/practice/usage/tick")
        .method("POST")
        .header("content-type", "application/json")
        .body(Body::from(usage_data.to_string()))?;

    let response = app.call(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_live_room_creation() -> Result<()> {
    let mut app = create_test_app().await?;

    let room_data = json!({
        "name": "Test Room"
    });

    let request = Request::builder()
        .uri("/api/live")
        .method("POST")
        .header("content-type", "application/json")
        .body(Body::from(room_data.to_string()))?;

    let response = app.call(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_media_upload() -> Result<()> {
    let mut app = create_test_app().await?;

    let request = Request::builder()
        .uri("/api/media/recordings")
        .method("POST")
        .header("content-type", "multipart/form-data")
        .body(Body::empty())?;

    let response = app.call(request).await?;
    // Should return 400 for empty body, but endpoint exists
    assert!(response.status() == StatusCode::BAD_REQUEST || response.status() == StatusCode::OK);

    Ok(())
}
