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
    let _ = get_user_id_from_token(&state.db, &headers).await?;
    
    sqlx::query("UPDATE foreshadows SET status = ?, resolution_chapter_id = ?, resolved_at = datetime('now') WHERE id = ? AND novel_id = ?")
        .bind(&req.status)
        .bind(&req.resolution_chapter_id)
        .bind(&foreshadow_id)
        .bind(&novel_id)
        .execute(&state.db)
        .await
        .map_err(|e| { tracing::error!("{}", e); AppError::internal("更新伏笔状态失败") })?;
    
    Ok(Json(ApiSuccess { success: true, data: serde_json::json!({"message": "状态已更新"}) }))
}

async fn get_user_id_from_token(db: &sqlx::SqlitePool, headers: &HeaderMap) -> Result<String, AppError> {
    let token = headers.get(header::AUTHORIZATION).ok_or_else(|| AppError::unauthorized("MISSING", ""))?.to_str().map_err(|_| AppError::unauthorized("INVALID", ""))?.strip_prefix("Bearer ").unwrap_or("");
    let user: (String,) = sqlx::query_as("SELECT id FROM users WHERE session_token = ?").bind(token).fetch_optional(db).await?.ok_or_else(|| AppError::unauthorized("EXPIRED", ""))?;
    Ok(user.0)
}
