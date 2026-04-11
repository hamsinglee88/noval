use axum::{Json, extract::{State, Query}, http::HeaderMap};
use serde::{Deserialize, Serialize};
use crate::{app_state::AppState, errors::AppError, auth_utils::{get_user_id_from_token, ApiSuccess}};

#[derive(Debug, Serialize)]
pub struct LLMStats {
    pub total_requests: i64,
    pub total_tokens: i64,
    pub total_cost_usd: f64,
    pub avg_latency_ms: f64,
    pub success_rate: f64,
    pub by_provider: Vec<ProviderStats>,
    pub daily_usage: Vec<DailyUsage>,
}

#[derive(Debug, Serialize)]
pub struct ProviderStats {
    pub provider: String,
    pub requests: i64,
    pub tokens: i64,
    pub cost_usd: f64,
}

#[derive(Debug, Serialize)]
pub struct DailyUsage {
    pub date: String,
    pub requests: i64,
    pub tokens: i64,
}

#[derive(Debug, Deserialize)]
pub struct StatsQuery {
    pub days: Option<i64>,
}

pub async fn get_llm_stats(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<StatsQuery>,
) -> Result<Json<ApiSuccess<LLMStats>>, AppError> {
    let user_id = get_user_id_from_token(&state.db, &headers).await?;
    let days = query.days.unwrap_or(30);
    
    // 获取总体统计
    let totals: (i64, i64, f64, f64, f64) = sqlx::query_as(
        r#"
        SELECT 
            COUNT(*) as total_requests,
            COALESCE(SUM(total_tokens), 0) as total_tokens,
            COALESCE(SUM(cost_usd), 0) as total_cost,
            COALESCE(AVG(latency_ms), 0) as avg_latency,
            COALESCE(AVG(CASE WHEN success = 1 THEN 1.0 ELSE 0.0 END), 0) as success_rate
        FROM llm_usage_logs 
        WHERE user_id = ? AND created_at > datetime('now', ?)
        "#
    )
    .bind(&user_id)
    .bind(format!("-{} days", days))
    .fetch_one(&state.db)
    .await
    .unwrap_or((0, 0, 0.0, 0.0, 0.0));
    
    // 按提供商统计
    let provider_stats: Vec<(String, i64, i64, f64)> = sqlx::query_as(
        r#"
        SELECT provider, COUNT(*), COALESCE(SUM(total_tokens), 0), COALESCE(SUM(cost_usd), 0)
        FROM llm_usage_logs 
        WHERE user_id = ? AND created_at > datetime('now', ?)
        GROUP BY provider
        "#
    )
    .bind(&user_id)
    .bind(format!("-{} days", days))
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();
    
    Ok(Json(ApiSuccess::ok(LLMStats {
        total_requests: totals.0,
        total_tokens: totals.1,
        total_cost_usd: totals.2,
        avg_latency_ms: totals.3,
        success_rate: totals.4,
        by_provider: provider_stats.into_iter().map(|p| ProviderStats {
            provider: p.0,
            requests: p.1,
            tokens: p.2,
            cost_usd: p.3,
        }).collect(),
        daily_usage: vec![],
    })))
}
