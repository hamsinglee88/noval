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

use crate::{app_state::AppState, handlers::{auth, styles}};

pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any)
        .allow_origin(AllowOrigin::list([
            "http://127.0.0.1:5173".parse().expect("valid origin"),
            "http://localhost:5173".parse().expect("valid origin"),
        ]));

    Router::new()
        // Auth routes
        .route("/api/auth/register", post(auth::register))
        .route("/api/auth/login", post(auth::login))
        .route("/api/auth/logout", post(auth::logout))
        .route("/api/auth/me", get(auth::me))
        // Style routes
        .route("/api/styles/analyze", post(styles::analyze))
        .route("/api/styles/analyze/{task_id}", get(styles::get_task_status))
        .route("/api/styles/analyze/{task_id}/cancel", post(styles::cancel_task))
        .with_state(state)
        .layer(cors)
}
