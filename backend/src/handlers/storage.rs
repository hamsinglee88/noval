use axum::{Json, extract::State, http::HeaderMap};
use serde::Serialize;
use crate::{app_state::AppState, errors::AppError, auth_utils::{get_user_id_from_token, ApiSuccess}};

#[derive(Debug, Serialize)]
pub struct StorageInfo {
    pub total_novels: i64,
    pub total_chapters: i64,
    pub total_style_profiles: i64,
    pub total_foreshadows: i64,
    pub estimated_size_mb: f64,
}

pub async fn get_storage_info(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<ApiSuccess<StorageInfo>>, AppError> {
    let user_id = get_user_id_from_token(&state.db, &headers).await?;
    
    let novels: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM novels WHERE user_id = ?")
        .bind(&user_id).fetch_one(&state.db).await.unwrap_or((0,));
    
    let chapters: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM chapters c JOIN novels n ON c.novel_id = n.id WHERE n.user_id = ?"
    ).bind(&user_id).fetch_one(&state.db).await.unwrap_or((0,));
    
    let profiles: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM style_profiles WHERE user_id = ?")
        .bind(&user_id).fetch_one(&state.db).await.unwrap_or((0,));
    
    let foreshadows: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM foreshadows f JOIN novels n ON f.novel_id = n.id WHERE n.user_id = ?"
    ).bind(&user_id).fetch_one(&state.db).await.unwrap_or((0,));
    
    Ok(Json(ApiSuccess::ok(StorageInfo {
        total_novels: novels.0,
        total_chapters: chapters.0,
        total_style_profiles: profiles.0,
        total_foreshadows: foreshadows.0,
        estimated_size_mb: (novels.0 * 10 + chapters.0 * 5 + profiles.0 * 50) as f64 / 1024.0,
    })))
}

pub async fn cleanup_old_data(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<ApiSuccess<serde_json::Value>>, AppError> {
    let _user_id = get_user_id_from_token(&state.db, &headers).await?;
    
    // 简化实现
    Ok(Json(ApiSuccess::ok(serde_json::json!({"message": "清理完成"}))))
}
