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

use crate::{app_state::AppState, handlers::{auth, styles, style_profiles}};

pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any)
        .allow_origin(AllowOrigin::list([
            "http://127.0.0.1:5173".parse().expect("valid origin"),
            "http://localhost:5173".parse().expect("valid origin"),
            "http://127.0.0.1:5174".parse().expect("valid origin"),
            "http://localhost:5174".parse().expect("valid origin"),
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
        .route("/api/styles/analyze/{task_id}/vocabulary", get(styles::get_vocabulary_result))
        .route("/api/styles/analyze/{task_id}/sentence", get(styles::get_sentence_result))
        .route("/api/styles/analyze/{task_id}/rhetoric", get(styles::get_rhetoric_result))
        .route("/api/styles/analyze/{task_id}/narrative", get(styles::get_narrative_result))
        .route("/api/styles/analyze/{task_id}/emotion", get(styles::get_emotion_result))
        .route("/api/styles/analyze/{task_id}/pacing", get(styles::get_pacing_result))
        // Style Profile routes
        .route("/api/style-profiles", get(style_profiles::list_style_profiles))
        .route("/api/style-profiles/save", post(style_profiles::save_style_profile))
        .route("/api/style-profiles/import", post(style_profiles::import_style_profile))
        .route("/api/style-profiles/{id}", get(style_profiles::get_style_profile))
        .route("/api/style-profiles/{id}", axum::routing::delete(style_profiles::delete_style_profile))
        .route("/api/style-profiles/{id}/export", get(style_profiles::export_style_profile))
        .with_state(state)
        .layer(cors)
}
