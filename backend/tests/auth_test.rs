use axum::{body::Body, http::{Request, StatusCode}};
use bcrypt::verify;
use http_body_util::BodyExt;
use noval_backend::{app_state::AppState, create_router, db};
use serde_json::{Value, json};
use sqlx::Row;
use tower::ServiceExt;

#[tokio::test]
async fn test_user_registration_success() {
    let (app, _db_path) = test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "username": "writer123",
                        "password": "SecurePass123"
                    })
                    .to_string(),
                ))
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.expect("body").to_bytes();
    let payload: Value = serde_json::from_slice(&body).expect("json");

    assert_eq!(payload["success"], true);
    assert_eq!(payload["data"]["user"]["username"], "writer123");
    assert!(payload["data"]["session"]["token"].is_string());
}

#[tokio::test]
async fn test_user_registration_duplicate_username() {
    let (app, _db_path) = test_app().await;
    register_user(&app, "repeat_user", "Repeat123").await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "username": "repeat_user",
                        "password": "Repeat123"
                    })
                    .to_string(),
                ))
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn test_user_login_success() {
    let (app, _db_path) = test_app().await;
    register_user(&app, "login_user", "Login123").await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "username": "login_user",
                        "password": "Login123"
                    })
                    .to_string(),
                ))
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.expect("body").to_bytes();
    let payload: Value = serde_json::from_slice(&body).expect("json");
    assert_eq!(payload["data"]["user"]["username"], "login_user");
    assert!(payload["data"]["user"]["last_login_at"].is_string());
}

#[tokio::test]
async fn test_user_login_invalid_credentials() {
    let (app, _db_path) = test_app().await;
    register_user(&app, "bad_login_user", "Secure123").await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "username": "bad_login_user",
                        "password": "Wrong1234"
                    })
                    .to_string(),
                ))
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_password_hashing() {
    let (_app, db_path) = test_app().await;
    let database_url = format!("sqlite://{}", db_path.display());
    let pool = db::create_pool(&database_url).await.expect("pool");
    db::run_migrations(&pool).await.expect("migrations");

    sqlx::query(
        "INSERT INTO users (
            id, username, password_hash, created_at, updated_at, last_login_at, session_token, session_expires_at
         ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind("user-1")
    .bind("hash_user")
    .bind(bcrypt::hash("Secure123", bcrypt::DEFAULT_COST).expect("hash"))
    .bind("0000000001Z")
    .bind("0000000001Z")
    .bind("0000000001Z")
    .bind("token-1")
    .bind("0000604801Z")
    .execute(&pool)
    .await
    .expect("seed");

    let row = sqlx::query("SELECT password_hash FROM users WHERE username = ?")
        .bind("hash_user")
        .fetch_one(&pool)
        .await
        .expect("select");

    let stored_hash: String = row.get("password_hash");
    assert_ne!(stored_hash, "Secure123");
    assert!(verify("Secure123", &stored_hash).expect("verify"));
}

async fn test_app() -> (axum::Router, std::path::PathBuf) {
    let temp_dir = tempfile::tempdir().expect("temp dir");
    let db_path = temp_dir.path().join("noval-test.db");
    let database_url = format!("sqlite://{}", db_path.display());
    let pool = db::create_pool(&database_url).await.expect("pool");
    db::run_migrations(&pool).await.expect("migrations");
    let app = create_router(AppState { db: pool });

    std::mem::forget(temp_dir);
    (app, db_path)
}

async fn register_user(app: &axum::Router, username: &str, password: &str) {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "username": username,
                        "password": password
                    })
                    .to_string(),
                ))
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::OK);
}
