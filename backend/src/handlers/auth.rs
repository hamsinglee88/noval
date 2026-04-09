use axum::{
    Json,
    extract::State,
    http::{HeaderMap, header},
};
use serde::Serialize;

use crate::{
    app_state::AppState,
    errors::AppError,
    models::user::{AuthPayload, Credentials},
    services::auth_service::AuthService,
};

#[derive(Debug, Serialize)]
pub struct ApiSuccess<T> {
    pub success: bool,
    pub data: T,
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<Credentials>,
) -> Result<Json<ApiSuccess<AuthPayload>>, AppError> {
    let service = AuthService::new(state.db);
    let data = service.register(payload).await?;

    Ok(Json(ApiSuccess {
        success: true,
        data,
    }))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<Credentials>,
) -> Result<Json<ApiSuccess<AuthPayload>>, AppError> {
    let service = AuthService::new(state.db);
    let data = service.login(payload).await?;

    Ok(Json(ApiSuccess {
        success: true,
        data,
    }))
}

pub async fn logout(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<ApiSuccess<MessageResponse>>, AppError> {
    let token = bearer_token(&headers)?;
    let service = AuthService::new(state.db);
    service.logout(&token).await?;

    Ok(Json(ApiSuccess {
        success: true,
        data: MessageResponse {
            message: "已成功登出".to_string(),
        },
    }))
}

pub async fn me(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<ApiSuccess<AuthPayload>>, AppError> {
    let token = bearer_token(&headers)?;
    let service = AuthService::new(state.db);
    let data = service.current_user(&token).await?;

    Ok(Json(ApiSuccess {
        success: true,
        data,
    }))
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
