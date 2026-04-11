use axum::http::{HeaderMap, header};
use crate::errors::AppError;

/// 从请求头中提取并验证用户 ID
pub async fn get_user_id_from_token(
    db: &sqlx::SqlitePool,
    headers: &HeaderMap,
) -> Result<String, AppError> {
    let token = bearer_token(headers)?;
    
    let user: (String,) = sqlx::query_as(
        "SELECT id FROM users WHERE session_token = ? AND session_expires_at > datetime('now')"
    )
    .bind(&token)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| AppError::unauthorized("SESSION_EXPIRED", "登录状态已失效"))?;
    
    Ok(user.0)
}

/// 从 Authorization 头中提取 Bearer token
pub fn bearer_token(headers: &HeaderMap) -> Result<String, AppError> {
    let value = headers
        .get(header::AUTHORIZATION)
        .ok_or_else(|| AppError::unauthorized("MISSING_SESSION", "缺少登录凭证"))?;
    
    let value = value
        .to_str()
        .map_err(|_| AppError::unauthorized("INVALID_SESSION", "登录凭证格式无效"))?;
    
    value
        .strip_prefix("Bearer ")
        .map(ToOwned::to_owned)
        .ok_or_else(|| AppError::unauthorized("INVALID_SESSION", "登录凭证格式无效"))
}

/// 验证用户是否拥有指定小说
pub async fn verify_novel_ownership(
    db: &sqlx::SqlitePool,
    user_id: &str,
    novel_id: &str,
) -> Result<(), AppError> {
    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM novels WHERE id = ? AND user_id = ?"
    )
    .bind(novel_id)
    .bind(user_id)
    .fetch_one(db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to verify novel ownership: {}", e);
        AppError::internal("验证权限失败")
    })?;
    
    if count.0 == 0 {
        return Err(AppError::not_found("NOVEL_NOT_FOUND", "小说不存在或无权访问"));
    }
    
    Ok(())
}

/// 验证用户是否拥有指定章节
pub async fn verify_chapter_ownership(
    db: &sqlx::SqlitePool,
    user_id: &str,
    novel_id: &str,
    chapter_id: &str,
) -> Result<(), AppError> {
    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM chapters c JOIN novels n ON c.novel_id = n.id WHERE c.id = ? AND c.novel_id = ? AND n.user_id = ?"
    )
    .bind(chapter_id)
    .bind(novel_id)
    .bind(user_id)
    .fetch_one(db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to verify chapter ownership: {}", e);
        AppError::internal("验证权限失败")
    })?;
    
    if count.0 == 0 {
        return Err(AppError::not_found("CHAPTER_NOT_FOUND", "章节不存在或无权访问"));
    }
    
    Ok(())
}

/// 通用 API 响应结构
#[derive(Debug, serde::Serialize)]
pub struct ApiSuccess<T> {
    pub success: bool,
    pub data: T,
}

impl<T> ApiSuccess<T> {
    pub fn ok(data: T) -> Self {
        Self { success: true, data }
    }
}