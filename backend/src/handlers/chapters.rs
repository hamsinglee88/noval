use axum::{
    Json,
    extract::{State, Path, Query},
    http::{HeaderMap, header},
};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    errors::AppError,
};

#[derive(Debug, Serialize)]
pub struct ApiSuccess<T> {
    pub success: bool,
    pub data: T,
}

#[derive(Debug, Deserialize)]
pub struct CreateChapterRequest {
    pub title: String,
    pub content: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateChapterRequest {
    pub title: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ChapterSummary {
    pub id: String,
    pub novel_id: String,
    pub title: String,
    pub chapter_number: i64,
    pub word_count: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct ChapterDetail {
    pub id: String,
    pub novel_id: String,
    pub title: String,
    pub chapter_number: i64,
    pub content: String,
    pub word_count: i64,
    pub created_at: String,
    pub updated_at: String,
}

/// 创建章节
pub async fn create_chapter(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(novel_id): Path<String>,
    Json(req): Json<CreateChapterRequest>,
) -> Result<Json<ApiSuccess<ChapterSummary>>, AppError> {
    let user_id = get_user_id_from_token(&state.db, &headers).await?;

    // 验证项目存在
    let exists: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM novels WHERE id = ? AND user_id = ?"
    )
    .bind(&novel_id)
    .bind(&user_id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to check novel: {}", e);
        AppError::internal("检查项目失败")
    })?;

    if exists.0 == 0 {
        return Err(AppError::not_found("NOVEL_NOT_FOUND", "项目不存在"));
    }

    // 获取当前最大章节号
    let max_number: (Option<i64>,) = sqlx::query_as(
        "SELECT MAX(chapter_number) FROM chapters WHERE novel_id = ?"
    )
    .bind(&novel_id)
    .fetch_one(&state.db)
    .await
    .unwrap_or((None,));

    let chapter_number = max_number.0.unwrap_or(0) + 1;
    let chapter_id = uuid::Uuid::new_v4().to_string();
    let content = req.content.unwrap_or_default();
    let word_count = content.chars().count() as i64;

    sqlx::query(
        r#"
        INSERT INTO chapters (id, novel_id, title, chapter_number, content, word_count, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        "#
    )
    .bind(&chapter_id)
    .bind(&novel_id)
    .bind(&req.title)
    .bind(chapter_number)
    .bind(&content)
    .bind(word_count)
    .execute(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create chapter: {}", e);
        AppError::internal("创建章节失败")
    })?;

    Ok(Json(ApiSuccess {
        success: true,
        data: ChapterSummary {
            id: chapter_id,
            novel_id,
            title: req.title,
            chapter_number,
            word_count,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        },
    }))
}

/// 获取章节列表
pub async fn list_chapters(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(novel_id): Path<String>,
) -> Result<Json<ApiSuccess<Vec<ChapterSummary>>>, AppError> {
    let user_id = get_user_id_from_token(&state.db, &headers).await?;

    // 验证项目存在
    let exists: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM novels WHERE id = ? AND user_id = ?"
    )
    .bind(&novel_id)
    .bind(&user_id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to check novel: {}", e);
        AppError::internal("检查项目失败")
    })?;

    if exists.0 == 0 {
        return Err(AppError::not_found("NOVEL_NOT_FOUND", "项目不存在"));
    }

    let chapters: Vec<(String, String, String, i64, i64, String, String)> = sqlx::query_as(
        "SELECT id, novel_id, title, chapter_number, word_count, created_at, updated_at FROM chapters WHERE novel_id = ? ORDER BY chapter_number"
    )
    .bind(&novel_id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch chapters: {}", e);
        AppError::internal("获取章节列表失败")
    })?;

    let summaries: Vec<ChapterSummary> = chapters.into_iter().map(|c| {
        ChapterSummary {
            id: c.0,
            novel_id: c.1,
            title: c.2,
            chapter_number: c.3,
            word_count: c.4,
            created_at: c.5,
            updated_at: c.6,
        }
    }).collect();

    Ok(Json(ApiSuccess {
        success: true,
        data: summaries,
    }))
}

/// 获取章节详情
pub async fn get_chapter(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((novel_id, chapter_id)): Path<(String, String)>,
) -> Result<Json<ApiSuccess<ChapterDetail>>, AppError> {
    let user_id = get_user_id_from_token(&state.db, &headers).await?;

    let chapter: (String, String, String, i64, String, i64, String, String) = sqlx::query_as(
        "SELECT id, novel_id, title, chapter_number, content, word_count, created_at, updated_at FROM chapters WHERE id = ? AND novel_id = ?"
    )
    .bind(&chapter_id)
    .bind(&novel_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch chapter: {}", e);
        AppError::internal("获取章节失败")
    })?
    .ok_or_else(|| AppError::not_found("CHAPTER_NOT_FOUND", "章节不存在"))?;

    Ok(Json(ApiSuccess {
        success: true,
        data: ChapterDetail {
            id: chapter.0,
            novel_id: chapter.1,
            title: chapter.2,
            chapter_number: chapter.3,
            content: chapter.4,
            word_count: chapter.5,
            created_at: chapter.6,
            updated_at: chapter.7,
        },
    }))
}

/// 更新章节
pub async fn update_chapter(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((novel_id, chapter_id)): Path<(String, String)>,
    Json(req): Json<UpdateChapterRequest>,
) -> Result<Json<ApiSuccess<ChapterSummary>>, AppError> {
    let user_id = get_user_id_from_token(&state.db, &headers).await?;

    // 验证章节存在
    let exists: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM chapters c JOIN novels n ON c.novel_id = n.id WHERE c.id = ? AND c.novel_id = ? AND n.user_id = ?"
    )
    .bind(&chapter_id)
    .bind(&novel_id)
    .bind(&user_id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to check chapter: {}", e);
        AppError::internal("检查章节失败")
    })?;

    if exists.0 == 0 {
        return Err(AppError::not_found("CHAPTER_NOT_FOUND", "章节不存在"));
    }

    if let Some(title) = req.title {
        sqlx::query("UPDATE chapters SET title = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(&title)
            .bind(&chapter_id)
            .execute(&state.db)
            .await
            .ok();
    }

    if let Some(content) = req.content {
        let word_count = content.chars().count() as i64;
        sqlx::query("UPDATE chapters SET content = ?, word_count = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(&content)
            .bind(word_count)
            .bind(&chapter_id)
            .execute(&state.db)
            .await
            .ok();
    }

    // 返回更新后的章节
    let chapter: (String, String, String, i64, i64, String, String) = sqlx::query_as(
        "SELECT id, novel_id, title, chapter_number, word_count, created_at, updated_at FROM chapters WHERE id = ?"
    )
    .bind(&chapter_id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch updated chapter: {}", e);
        AppError::internal("获取更新后的章节失败")
    })?;

    Ok(Json(ApiSuccess {
        success: true,
        data: ChapterSummary {
            id: chapter.0,
            novel_id: chapter.1,
            title: chapter.2,
            chapter_number: chapter.3,
            word_count: chapter.4,
            created_at: chapter.5,
            updated_at: chapter.6,
        },
    }))
}

/// 删除章节
pub async fn delete_chapter(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((novel_id, chapter_id)): Path<(String, String)>,
) -> Result<Json<ApiSuccess<serde_json::Value>>, AppError> {
    let user_id = get_user_id_from_token(&state.db, &headers).await?;

    sqlx::query(
        "DELETE FROM chapters WHERE id = ? AND novel_id = ? AND novel_id IN (SELECT id FROM novels WHERE user_id = ?)"
    )
    .bind(&chapter_id)
    .bind(&novel_id)
    .bind(&user_id)
    .execute(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to delete chapter: {}", e);
        AppError::internal("删除章节失败")
    })?;

    Ok(Json(ApiSuccess {
        success: true,
        data: serde_json::json!({"message": "章节已删除"}),
    }))
}

async fn get_user_id_from_token(db: &sqlx::SqlitePool, headers: &HeaderMap) -> Result<String, AppError> {
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

fn bearer_token(headers: &HeaderMap) -> Result<String, AppError> {
    let value = headers.get(header::AUTHORIZATION)
        .ok_or_else(|| AppError::unauthorized("MISSING_SESSION", "缺少登录凭证"))?;
    let value = value.to_str()
        .map_err(|_| AppError::unauthorized("INVALID_SESSION", "登录凭证格式无效"))?;
    value.strip_prefix("Bearer ")
        .map(ToOwned::to_owned)
        .ok_or_else(|| AppError::unauthorized("INVALID_SESSION", "登录凭证格式无效"))
}