use axum::{
    Json,
    extract::{Multipart, State, Path},
    http::{HeaderMap, header},
};
use serde::Serialize;

use crate::{
    app_state::AppState,
    errors::AppError,
    models::style::StyleAnalysisTask,
    services::style_upload_service::StyleUploadService,
};

#[derive(Debug, Serialize)]
pub struct ApiSuccess<T> {
    pub success: bool,
    pub data: T,
}

#[derive(Debug, Serialize)]
pub struct TaskData {
    pub task_id: String,
    pub status: String,
    pub progress: f64,
    pub source_file_path: String,
    pub status_message: Option<String>,
    // Story 1.3: 词汇层和句式层分析结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentence: Option<serde_json::Value>,
    // Story 1.4: 修辞层和叙事层分析结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rhetoric: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub narrative: Option<serde_json::Value>,
    // Story 1.5: 情感层和节奏层分析结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emotion: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pacing: Option<serde_json::Value>,
}

impl From<StyleAnalysisTask> for TaskData {
    fn from(task: StyleAnalysisTask) -> Self {
        let vocabulary = task.vocabulary_json.and_then(|v| serde_json::from_str(&v).ok());
        let sentence = task.sentence_json.and_then(|v| serde_json::from_str(&v).ok());
        let rhetoric = task.rhetoric_json.and_then(|v| serde_json::from_str(&v).ok());
        let narrative = task.narrative_json.and_then(|v| serde_json::from_str(&v).ok());
        let emotion = task.emotion_json.and_then(|v| serde_json::from_str(&v).ok());
        let pacing = task.pacing_json.and_then(|v| serde_json::from_str(&v).ok());

        Self {
            task_id: task.id,
            status: task.status,
            progress: task.progress,
            source_file_path: task.source_file_path,
            status_message: task.status_message,
            vocabulary,
            sentence,
            rhetoric,
            narrative,
            emotion,
            pacing,
        }
    }
}

/// 上传小说并进行风格分析
/// POST /api/styles/analyze
pub async fn analyze(
    State(state): State<AppState>,
    headers: HeaderMap,
    multipart: Multipart,
) -> Result<Json<ApiSuccess<TaskData>>, AppError> {
    // 获取用户 ID（从 session token）
    let user_id = get_user_id_from_token(&state.db, &headers).await?;

    let upload_service = StyleUploadService::new(state.db.clone());
    let task = upload_service.upload_and_create_task(multipart, &user_id).await?;

    // 在后台启动分析任务
    let db = state.db.clone();
    let task_id = task.id.clone();
    let file_path = task.source_file_path.clone();

    tokio::spawn(async move {
        use crate::services::style_analysis::StyleAnalyzer;

        let analyzer = StyleAnalyzer::new(db);
        match analyzer.analyze_all_layers(&task_id, &file_path).await {
            Ok(_) => {
                tracing::info!("Style analysis completed for task: {}", task_id);
            }
            Err(e) => {
                tracing::error!("Style analysis failed for task {}: {}", task_id, e);
            }
        }
    });

    Ok(Json(ApiSuccess {
        success: true,
        data: task.into(),
    }))
}

/// 获取分析任务状态
/// GET /api/styles/analyze/:task_id
pub async fn get_task_status(
    State(state): State<AppState>,
    headers: HeaderMap,
    axum::extract::Path(task_id): axum::extract::Path<String>,
) -> Result<Json<ApiSuccess<TaskData>>, AppError> {
    let user_id = get_user_id_from_token(&state.db, &headers).await?;

    let service = StyleUploadService::new(state.db);
    let task = service.get_task(&task_id, &user_id).await?;

    Ok(Json(ApiSuccess {
        success: true,
        data: task.into(),
    }))
}

/// 取消上传/分析任务
/// POST /api/styles/analyze/:task_id/cancel
pub async fn cancel_task(
    State(state): State<AppState>,
    headers: HeaderMap,
    axum::extract::Path(task_id): axum::extract::Path<String>,
) -> Result<Json<ApiSuccess<serde_json::Value>>, AppError> {
    let user_id = get_user_id_from_token(&state.db, &headers).await?;

    let service = StyleUploadService::new(state.db);

    // 验证任务属于当前用户
    let task = service.get_task(&task_id, &user_id).await?;

    // 只允许取消 pending 或 processing 状态的任务
    if task.status == "completed" || task.status == "failed" {
        return Err(AppError::bad_request(
            "INVALID_TASK_STATUS",
            "当前任务状态无法取消。",
        ));
    }

    // 标记为失败
    service.mark_task_failed(&task_id, "用户取消上传。").await?;

    // 清理上传的文件（可选，保留用于恢复）
    // tokio::fs::remove_file(&task.source_file_path).await.ok();

    Ok(Json(ApiSuccess {
        success: true,
        data: serde_json::json!({"message": "已取消分析任务"}),
    }))
}

/// 获取词汇层分析结果
/// GET /api/styles/analyze/:task_id/vocabulary
pub async fn get_vocabulary_result(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(task_id): Path<String>,
) -> Result<Json<ApiSuccess<serde_json::Value>>, AppError> {
    let user_id = get_user_id_from_token(&state.db, &headers).await?;

    let service = StyleUploadService::new(state.db);
    let task = service.get_task(&task_id, &user_id).await?;

    let vocabulary = task.vocabulary_json
        .ok_or_else(|| AppError::not_found("VOCABULARY_NOT_READY", "词汇层分析尚未完成。"))?;

    let vocab_value: serde_json::Value = serde_json::from_str(&vocabulary)
        .map_err(|e| AppError::serialization_error(format!("解析词汇分析结果失败：{}", e)))?;

    Ok(Json(ApiSuccess {
        success: true,
        data: vocab_value,
    }))
}

/// 获取句式层分析结果
/// GET /api/styles/analyze/:task_id/sentence
pub async fn get_sentence_result(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(task_id): Path<String>,
) -> Result<Json<ApiSuccess<serde_json::Value>>, AppError> {
    let user_id = get_user_id_from_token(&state.db, &headers).await?;

    let service = StyleUploadService::new(state.db);
    let task = service.get_task(&task_id, &user_id).await?;

    let sentence = task.sentence_json
        .ok_or_else(|| AppError::not_found("SENTENCE_NOT_READY", "句式层分析尚未完成。"))?;

    let sentence_value: serde_json::Value = serde_json::from_str(&sentence)
        .map_err(|e| AppError::serialization_error(format!("解析句式分析结果失败：{}", e)))?;

    Ok(Json(ApiSuccess {
        success: true,
        data: sentence_value,
    }))
}

/// 获取修辞层分析结果
/// GET /api/styles/analyze/:task_id/rhetoric
pub async fn get_rhetoric_result(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(task_id): Path<String>,
) -> Result<Json<ApiSuccess<serde_json::Value>>, AppError> {
    let user_id = get_user_id_from_token(&state.db, &headers).await?;

    let service = StyleUploadService::new(state.db);
    let task = service.get_task(&task_id, &user_id).await?;

    let rhetoric = task.rhetoric_json
        .ok_or_else(|| AppError::not_found("RHETORIC_NOT_READY", "修辞层分析尚未完成。"))?;

    let rhetoric_value: serde_json::Value = serde_json::from_str(&rhetoric)
        .map_err(|e| AppError::serialization_error(format!("解析修辞分析结果失败：{}", e)))?;

    Ok(Json(ApiSuccess {
        success: true,
        data: rhetoric_value,
    }))
}

/// 获取叙事层分析结果
/// GET /api/styles/analyze/:task_id/narrative
pub async fn get_narrative_result(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(task_id): Path<String>,
) -> Result<Json<ApiSuccess<serde_json::Value>>, AppError> {
    let user_id = get_user_id_from_token(&state.db, &headers).await?;

    let service = StyleUploadService::new(state.db);
    let task = service.get_task(&task_id, &user_id).await?;

    let narrative = task.narrative_json
        .ok_or_else(|| AppError::not_found("NARRATIVE_NOT_READY", "叙事层分析尚未完成。"))?;

    let narrative_value: serde_json::Value = serde_json::from_str(&narrative)
        .map_err(|e| AppError::serialization_error(format!("解析叙事分析结果失败：{}", e)))?;

    Ok(Json(ApiSuccess {
        success: true,
        data: narrative_value,
    }))
}

/// 获取情感层分析结果
/// GET /api/styles/analyze/:task_id/emotion
pub async fn get_emotion_result(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(task_id): Path<String>,
) -> Result<Json<ApiSuccess<serde_json::Value>>, AppError> {
    let user_id = get_user_id_from_token(&state.db, &headers).await?;

    let service = StyleUploadService::new(state.db);
    let task = service.get_task(&task_id, &user_id).await?;

    let emotion = task.emotion_json
        .ok_or_else(|| AppError::not_found("EMOTION_NOT_READY", "情感层分析尚未完成。"))?;

    let emotion_value: serde_json::Value = serde_json::from_str(&emotion)
        .map_err(|e| AppError::serialization_error(format!("解析情感分析结果失败：{}", e)))?;

    Ok(Json(ApiSuccess {
        success: true,
        data: emotion_value,
    }))
}

/// 获取节奏层分析结果
/// GET /api/styles/analyze/:task_id/pacing
pub async fn get_pacing_result(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(task_id): Path<String>,
) -> Result<Json<ApiSuccess<serde_json::Value>>, AppError> {
    let user_id = get_user_id_from_token(&state.db, &headers).await?;

    let service = StyleUploadService::new(state.db);
    let task = service.get_task(&task_id, &user_id).await?;

    let pacing = task.pacing_json
        .ok_or_else(|| AppError::not_found("PACING_NOT_READY", "节奏层分析尚未完成。"))?;

    let pacing_value: serde_json::Value = serde_json::from_str(&pacing)
        .map_err(|e| AppError::serialization_error(format!("解析节奏分析结果失败：{}", e)))?;

    Ok(Json(ApiSuccess {
        success: true,
        data: pacing_value,
    }))
}

/// 从 JWT token 获取用户 ID
async fn get_user_id_from_token(
    db: &sqlx::SqlitePool,
    headers: &HeaderMap,
) -> Result<String, AppError> {
    use jsonwebtoken::{decode, DecodingKey, Validation};

    #[derive(Debug, serde::Deserialize)]
    struct JwtClaims {
        #[allow(dead_code)]
        sub: String,
        #[allow(dead_code)]
        exp: usize,
    }

    let token = bearer_token(headers)?;

    // 验证 JWT token
    let secret = std::env::var("NOVAL_JWT_SECRET")
        .unwrap_or_else(|_| "noval-dev-secret-key-change-in-prod".to_string())
        .into_bytes();

    // 首先验证 token 有效性
    let _token_data = decode::<JwtClaims>(
        &token,
        &DecodingKey::from_secret(&secret),
        &Validation::default(),
    ).map_err(|_| AppError::unauthorized("INVALID_TOKEN", "登录凭证无效或已过期。"))?;

    // 验证数据库中是否存在该 session
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
