use axum::{
    Json,
    extract::State,
    http::{HeaderMap, header},
};
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, errors::AppError};

#[derive(Debug, Serialize)]
pub struct ApiSuccess<T> { pub success: bool, pub data: T }

#[derive(Debug, Deserialize)]
pub struct AIRequest {
    pub action: String,
    pub text: String,
    pub context: Option<String>,
    pub style_id: Option<String>,
}

impl AIRequest {
    pub fn validate(&self) -> Result<(), AppError> {
        if self.text.trim().is_empty() {
            return Err(AppError::bad_request("EMPTY_TEXT", "文本不能为空"));
        }
        if self.text.len() > 50_000 {
            return Err(AppError::bad_request("TEXT_TOO_LONG", "文本长度超出限制（最大50000字符）"));
        }
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct AIResponse {
    pub result: String,
    pub usage: Option<TokenUsage>,
}

#[derive(Debug, Serialize)]
pub struct TokenUsage {
    pub prompt_tokens: i64,
    pub completion_tokens: i64,
    pub total_tokens: i64,
}

/// AI 续写
pub async fn continue_text(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<AIRequest>,
) -> Result<Json<ApiSuccess<AIResponse>>, AppError> {
    let _user_id = get_user_id_from_token(&state.db, &headers).await?;
    req.validate()?;

    let prompt = build_continue_prompt(&req.text, req.context.as_deref());
    let result = call_llm(&prompt, req.style_id.as_deref()).await?;

    Ok(Json(ApiSuccess {
        success: true,
        data: AIResponse { result, usage: None },
    }))
}

/// AI 润色
pub async fn polish_text(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<AIRequest>,
) -> Result<Json<ApiSuccess<AIResponse>>, AppError> {
    let _user_id = get_user_id_from_token(&state.db, &headers).await?;
    req.validate()?;

    let prompt = build_polish_prompt(&req.text, req.context.as_deref());
    let result = call_llm(&prompt, req.style_id.as_deref()).await?;

    Ok(Json(ApiSuccess {
        success: true,
        data: AIResponse { result, usage: None },
    }))
}

/// AI 扩写
pub async fn expand_text(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<AIRequest>,
) -> Result<Json<ApiSuccess<AIResponse>>, AppError> {
    let _user_id = get_user_id_from_token(&state.db, &headers).await?;
    req.validate()?;

    let prompt = build_expand_prompt(&req.text, req.context.as_deref());
    let result = call_llm(&prompt, req.style_id.as_deref()).await?;

    Ok(Json(ApiSuccess {
        success: true,
        data: AIResponse { result, usage: None },
    }))
}

/// AI 总结
pub async fn summarize_text(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<AIRequest>,
) -> Result<Json<ApiSuccess<AIResponse>>, AppError> {
    let _user_id = get_user_id_from_token(&state.db, &headers).await?;
    req.validate()?;

    let prompt = build_summarize_prompt(&req.text);
    let result = call_llm(&prompt, None).await?;

    Ok(Json(ApiSuccess {
        success: true,
        data: AIResponse { result, usage: None },
    }))
}

/// AI 改写
pub async fn rewrite_text(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<AIRequest>,
) -> Result<Json<ApiSuccess<AIResponse>>, AppError> {
    let _user_id = get_user_id_from_token(&state.db, &headers).await?;
    req.validate()?;

    let prompt = build_rewrite_prompt(&req.text, req.context.as_deref());
    let result = call_llm(&prompt, req.style_id.as_deref()).await?;

    Ok(Json(ApiSuccess {
        success: true,
        data: AIResponse { result, usage: None },
    }))
}

// 辅助函数

fn build_continue_prompt(text: &str, context: Option<&str>) -> String {
    let context_str = context.map(|c| format!("\n\n上下文:\n{}", c)).unwrap_or_default();
    format!(
        "请基于以下文本继续创作，保持相同的风格和语气：\n\n{}{}\n\n续写内容：",
        text, context_str
    )
}

fn build_polish_prompt(text: &str, context: Option<&str>) -> String {
    let context_str = context.map(|c| format!("\n\n上下文:\n{}", c)).unwrap_or_default();
    format!(
        "请润色以下文本，改进表达和文采，但保持原意：\n\n{}{}\n\n润色后：",
        text, context_str
    )
}

fn build_expand_prompt(text: &str, context: Option<&str>) -> String {
    let context_str = context.map(|c| format!("\n\n上下文:\n{}", c)).unwrap_or_default();
    format!(
        "请将以下大纲扩展为详细的场景描写：\n\n{}{}\n\n扩展后：",
        text, context_str
    )
}

fn build_summarize_prompt(text: &str) -> String {
    format!("请总结以下文本的主要内容：\n\n{}\n\n总结：", text)
}

fn build_rewrite_prompt(text: &str, context: Option<&str>) -> String {
    let style_hint = context.unwrap_or("更加生动和富有表现力");
    format!(
        "请用{}的风格改写以下文本：\n\n{}\n\n改写后：",
        style_hint, text
    )
}

async fn call_llm(prompt: &str, _style_id: Option<&str>) -> Result<String, AppError> {
    // 这里简化处理，返回模拟结果
    // 实际应该调用配置的 LLM API (Claude/OpenAI/Ollama)
    
    // 模拟延迟
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    // 返回模拟响应
    Ok(format!("（AI 生成的内容）基于您的输入，这是一个续写示例：\n\n{}\n\n[这是 AI 续写的内容，实际应该调用 LLM API]", prompt))
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