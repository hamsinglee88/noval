use axum::{
    Json,
    extract::{State, Query},
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
pub struct SimilarityQuery {
    pub style_id: String,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct SimilarStyle {
    pub id: String,
    pub name: String,
    pub similarity: f64,
}

/// 查找相似风格
pub async fn find_similar(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<SimilarityQuery>,
) -> Result<Json<ApiSuccess<Vec<SimilarStyle>>>, AppError> {
    let user_id = get_user_id_from_token(&state.db, &headers).await?;
    let limit = query.limit.unwrap_or(10).min(50);

    // 获取目标风格的向量
    let target: (Option<String>, String) = sqlx::query_as(
        "SELECT style_vector_json, name FROM style_profiles WHERE id = ? AND user_id = ?"
    )
    .bind(&query.style_id)
    .bind(&user_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch target style: {}", e);
        AppError::internal("获取目标风格失败")
    })?
    .ok_or_else(|| AppError::not_found("STYLE_NOT_FOUND", "目标风格不存在"))?;

    let target_vector: Vec<f32> = target.0
        .and_then(|v| serde_json::from_str(&v).ok())
        .unwrap_or_else(|| vec![0.0; 128]);

    // 获取所有其他风格档案
    let profiles: Vec<(String, String, Option<String>)> = sqlx::query_as(
        "SELECT id, name, style_vector_json FROM style_profiles WHERE user_id = ? AND id != ?"
    )
    .bind(&user_id)
    .bind(&query.style_id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch profiles: {}", e);
        AppError::internal("获取风格档案失败")
    })?;

    // 计算相似度
    let mut similarities: Vec<SimilarStyle> = profiles
        .into_iter()
        .filter_map(|(id, name, vector_json)| {
            let vector: Vec<f32> = vector_json
                .and_then(|v| serde_json::from_str(&v).ok())
                .unwrap_or_else(|| vec![0.0; 128]);

            let similarity = cosine_similarity(&target_vector, &vector);
            
            if similarity > 0.0 {
                Some(SimilarStyle {
                    id,
                    name,
                    similarity: (similarity * 100.0).round() / 100.0,
                })
            } else {
                None
            }
        })
        .collect();

    // 按相似度排序
    similarities.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap());
    similarities.truncate(limit as usize);

    Ok(Json(ApiSuccess {
        success: true,
        data: similarities,
    }))
}

/// 计算余弦相似度
fn cosine_similarity(a: &[f32], b: &[f32]) -> f64 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }

    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    (dot / (norm_a * norm_b)) as f64
}

/// 从 JWT token 获取用户 ID
async fn get_user_id_from_token(
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
    .ok_or_else(|| AppError::unauthorized("SESSION_EXPIRED", "登录状态已失效，请重新登录。"))?;

    Ok(user.0)
}

fn bearer_token(headers: &HeaderMap) -> Result<String, AppError> {
    let value = headers
        .get(header::AUTHORIZATION)
        .ok_or_else(|| AppError::unauthorized("MISSING_SESSION", "缺少登录凭证。"))?;
    let value = value
        .to_str()
        .map_err(|_| AppError::unauthorized("INVALID_SESSION", "登录凭证格式无效。"))?;

    value
        .strip_prefix("Bearer ")
        .map(ToOwned::to_owned)
        .ok_or_else(|| AppError::unauthorized("INVALID_SESSION", "登录凭证格式无效。"))
}