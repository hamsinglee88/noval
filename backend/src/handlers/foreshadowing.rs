use axum::{
    Json,
    extract::{State, Path},
    http::{HeaderMap, header},
};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    errors::AppError,
    services::foreshadowing::{detect_foreshadows, Foreshadow, ForeshadowStatus},
};

#[derive(Debug, Serialize)]
pub struct ApiSuccess<T> { pub success: bool, pub data: T }

#[derive(Debug, Deserialize)]
pub struct DetectRequest {
    pub chapter_id: String,
}

#[derive(Debug, Serialize)]
pub struct DetectResponse {
    pub foreshadows: Vec<Foreshadow>,
    pub count: usize,
}

#[derive(Debug, Serialize)]
pub struct ForeshadowSummary {
    pub id: String,
    pub content: String,
    pub chapter_id: String,
    pub foreshadow_type: String,
    pub status: String,
    pub confidence_score: f64,
}

/// 检测章节伏笔
pub async fn detect_chapter_foreshadows(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((novel_id, chapter_id)): Path<(String, String)>,
) -> Result<Json<ApiSuccess<DetectResponse>>, AppError> {
    let _user_id = get_user_id_from_token(&state.db, &headers).await?;

    // 获取章节内容
    let chapter: (String,) = sqlx::query_as(
        "SELECT content FROM chapters WHERE id = ? AND novel_id = ?"
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

    // 检测伏笔
    let result = detect_foreshadows(&chapter.0, &chapter_id);

    // 保存检测到的伏笔到数据库
    for foreshadow in &result.foreshadows {
        sqlx::query(
            r#"
            INSERT INTO foreshadows (id, novel_id, chapter_id, content, foreshadow_type, status, confidence_score, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&foreshadow.id)
        .bind(&novel_id)
        .bind(&chapter_id)
        .bind(&foreshadow.content)
        .bind(format!("{:?}", foreshadow.foreshadow_type))
        .bind(format!("{:?}", foreshadow.status))
        .bind(foreshadow.confidence_score)
        .bind(&foreshadow.created_at)
        .execute(&state.db)
        .await
        .ok();
    }

    Ok(Json(ApiSuccess {
        success: true,
        data: DetectResponse {
            foreshadows: result.foreshadows,
            count: result.count,
        },
    }))
}

/// 获取小说所有伏笔
pub async fn list_foreshadows(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(novel_id): Path<String>,
) -> Result<Json<ApiSuccess<Vec<ForeshadowSummary>>>, AppError> {
    let _user_id = get_user_id_from_token(&state.db, &headers).await?;

    let foreshadows: Vec<(String, String, String, String, String, f64)> = sqlx::query_as(
        "SELECT id, content, chapter_id, foreshadow_type, status, confidence_score FROM foreshadows WHERE novel_id = ? ORDER BY created_at DESC"
    )
    .bind(&novel_id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch foreshadows: {}", e);
        AppError::internal("获取伏笔列表失败")
    })?;

    let summaries: Vec<ForeshadowSummary> = foreshadows.into_iter().map(|f| {
        ForeshadowSummary {
            id: f.0,
            content: f.1,
            chapter_id: f.2,
            foreshadow_type: f.3,
            status: f.4,
            confidence_score: f.5,
        }
    }).collect();

    Ok(Json(ApiSuccess {
        success: true,
        data: summaries,
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
    let value = headers.get(header::AUTHORIZATION)
        .ok_or_else(|| AppError::unauthorized("MISSING_SESSION", "缺少登录凭证"))?;
    let value = value.to_str()
        .map_err(|_| AppError::unauthorized("INVALID_SESSION", "登录凭证格式无效"))?;
    value.strip_prefix("Bearer ")
        .map(ToOwned::to_owned)
        .ok_or_else(|| AppError::unauthorized("INVALID_SESSION", "登录凭证格式无效"))
}