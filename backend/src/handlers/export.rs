use axum::{Json, extract::{State, Path}, http::{HeaderMap, header}};
use crate::{app_state::AppState, errors::AppError};
#[derive(serde::Serialize)] pub struct ApiSuccess<T> { pub success: bool, pub data: T }
pub async fn export_novel(State(state): State<AppState>, headers: HeaderMap, Path(novel_id): Path<String>) -> Result<Json<ApiSuccess<serde_json::Value>>, AppError> {
    let _ = get_user_id_from_token(&state.db, &headers).await?;
    let chapters: Vec<(String, String)> = sqlx::query_as("SELECT title, content FROM chapters WHERE novel_id = ? ORDER BY chapter_number")
        .bind(&novel_id).fetch_all(&state.db).await.map_err(|e| { tracing::error!("{}", e); AppError::internal("导出失败") })?;
    let content: String = chapters.iter().map(|(t, c)| format!("{}\n\n{}", t, c)).collect::<Vec<_>>().join("\n\n---\n\n");
    Ok(Json(ApiSuccess { success: true, data: serde_json::json!({"content": content, "format": "txt"}) }))
}
async fn get_user_id_from_token(db: &sqlx::SqlitePool, headers: &HeaderMap) -> Result<String, AppError> {
    let token = headers.get(header::AUTHORIZATION).ok_or_else(|| AppError::unauthorized("MISSING", ""))?.to_str().map_err(|_| AppError::unauthorized("INVALID", ""))?.strip_prefix("Bearer ").unwrap_or("");
    let user: (String,) = sqlx::query_as("SELECT id FROM users WHERE session_token = ?").bind(token).fetch_optional(db).await?.ok_or_else(|| AppError::unauthorized("EXPIRED", ""))?;
    Ok(user.0)
}
