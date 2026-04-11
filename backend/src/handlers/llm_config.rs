use axum::{Json, extract::{State, Path}, http::HeaderMap};
use serde::{Deserialize, Serialize};
use crate::{app_state::AppState, errors::AppError, auth_utils::{get_user_id_from_token, ApiSuccess}};

#[derive(Debug, Serialize)]
pub struct LLMConfigResponse {
    pub id: String,
    pub provider: String,
    pub model: String,
    pub is_default: bool,
    pub is_active: bool,
    pub max_tokens: i64,
    pub temperature: f64,
}

#[derive(Debug, Deserialize)]
pub struct CreateLLMConfigRequest {
    pub provider: String,
    pub model: String,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    pub is_default: Option<bool>,
    pub max_tokens: Option<i64>,
    pub temperature: Option<f64>,
}

impl CreateLLMConfigRequest {
    pub fn validate(&self) -> Result<(), AppError> {
        if self.provider.is_empty() {
            return Err(AppError::bad_request("INVALID_PROVIDER", "提供商不能为空"));
        }
        if self.model.is_empty() {
            return Err(AppError::bad_request("INVALID_MODEL", "模型不能为空"));
        }
        if let Some(temp) = self.temperature {
            if temp < 0.0 || temp > 2.0 {
                return Err(AppError::bad_request("INVALID_TEMPERATURE", "温度必须在 0-2 之间"));
            }
        }
        if let Some(tokens) = self.max_tokens {
            if tokens < 1 || tokens > 100000 {
                return Err(AppError::bad_request("INVALID_MAX_TOKENS", "最大 Tokens 必须在 1-100000 之间"));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateLLMConfigRequest {
    pub model: Option<String>,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    pub is_default: Option<bool>,
    pub is_active: Option<bool>,
    pub max_tokens: Option<i64>,
    pub temperature: Option<f64>,
}

/// 获取用户的 LLM 配置列表
pub async fn list_llm_configs(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<ApiSuccess<Vec<LLMConfigResponse>>>, AppError> {
    let user_id = get_user_id_from_token(&state.db, &headers).await?;

    let configs: Vec<(String, String, String, i64, i64, i64, f64)> = sqlx::query_as(
        "SELECT id, provider, model, is_default, is_active, max_tokens, temperature FROM llm_configs WHERE user_id = ? ORDER BY is_default DESC, created_at DESC"
    )
    .bind(&user_id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch LLM configs: {}", e);
        AppError::internal("获取 LLM 配置失败")
    })?;

    let response: Vec<LLMConfigResponse> = configs.into_iter().map(|c| {
        LLMConfigResponse {
            id: c.0,
            provider: c.1,
            model: c.2,
            is_default: c.3 == 1,
            is_active: c.4 == 1,
            max_tokens: c.5,
            temperature: c.6,
        }
    }).collect();

    Ok(Json(ApiSuccess::ok(response)))
}

/// 创建 LLM 配置
pub async fn create_llm_config(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<CreateLLMConfigRequest>,
) -> Result<Json<ApiSuccess<LLMConfigResponse>>, AppError> {
    let user_id = get_user_id_from_token(&state.db, &headers).await?;
    req.validate()?;

    let config_id = uuid::Uuid::new_v4().to_string();
    let is_default = req.is_default.unwrap_or(false);

    // 如果设置为默认，先取消其他默认配置
    if is_default {
        sqlx::query("UPDATE llm_configs SET is_default = 0 WHERE user_id = ?")
            .bind(&user_id)
            .execute(&state.db)
            .await
            .ok();
    }

    sqlx::query(
        r#"
        INSERT INTO llm_configs (id, user_id, provider, model, api_key, base_url, is_default, max_tokens, temperature)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(&config_id)
    .bind(&user_id)
    .bind(&req.provider)
    .bind(&req.model)
    .bind(&req.api_key)
    .bind(&req.base_url)
    .bind(if is_default { 1 } else { 0 })
    .bind(req.max_tokens.unwrap_or(2000))
    .bind(req.temperature.unwrap_or(0.7))
    .execute(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create LLM config: {}", e);
        AppError::internal("创建 LLM 配置失败")
    })?;

    Ok(Json(ApiSuccess::ok(LLMConfigResponse {
        id: config_id,
        provider: req.provider,
        model: req.model,
        is_default,
        is_active: true,
        max_tokens: req.max_tokens.unwrap_or(2000),
        temperature: req.temperature.unwrap_or(0.7),
    })))
}

/// 更新 LLM 配置
pub async fn update_llm_config(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(config_id): Path<String>,
    Json(req): Json<UpdateLLMConfigRequest>,
) -> Result<Json<ApiSuccess<serde_json::Value>>, AppError> {
    let user_id = get_user_id_from_token(&state.db, &headers).await?;

    // 如果设置为默认，先取消其他默认配置
    if req.is_default == Some(true) {
        sqlx::query("UPDATE llm_configs SET is_default = 0 WHERE user_id = ? AND id != ?")
            .bind(&user_id)
            .bind(&config_id)
            .execute(&state.db)
            .await
            .ok();
    }

    if let Some(model) = req.model {
        sqlx::query("UPDATE llm_configs SET model = ?, updated_at = datetime('now') WHERE id = ? AND user_id = ?")
            .bind(&model)
            .bind(&config_id)
            .bind(&user_id)
            .execute(&state.db)
            .await
            .ok();
    }

    if let Some(is_default) = req.is_default {
        sqlx::query("UPDATE llm_configs SET is_default = ?, updated_at = datetime('now') WHERE id = ? AND user_id = ?")
            .bind(if is_default { 1 } else { 0 })
            .bind(&config_id)
            .bind(&user_id)
            .execute(&state.db)
            .await
            .ok();
    }

    if let Some(is_active) = req.is_active {
        sqlx::query("UPDATE llm_configs SET is_active = ?, updated_at = datetime('now') WHERE id = ? AND user_id = ?")
            .bind(if is_active { 1 } else { 0 })
            .bind(&config_id)
            .bind(&user_id)
            .execute(&state.db)
            .await
            .ok();
    }

    Ok(Json(ApiSuccess::ok(serde_json::json!({"message": "配置已更新"}))))
}

/// 删除 LLM 配置
pub async fn delete_llm_config(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(config_id): Path<String>,
) -> Result<Json<ApiSuccess<serde_json::Value>>, AppError> {
    let user_id = get_user_id_from_token(&state.db, &headers).await?;

    sqlx::query("DELETE FROM llm_configs WHERE id = ? AND user_id = ?")
        .bind(&config_id)
        .bind(&user_id)
        .execute(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to delete LLM config: {}", e);
            AppError::internal("删除 LLM 配置失败")
        })?;

    Ok(Json(ApiSuccess::ok(serde_json::json!({"message": "配置已删除"}))))
}