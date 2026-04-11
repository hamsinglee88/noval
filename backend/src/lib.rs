pub mod app_state; pub mod db; pub mod errors; pub mod handlers; pub mod models; pub mod services; pub mod validation;
use axum::{Router, routing::{get, post, put, delete}};
use tower_http::cors::{AllowOrigin, CorsLayer};
use crate::{app_state::AppState, handlers::*};

pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new().allow_methods(tower_http::cors::Any).allow_headers(tower_http::cors::Any)
        .allow_origin(AllowOrigin::list(["http://127.0.0.1:5173".parse().unwrap(), "http://localhost:5173".parse().unwrap()]));
    Router::new()
        .route("/api/auth/register", post(auth::register)).route("/api/auth/login", post(auth::login))
        .route("/api/auth/logout", post(auth::logout)).route("/api/auth/me", get(auth::me))
        .route("/api/styles/analyze", post(styles::analyze)).route("/api/styles/analyze/{task_id}", get(styles::get_task_status))
        .route("/api/style-profiles", get(style_profiles::list_style_profiles)).route("/api/style-profiles/save", post(style_profiles::save_style_profile))
        .route("/api/style-profiles/{id}", get(style_profiles::get_style_profile)).route("/api/style-profiles/{id}", delete(style_profiles::delete_style_profile))
        .route("/api/style-profiles/{id}/export", get(style_profiles::export_style_profile))
        .route("/api/projects", get(projects::list_projects)).route("/api/projects", post(projects::create_project))
        .route("/api/projects/{id}", get(projects::get_project)).route("/api/projects/{id}", put(projects::update_project)).route("/api/projects/{id}", delete(projects::delete_project))
        .route("/api/projects/{novel_id}/chapters", get(chapters::list_chapters)).route("/api/projects/{novel_id}/chapters", post(chapters::create_chapter))
        .route("/api/projects/{novel_id}/chapters/{chapter_id}", get(chapters::get_chapter)).route("/api/projects/{novel_id}/chapters/{chapter_id}", put(chapters::update_chapter))
        .route("/api/projects/{novel_id}/chapters/{chapter_id}", delete(chapters::delete_chapter))
        .route("/api/projects/{novel_id}/chapters/{chapter_id}/versions", get(versions::list_versions))
        .route("/api/projects/{novel_id}/chapters/{chapter_id}/versions/{version_id}", get(versions::get_version))
        .route("/api/projects/{novel_id}/chapters/{chapter_id}/versions/{version_id}/rollback", post(versions::rollback_version))
        .route("/api/projects/{novel_id}/export", get(export::export_novel))
        .route("/api/styles/mix/preview", post(style_mixing::preview_mix)).route("/api/styles/mix/save", post(style_mixing::save_mixed_style))
        .route("/api/styles/similar", get(style_similarity::find_similar))
        .route("/api/ai/continue", post(ai::continue_text))
        .route("/api/ai/polish", post(ai::polish_text))
        .route("/api/ai/expand", post(ai::expand_text))
        .route("/api/ai/summarize", post(ai::summarize_text))
        .route("/api/ai/rewrite", post(ai::rewrite_text))
        .route("/api/projects/{novel_id}/foreshadows", get(foreshadowing::list_foreshadows))
        .route("/api/projects/{novel_id}/chapters/{chapter_id}/detect-foreshadows", post(foreshadowing::detect_chapter_foreshadows))
        .with_state(state).layer(cors)
}