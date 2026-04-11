use axum::{Json, extract::{State, Path}, http::HeaderMap};
use serde::{Deserialize, Serialize};
use crate::{app_state::AppState, errors::AppError, auth_utils::{get_user_id_from_token, verify_novel_ownership, ApiSuccess}};

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
    verify_novel_ownership(&state.db, &user_id, &novel_id).await?;
    
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
    
    Ok(Json(ApiSuccess::ok(serde_json::json!({"message": "状态已更新"}))))
}