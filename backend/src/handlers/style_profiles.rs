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
pub struct SaveStyleProfileRequest {
    pub task_id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct StyleProfileSummary {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub source_file: Option<String>,
    pub total_chars: Option<i64>,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct StyleProfileDetail {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub source_file: Option<String>,
    pub total_chars: Option<i64>,
    pub created_at: String,
    pub vocabulary: Option<serde_json::Value>,
    pub sentence: Option<serde_json::Value>,
    pub rhetoric: Option<serde_json::Value>,
    pub narrative: Option<serde_json::Value>,
    pub emotion: Option<serde_json::Value>,
    pub pacing: Option<serde_json::Value>,
    pub dialogue: Option<serde_json::Value>,
    pub description_data: Option<serde_json::Value>,
    pub style_vector: Option<serde_json::Value>,
    pub example_passages: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

/// 保存风格档案
pub async fn save_style_profile(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<SaveStyleProfileRequest>,
) -> Result<Json<ApiSuccess<StyleProfileSummary>>, AppError> {
    let user_id = get_user_id_from_token(&state.db, &headers).await?;

    // 验证名称长度
    if req.name.len() < 2 || req.name.len() > 50 {
        return Err(AppError::bad_request(
            "INVALID_NAME_LENGTH",
            "风格名称长度必须在 2-50 字符之间。",
        ));
    }

    // 验证名称不重复
    let exists: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM style_profiles WHERE user_id = ? AND name = ?"
    )
    .bind(&user_id)
    .bind(&req.name)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to check name uniqueness: {}", e);
        AppError::internal("检查名称唯一性失败")
    })?;

    if exists.0 > 0 {
        return Err(AppError::bad_request(
            "DUPLICATE_NAME",
            "该风格名称已存在。",
        ));
    }

    // 获取分析任务数据
    let task: (String, String, Option<String>, Option<String>, Option<String>, 
               Option<String>, Option<String>, Option<String>, Option<String>, 
               Option<String>, Option<String>, Option<String>) = sqlx::query_as(
        r#"
        SELECT id, source_file_path, vocabulary_json, sentence_json, rhetoric_json,
               narrative_json, emotion_json, pacing_json, dialogue_json, 
               description_json, style_vector_json, status
        FROM style_analysis_tasks 
        WHERE id = ? AND user_id = ? AND status = 'completed'
        "#
    )
    .bind(&req.task_id)
    .bind(&user_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch task: {}", e);
        AppError::internal("获取分析任务失败")
    })?
    .ok_or_else(|| AppError::not_found("TASK_NOT_FOUND", "分析任务不存在或未完成"))?;

    let profile_id = uuid::Uuid::new_v4().to_string();

    // 保存风格档案
    sqlx::query(
        r#"
        INSERT INTO style_profiles (
            id, user_id, name, description, source_file_path, total_chars,
            vocabulary_json, sentence_json, rhetoric_json, narrative_json,
            emotion_json, pacing_json, dialogue_json, description_json,
            style_vector_json, created_at, updated_at
        ) VALUES (?, ?, ?, ?, ?, 0, ?, ?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        "#
    )
    .bind(&profile_id)
    .bind(&user_id)
    .bind(&req.name)
    .bind(&req.description)
    .bind(&task.1) // source_file_path
    .bind(&task.2) // vocabulary_json
    .bind(&task.3) // sentence_json
    .bind(&task.4) // rhetoric_json
    .bind(&task.5) // narrative_json
    .bind(&task.6) // emotion_json
    .bind(&task.7) // pacing_json
    .bind(&task.8) // dialogue_json
    .bind(&task.9) // description_json
    .bind(&task.10) // style_vector_json
    .execute(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to save style profile: {}", e);
        AppError::internal("保存风格档案失败")
    })?;

    // 更新分析任务的 result_profile_id
    sqlx::query(
        "UPDATE style_analysis_tasks SET result_profile_id = ? WHERE id = ?"
    )
    .bind(&profile_id)
    .bind(&req.task_id)
    .execute(&state.db)
    .await
    .ok();

    Ok(Json(ApiSuccess {
        success: true,
        data: StyleProfileSummary {
            id: profile_id,
            name: req.name,
            description: req.description,
            source_file: Some(task.1),
            total_chars: None,
            created_at: chrono::Utc::now().to_rfc3339(),
        },
    }))
}

/// 获取风格档案列表
pub async fn list_style_profiles(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(params): Query<ListParams>,
) -> Result<Json<ApiSuccess<Vec<StyleProfileSummary>>>, AppError> {
    let user_id = get_user_id_from_token(&state.db, &headers).await?;

    let sort_by = params.sort_by.unwrap_or_else(|| "created_at".to_string());
    let sort_order = params.sort_order.unwrap_or_else(|| "desc".to_string());

    let order_clause = match sort_by.as_str() {
        "name" => if sort_order == "asc" { "name ASC" } else { "name DESC" },
        "created_at" => if sort_order == "asc" { "created_at ASC" } else { "created_at DESC" },
        _ => "created_at DESC",
    };

    let query = format!(
        "SELECT id, name, description, source_file_path, total_chars, created_at 
         FROM style_profiles WHERE user_id = ? ORDER BY {}",
        order_clause
    );

    let profiles: Vec<(String, String, Option<String>, Option<String>, Option<i64>, String)> = 
        sqlx::query_as(&query)
        .bind(&user_id)
        .fetch_all(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch style profiles: {}", e);
            AppError::internal("获取风格档案列表失败")
        })?;

    let summaries: Vec<StyleProfileSummary> = profiles.into_iter().map(|p| {
        StyleProfileSummary {
            id: p.0,
            name: p.1,
            description: p.2,
            source_file: p.3,
            total_chars: p.4,
            created_at: p.5,
        }
    }).collect();

    Ok(Json(ApiSuccess {
        success: true,
        data: summaries,
    }))
}

/// 获取风格档案详情
pub async fn get_style_profile(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<ApiSuccess<StyleProfileDetail>>, AppError> {
    let user_id = get_user_id_from_token(&state.db, &headers).await?;

    let profile: (String, String, Option<String>, Option<String>, Option<i64>, String,
                  Option<String>, Option<String>, Option<String>, Option<String>,
                  Option<String>, Option<String>, Option<String>, Option<String>,
                  Option<String>, Option<String>) = sqlx::query_as(
        r#"
        SELECT id, name, description, source_file_path, total_chars, created_at,
               vocabulary_json, sentence_json, rhetoric_json, narrative_json,
               emotion_json, pacing_json, dialogue_json, description_json,
               style_vector_json, example_passages_json
        FROM style_profiles 
        WHERE id = ? AND user_id = ?
        "#
    )
    .bind(&id)
    .bind(&user_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch style profile: {}", e);
        AppError::internal("获取风格档案失败")
    })?
    .ok_or_else(|| AppError::not_found("PROFILE_NOT_FOUND", "风格档案不存在"))?;

    let parse_json = |s: Option<String>| -> Option<serde_json::Value> {
        s.and_then(|v| serde_json::from_str(&v).ok())
    };

    Ok(Json(ApiSuccess {
        success: true,
        data: StyleProfileDetail {
            id: profile.0,
            name: profile.1,
            description: profile.2,
            source_file: profile.3,
            total_chars: profile.4,
            created_at: profile.5,
            vocabulary: parse_json(profile.6),
            sentence: parse_json(profile.7),
            rhetoric: parse_json(profile.8),
            narrative: parse_json(profile.9),
            emotion: parse_json(profile.10),
            pacing: parse_json(profile.11),
            dialogue: parse_json(profile.12),
            description_data: parse_json(profile.13),
            style_vector: parse_json(profile.14),
            example_passages: parse_json(profile.15),
        },
    }))
}

/// 删除风格档案
pub async fn delete_style_profile(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<ApiSuccess<serde_json::Value>>, AppError> {
    let user_id = get_user_id_from_token(&state.db, &headers).await?;

    // 验证风格档案存在且属于当前用户
    let exists: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM style_profiles WHERE id = ? AND user_id = ?"
    )
    .bind(&id)
    .bind(&user_id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to check profile existence: {}", e);
        AppError::internal("检查风格档案失败")
    })?;

    if exists.0 == 0 {
        return Err(AppError::not_found("PROFILE_NOT_FOUND", "风格档案不存在"));
    }

    // 删除风格档案
    sqlx::query("DELETE FROM style_profiles WHERE id = ? AND user_id = ?")
        .bind(&id)
        .bind(&user_id)
        .execute(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to delete style profile: {}", e);
            AppError::internal("删除风格档案失败")
        })?;

    Ok(Json(ApiSuccess {
        success: true,
        data: serde_json::json!({"message": "风格档案已删除"}),
    }))
}

/// 导出风格档案
pub async fn export_style_profile(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user_id = get_user_id_from_token(&state.db, &headers).await?;

    let profile: (String, String, Option<String>, Option<i64>,
                  Option<String>, Option<String>, Option<String>, Option<String>,
                  Option<String>, Option<String>, Option<String>, Option<String>,
                  Option<String>, Option<String>) = sqlx::query_as(
        r#"
        SELECT name, description, source_file_path, total_chars,
               vocabulary_json, sentence_json, rhetoric_json, narrative_json,
               emotion_json, pacing_json, dialogue_json, description_json,
               style_vector_json, example_passages_json
        FROM style_profiles 
        WHERE id = ? AND user_id = ?
        "#
    )
    .bind(&id)
    .bind(&user_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch profile for export: {}", e);
        AppError::internal("获取风格档案失败")
    })?
    .ok_or_else(|| AppError::not_found("PROFILE_NOT_FOUND", "风格档案不存在"))?;

    let parse_json = |s: Option<String>| -> serde_json::Value {
        s.and_then(|v| serde_json::from_str(&v).ok()).unwrap_or(serde_json::Value::Null)
    };

    Ok(Json(serde_json::json!({
        "format": "noval-style-profile-v1",
        "name": profile.0,
        "description": profile.1,
        "source_file": profile.2,
        "total_chars": profile.3,
        "vocabulary": parse_json(profile.4),
        "sentence": parse_json(profile.5),
        "rhetoric": parse_json(profile.6),
        "narrative": parse_json(profile.7),
        "emotion": parse_json(profile.8),
        "pacing": parse_json(profile.9),
        "dialogue": parse_json(profile.10),
        "description_data": parse_json(profile.11),
        "style_vector": parse_json(profile.12),
        "example_passages": parse_json(profile.13),
    })))
}

/// 导入风格档案
#[derive(Debug, Deserialize)]
pub struct ImportRequest {
    pub name: String,
    pub description: Option<String>,
    pub vocabulary: Option<serde_json::Value>,
    pub sentence: Option<serde_json::Value>,
    pub rhetoric: Option<serde_json::Value>,
    pub narrative: Option<serde_json::Value>,
    pub emotion: Option<serde_json::Value>,
    pub pacing: Option<serde_json::Value>,
    pub dialogue: Option<serde_json::Value>,
    pub description_data: Option<serde_json::Value>,
    pub style_vector: Option<serde_json::Value>,
    pub example_passages: Option<serde_json::Value>,
}

pub async fn import_style_profile(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<ImportRequest>,
) -> Result<Json<ApiSuccess<StyleProfileSummary>>, AppError> {
    let user_id = get_user_id_from_token(&state.db, &headers).await?;

    // 验证名称长度
    if req.name.len() < 2 || req.name.len() > 50 {
        return Err(AppError::bad_request(
            "INVALID_NAME_LENGTH",
            "风格名称长度必须在 2-50 字符之间。",
        ));
    }

    // 验证名称不重复
    let exists: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM style_profiles WHERE user_id = ? AND name = ?"
    )
    .bind(&user_id)
    .bind(&req.name)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to check name uniqueness: {}", e);
        AppError::internal("检查名称唯一性失败")
    })?;

    if exists.0 > 0 {
        return Err(AppError::bad_request(
            "DUPLICATE_NAME",
            "该风格名称已存在。",
        ));
    }

    let profile_id = uuid::Uuid::new_v4().to_string();
    let to_json = |v: Option<serde_json::Value>| v.map(|v| v.to_string());

    sqlx::query(
        r#"
        INSERT INTO style_profiles (
            id, user_id, name, description,
            vocabulary_json, sentence_json, rhetoric_json, narrative_json,
            emotion_json, pacing_json, dialogue_json, description_json,
            style_vector_json, example_passages_json,
            created_at, updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        "#
    )
    .bind(&profile_id)
    .bind(&user_id)
    .bind(&req.name)
    .bind(&req.description)
    .bind(to_json(req.vocabulary))
    .bind(to_json(req.sentence))
    .bind(to_json(req.rhetoric))
    .bind(to_json(req.narrative))
    .bind(to_json(req.emotion))
    .bind(to_json(req.pacing))
    .bind(to_json(req.dialogue))
    .bind(to_json(req.description_data))
    .bind(to_json(req.style_vector))
    .bind(to_json(req.example_passages))
    .execute(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to import style profile: {}", e);
        AppError::internal("导入风格档案失败")
    })?;

    Ok(Json(ApiSuccess {
        success: true,
        data: StyleProfileSummary {
            id: profile_id,
            name: req.name,
            description: req.description,
            source_file: None,
            total_chars: None,
            created_at: chrono::Utc::now().to_rfc3339(),
        },
    }))
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