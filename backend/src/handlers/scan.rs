use axum::{Json, extract::{State, Path}, http::HeaderMap};
use serde::Serialize;
use crate::{app_state::AppState, errors::AppError, auth_utils::{get_user_id_from_token, verify_novel_ownership, ApiSuccess}};

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
    let user_id = get_user_id_from_token(&state.db, &headers).await?;
    verify_novel_ownership(&state.db, &user_id, &novel_id).await?;
    
    // 获取章节数
    let chapter_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM chapters WHERE novel_id = ?")
        .bind(&novel_id)
        .fetch_one(&state.db)
        .await
        .map_err(|e| { tracing::error!("{}", e); AppError::internal("获取章节数失败") })?;
    
    // 获取伏笔数
    let foreshadow_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM foreshadows WHERE novel_id = ?")
        .bind(&novel_id)
        .fetch_one(&state.db)
        .await
        .map_err(|e| { tracing::error!("{}", e); AppError::internal("获取伏笔数失败") })?;
    
    // 获取逾期伏笔数
    let overdue_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM foreshadows WHERE novel_id = ? AND status = 'Overdue'")
        .bind(&novel_id)
        .fetch_one(&state.db)
        .await
        .map_err(|e| { tracing::error!("{}", e); AppError::internal("获取逾期伏笔数失败") })?;
    
    // 计算一致性评分
    let consistency_score = if foreshadow_count.0 > 0 {
        1.0 - (overdue_count.0 as f64 / foreshadow_count.0 as f64 * 0.5)
    } else {
        1.0
    };
    
    let issues = if overdue_count.0 > 0 {
        vec![format!("有 {} 个伏笔逾期未回收", overdue_count.0)]
    } else {
        vec![]
    };
    
    Ok(Json(ApiSuccess::ok(ScanResult {
        total_chapters: chapter_count.0 as usize,
        foreshadow_count: foreshadow_count.0 as usize,
        overdue_count: overdue_count.0 as usize,
        consistency_score,
        issues,
    })))
}