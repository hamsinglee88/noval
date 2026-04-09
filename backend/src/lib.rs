pub mod app_state;
pub mod db;
pub mod errors;
pub mod handlers;
pub mod models;
pub mod services;
pub mod validation;

use axum::{
    Router,
    routing::{get, post},
};
use tower_http::cors::{AllowOrigin, CorsLayer};

use crate::{app_state::AppState, handlers::auth};

pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any)
        .allow_origin(AllowOrigin::list([
            "http://127.0.0.1:5173".parse().expect("valid origin"),
            "http://localhost:5173".parse().expect("valid origin"),
        ]));

    Router::new()
        .route("/api/auth/register", post(auth::register))
        .route("/api/auth/login", post(auth::login))
        .route("/api/auth/logout", post(auth::logout))
        .route("/api/auth/me", get(auth::me))
        .with_state(state)
        .layer(cors)
}
