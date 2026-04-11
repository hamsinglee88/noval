use axum::{
    Json,
    extract::{State, Path},
    http::{HeaderMap, header},
};
use serde::Serialize;

use crate::{app_state::AppState, errors::AppError};

#[derive(Debug, Serialize)]
pub struct ApiSuccess<T> { pub success: bool, pub data: T }

#[derive(Debug, Serialize)]
pub struct VersionInfo {
    pub id: String,
    pub chapter_id: String,
    pub version_number: i64,
    pub word_count: i64,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct VersionDetail {
    pub id: String,
    pub chapter_id: String,
    pub content: String,
    pub version_number: i64,
    pub word_count: i64,
    pub created_at: String,
}

/// 获取章节版本历史
pub async fn list_versions(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((novel_id, chapter_id)): Path<(String, String)>,
) -> Result<Json<ApiSuccess<Vec<VersionInfo>>>, AppError> {
    let _user_id = get_user_id_from_token(&state.db, &headers).await?;

    let versions: Vec<(String, String, i64, i64, String)> = sqlx::query_as(
        "SELECT id, chapter_id, version_number, word_count, created_at FROM chapter_versions WHERE chapter_id = ? ORDER BY version_number DESC"
    )
    .bind(&chapter_id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch versions: {}", e);
        AppError::internal("获取版本历史失败")
    })?;

    let version_list: Vec<VersionInfo> = versions.into_iter().map(|v| {
        VersionInfo { id: v.0, chapter_id: v.1, version_number: v.2, word_count: v.3, created_at: v.4 }
    }).collect();

    Ok(Json(ApiSuccess { success: true, data: version_list }))
}

/// 获取版本详情
pub async fn get_version(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((novel_id, chapter_id, version_id)): Path<(String, String, String)>,
) -> Result<Json<ApiSuccess<VersionDetail>>, AppError> {
    let _user_id = get_user_id_from_token(&state.db, &headers).await?;

    let version: (String, String, String, i64, i64, String) = sqlx::query_as(
        "SELECT id, chapter_id, content, version_number, word_count, created_at FROM chapter_versions WHERE id = ? AND chapter_id = ?"
    )
    .bind(&version_id)
    .bind(&chapter_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch version: {}", e);
        AppError::internal("获取版本失败")
    })?
    .ok_or_else(|| AppError::not_found("VERSION_NOT_FOUND", "版本不存在"))?;

    Ok(Json(ApiSuccess {
        success: true,
        data: VersionDetail { id: version.0, chapter_id: version.1, content: version.2, version_number: version.3, word_count: version.4, created_at: version.5 },
    }))
}

async fn get_user_id_from_token(db: &sqlx::SqlitePool, headers: &HeaderMap) -> Result<String, AppError> {
    let token = bearer_token(headers)?;
    let user: (String,) = sqlx::query_as("SELECT id FROM users WHERE session_token = ? AND session_expires_at > datetime('now')")
        .bind(&token).fetch_optional(db).await?
        .ok_or_else(|| AppError::unauthorized("SESSION_EXPIRED", "登录状态已失效"))?;
    Ok(user.0)
}

fn bearer_token(headers: &HeaderMap) -> Result<String, AppError> {
    let value = headers.get(header::AUTHORIZATION).ok_or_else(|| AppError::unauthorized("MISSING_SESSION", "缺少登录凭证"))?;
    let value = value.to_str().map_err(|_| AppError::unauthorized("INVALID_SESSION", "登录凭证格式无效"))?;
    value.strip_prefix("Bearer ").map(ToOwned::to_owned).ok_or_else(|| AppError::unauthorized("INVALID_SESSION", "登录凭证格式无效"))
}