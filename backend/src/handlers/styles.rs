use axum::{
    Json,
    extract::{Multipart, State},
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
}

impl From<StyleAnalysisTask> for TaskData {
    fn from(task: StyleAnalysisTask) -> Self {
        Self {
            task_id: task.id,
            status: task.status,
            progress: task.progress,
            source_file_path: task.source_file_path,
            status_message: task.status_message,
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

    let service = StyleUploadService::new(state.db);
    let task = service.upload_and_create_task(multipart, &user_id).await?;

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

/// 从 JWT token 获取用户 ID
async fn get_user_id_from_token(
    db: &sqlx::SqlitePool,
    headers: &HeaderMap,
) -> Result<String, AppError> {
    use jsonwebtoken::{decode, DecodingKey, Validation};

    #[derive(Debug, serde::Deserialize)]
    struct JwtClaims {
        sub: String,
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
