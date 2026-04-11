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
    
    // 计算一致性评分（简化版）
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
    
    Ok(Json(ApiSuccess {
        success: true,
        data: ScanResult {
            total_chapters: chapter_count.0 as usize,
            foreshadow_count: foreshadow_count.0 as usize,
            overdue_count: overdue_count.0 as usize,
            consistency_score,
            issues,
        },
    }))
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