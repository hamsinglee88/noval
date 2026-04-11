use axum::{Json, extract::State, http::HeaderMap};
use serde::{Deserialize, Serialize};
use crate::{app_state::AppState, errors::AppError, auth_utils::{get_user_id_from_token, ApiSuccess}};

#[derive(Debug, Serialize)]
pub struct UserPreferences {
    pub theme: String,
    pub font_size: i64,
    pub auto_save_interval: i64,
    pub default_llm_provider: Option<String>,
    pub language: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePreferencesRequest {
    pub theme: Option<String>,
    pub font_size: Option<i64>,
    pub auto_save_interval: Option<i64>,
    pub default_llm_provider: Option<String>,
    pub language: Option<String>,
}

pub async fn get_preferences(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<ApiSuccess<UserPreferences>>, AppError> {
    let _user_id = get_user_id_from_token(&state.db, &headers).await?;
    
    // 返回默认偏好
    Ok(Json(ApiSuccess::ok(UserPreferences {
        theme: "dark".to_string(),
        font_size: 16,
        auto_save_interval: 30,
        default_llm_provider: None,
        language: "zh-CN".to_string(),
    })))
}

pub async fn update_preferences(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(_req): Json<UpdatePreferencesRequest>,
) -> Result<Json<ApiSuccess<serde_json::Value>>, AppError> {
    let _user_id = get_user_id_from_token(&state.db, &headers).await?;
    
    // 简化实现
    Ok(Json(ApiSuccess::ok(serde_json::json!({"message": "偏好已更新"}))))
}
