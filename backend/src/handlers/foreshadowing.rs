use axum::{Json, extract::{State, Path}, http::HeaderMap};
use serde::{Deserialize, Serialize};
use crate::{app_state::AppState, errors::AppError, auth_utils::{get_user_id_from_token, verify_novel_ownership, ApiSuccess}, services::foreshadowing::{detect_foreshadows, Foreshadow, ForeshadowStatus}};

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
    let user_id = get_user_id_from_token(&state.db, &headers).await?;
    verify_novel_ownership(&state.db, &user_id, &novel_id).await?;

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

    // 使用事务批量保存伏笔
    let mut tx = state.db.begin().await.map_err(|e| {
        tracing::error!("Failed to start transaction: {}", e);
        AppError::internal("数据库事务失败")
    })?;

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
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to save foreshadow: {}", e);
            AppError::internal("保存伏笔失败")
        })?;
    }

    tx.commit().await.map_err(|e| {
        tracing::error!("Failed to commit transaction: {}", e);
        AppError::internal("提交事务失败")
    })?;

    Ok(Json(ApiSuccess::ok(DetectResponse {
        foreshadows: result.foreshadows,
        count: result.count,
    })))
}

/// 获取小说所有伏笔（分页）
pub async fn list_foreshadows(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(novel_id): Path<String>,
) -> Result<Json<ApiSuccess<Vec<ForeshadowSummary>>>, AppError> {
    let user_id = get_user_id_from_token(&state.db, &headers).await?;
    verify_novel_ownership(&state.db, &user_id, &novel_id).await?;

    // TODO: 添加分页参数
    let foreshadows: Vec<(String, String, String, String, String, f64)> = sqlx::query_as(
        "SELECT id, content, chapter_id, foreshadow_type, status, confidence_score FROM foreshadows WHERE novel_id = ? ORDER BY created_at DESC LIMIT 100"
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

    Ok(Json(ApiSuccess::ok(summaries)))
}