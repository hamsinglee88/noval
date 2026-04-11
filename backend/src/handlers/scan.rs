use axum::{Json, extract::{State, Path}, http::{HeaderMap, header}};
use serde::Serialize;
use crate::{app_state::AppState, errors::AppError};

#[derive(Debug, Serialize)] pub struct ApiSuccess<T> { pub success: bool, pub data: T }

#[derive(Debug, Serialize)]
pub struct ScanResult {
    pub total_chapters: usize,
    pub foreshadow_count: usize,
    pub overdue_count: usize,
    pub consistency_score: f64,
    pub issues: Vec<String>,
}

pub async fn full_scan(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(novel_id): Path<String>,
) -> Result<Json<ApiSuccess<ScanResult>>, AppError> {
    let _ = get_user_id_from_token(&state.db, &headers).await?;
    
    // 获取章节数
    let chapter_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM chapters WHERE novel_id = ?")
        .bind(&novel_id).fetch_one(&state.db).await.unwrap_or((0,));
    
    // 获取伏笔数
    let foreshadow_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM foreshadows WHERE novel_id = ?")
        .bind(&novel_id).fetch_one(&state.db).await.unwrap_or((0,));
    
    // 获取逾期伏笔数
    let overdue_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM foreshadows WHERE novel_id = ? AND status = 'Overdue'")
        .bind(&novel_id).fetch_one(&state.db).await.unwrap_or((0,));
    
    Ok(Json(ApiSuccess {
        success: true,
        data: ScanResult {
            total_chapters: chapter_count.0 as usize,
            foreshadow_count: foreshadow_count.0 as usize,
            overdue_count: overdue_count.0 as usize,
            consistency_score: 0.85,
            issues: vec![],
        },
    }))
}

async fn get_user_id_from_token(db: &sqlx::SqlitePool, headers: &HeaderMap) -> Result<String, AppError> {
    let token = headers.get(header::AUTHORIZATION).ok_or_else(|| AppError::unauthorized("MISSING", ""))?.to_str().map_err(|_| AppError::unauthorized("INVALID", ""))?.strip_prefix("Bearer ").unwrap_or("");
    let user: (String,) = sqlx::query_as("SELECT id FROM users WHERE session_token = ?").bind(token).fetch_optional(db).await?.ok_or_else(|| AppError::unauthorized("EXPIRED", ""))?;
    Ok(user.0)
}
