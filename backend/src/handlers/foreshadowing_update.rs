use axum::{Json, extract::{State, Path}, http::{HeaderMap, header}};
use serde::{Deserialize, Serialize};
use crate::{app_state::AppState, errors::AppError};

#[derive(Debug, Serialize)] pub struct ApiSuccess<T> { pub success: bool, pub data: T }

#[derive(Debug, Deserialize)]
pub struct UpdateStatusRequest {
    pub status: String,  // Active, Resolved, Abandoned
    pub resolution_chapter_id: Option<String>,
}

pub async fn update_foreshadow_status(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((novel_id, foreshadow_id)): Path<(String, String)>,
    Json(req): Json<UpdateStatusRequest>,
) -> Result<Json<ApiSuccess<serde_json::Value>>, AppError> {
    let user_id = get_user_id_from_token(&state.db, &headers).await?;
    
    // 验证用户拥有该小说
    let novel_owner: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM novels WHERE id = ? AND user_id = ?")
        .bind(&novel_id)
        .bind(&user_id)
        .fetch_one(&state.db)
        .await
        .map_err(|e| { tracing::error!("{}", e); AppError::internal("验证权限失败") })?;
    
    if novel_owner.0 == 0 {
        return Err(AppError::not_found("NOVEL_NOT_FOUND", "小说不存在或无权访问"));
    }
    
    let result = sqlx::query("UPDATE foreshadows SET status = ?, resolution_chapter_id = ?, resolved_at = datetime('now') WHERE id = ? AND novel_id = ?")
        .bind(&req.status)
        .bind(&req.resolution_chapter_id)
        .bind(&foreshadow_id)
        .bind(&novel_id)
        .execute(&state.db)
        .await
        .map_err(|e| { tracing::error!("{}", e); AppError::internal("更新伏笔状态失败") })?;
    
    if result.rows_affected() == 0 {
        return Err(AppError::not_found("FORESHADOW_NOT_FOUND", "伏笔不存在"));
    }
    
    Ok(Json(ApiSuccess { success: true, data: serde_json::json!({"message": "状态已更新"}) }))
}

async fn get_user_id_from_token(db: &sqlx::SqlitePool, headers: &HeaderMap) -> Result<String, AppError> {
    let token = headers.get(header::AUTHORIZATION)
        .ok_or_else(|| AppError::unauthorized("MISSING_SESSION", "缺少登录凭证"))?
        .to_str()
        .map_err(|_| AppError::unauthorized("INVALID_SESSION", "登录凭证格式无效"))?
        .strip_prefix("Bearer ")
        .ok_or_else(|| AppError::unauthorized("INVALID_SESSION", "登录凭证格式无效"))?;
    
    let user: (String,) = sqlx::query_as("SELECT id FROM users WHERE session_token = ? AND session_expires_at > datetime('now')")
        .bind(token)
        .fetch_optional(db)
        .await?
        .ok_or_else(|| AppError::unauthorized("SESSION_EXPIRED", "登录状态已失效"))?;
    
    Ok(user.0)
}