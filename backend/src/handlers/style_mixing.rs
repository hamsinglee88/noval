use axum::{
    Json,
    extract::State,
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
pub struct MixStylesRequest {
    pub style_ids: Vec<String>,
    pub weights: Vec<f32>,
    pub custom_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct StylePreview {
    pub name: String,
    pub style_vector: Vec<f32>,
    pub layer_scores: Vec<f64>,
}

#[derive(Debug, Serialize)]
pub struct MixedStyleResult {
    pub id: String,
    pub name: String,
}

/// 预览混合效果
pub async fn preview_mix(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<MixStylesRequest>,
) -> Result<Json<ApiSuccess<StylePreview>>, AppError> {
    let user_id = get_user_id_from_token(&state.db, &headers).await?;

    // 验证输入
    if req.style_ids.len() < 2 || req.style_ids.len() > 5 {
        return Err(AppError::bad_request(
            "INVALID_STYLE_COUNT",
            "请选择 2-5 个风格档案。",
        ));
    }

    if req.style_ids.len() != req.weights.len() {
        return Err(AppError::bad_request(
            "WEIGHT_MISMATCH",
            "风格数量与权重数量不匹配。",
        ));
    }

    let weight_sum: f32 = req.weights.iter().sum();
    if (weight_sum - 100.0).abs() > 1.0 {
        return Err(AppError::bad_request(
            "INVALID_WEIGHTS",
            format!("权重总和应为 100%，当前为 {:.1}%", weight_sum),
        ));
    }

    // 获取所有风格档案
    let mut style_vectors: Vec<Vec<f32>> = Vec::new();
    let mut style_names: Vec<String> = Vec::new();

    for style_id in &req.style_ids {
        let profile: (Option<String>, String) = sqlx::query_as(
            "SELECT style_vector_json, name FROM style_profiles WHERE id = ? AND user_id = ?"
        )
        .bind(style_id)
        .bind(&user_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch style profile: {}", e);
            AppError::internal("获取风格档案失败")
        })?
        .ok_or_else(|| AppError::not_found("STYLE_NOT_FOUND", format!("风格档案 {} 不存在", style_id)))?;

        let vector: Vec<f32> = profile.0
            .and_then(|v| serde_json::from_str(&v).ok())
            .unwrap_or_else(|| vec![0.0; 128]);

        style_vectors.push(vector);
        style_names.push(profile.1);
    }

    // 计算混合向量
    let mut mixed_vector = vec![0.0f32; 128];
    for (vector, &weight) in style_vectors.iter().zip(req.weights.iter()) {
        let normalized_weight = weight / 100.0;
        for (i, &v) in vector.iter().enumerate() {
            mixed_vector[i] += v * normalized_weight;
        }
    }

    // 计算各层分数
    let layer_scores = calculate_layer_scores(&mixed_vector);

    // 生成名称
    let name = req.custom_name.unwrap_or_else(|| {
        format!("{} + {} 混合", 
            style_names.first().map(|s| s.as_str()).unwrap_or("风格"),
            style_names.get(1).map(|s| s.as_str()).unwrap_or("风格")
        )
    });

    Ok(Json(ApiSuccess {
        success: true,
        data: StylePreview {
            name,
            style_vector: mixed_vector,
            layer_scores,
        },
    }))
}

/// 保存混合风格
pub async fn save_mixed_style(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<MixStylesRequest>,
) -> Result<Json<ApiSuccess<MixedStyleResult>>, AppError> {
    let user_id = get_user_id_from_token(&state.db, &headers).await?;

    // 验证输入（同 preview_mix）
    if req.style_ids.len() < 2 || req.style_ids.len() > 5 {
        return Err(AppError::bad_request(
            "INVALID_STYLE_COUNT",
            "请选择 2-5 个风格档案。",
        ));
    }

    if req.style_ids.len() != req.weights.len() {
        return Err(AppError::bad_request(
            "WEIGHT_MISMATCH",
            "风格数量与权重数量不匹配。",
        ));
    }

    let weight_sum: f32 = req.weights.iter().sum();
    if (weight_sum - 100.0).abs() > 1.0 {
        return Err(AppError::bad_request(
            "INVALID_WEIGHTS",
            format!("权重总和应为 100%，当前为 {:.1}%", weight_sum),
        ));
    }

    // 获取所有风格档案的数据
    let mut style_data: Vec<(String, String, Option<String>)> = Vec::new();

    for style_id in &req.style_ids {
        let profile: (String, Option<String>, String) = sqlx::query_as(
            "SELECT name, style_vector_json, vocabulary_json FROM style_profiles WHERE id = ? AND user_id = ?"
        )
        .bind(style_id)
        .bind(&user_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch style profile: {}", e);
            AppError::internal("获取风格档案失败")
        })?
        .ok_or_else(|| AppError::not_found("STYLE_NOT_FOUND", format!("风格档案 {} 不存在", style_id)))?;

        style_data.push((profile.0, profile.1.unwrap_or_default(), Some(profile.2)));
    }

    // 计算混合向量
    let mut mixed_vector = vec![0.0f32; 128];
    let mut style_names: Vec<String> = Vec::new();

    for ((name, vector_json, _), &weight) in style_data.iter().zip(req.weights.iter()) {
        let vector: Vec<f32> = serde_json::from_str(&vector_json).unwrap_or_else(|_| vec![0.0; 128]);
        let normalized_weight = weight / 100.0;

        for (i, &v) in vector.iter().enumerate() {
            mixed_vector[i] += v * normalized_weight;
        }

        style_names.push(name.clone());
    }

    // 生成名称
    let name = req.custom_name.unwrap_or_else(|| {
        format!("{} + {} 混合", 
            style_names.first().map(|s| s.as_str()).unwrap_or("风格"),
            style_names.get(1).map(|s| s.as_str()).unwrap_or("风格")
        )
    });

    // 验证名称不重复
    let exists: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM style_profiles WHERE user_id = ? AND name = ?"
    )
    .bind(&user_id)
    .bind(&name)
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
    let vector_json = serde_json::to_string(&mixed_vector).map_err(|e| {
        AppError::serialization_error(format!("序列化向量失败：{}", e))
    })?;

    // 保存混合风格
    sqlx::query(
        r#"
        INSERT INTO style_profiles (id, user_id, name, style_vector_json, created_at, updated_at)
        VALUES (?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        "#
    )
    .bind(&profile_id)
    .bind(&user_id)
    .bind(&name)
    .bind(&vector_json)
    .execute(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to save mixed style: {}", e);
        AppError::internal("保存混合风格失败")
    })?;

    // 保存混合历史
    let source_styles = serde_json::to_string(&req.style_ids.iter().zip(req.weights.iter())
        .map(|(id, w)| serde_json::json!({"style_id": id, "weight": w}))
        .collect::<Vec<_>>()
    ).unwrap_or_default();

    sqlx::query(
        r#"
        INSERT INTO style_mix_history (id, result_style_id, user_id, source_styles, created_at)
        VALUES (?, ?, ?, ?, CURRENT_TIMESTAMP)
        "#
    )
    .bind(uuid::Uuid::new_v4().to_string())
    .bind(&profile_id)
    .bind(&user_id)
    .bind(&source_styles)
    .execute(&state.db)
    .await
    .ok();

    Ok(Json(ApiSuccess {
        success: true,
        data: MixedStyleResult {
            id: profile_id,
            name,
        },
    }))
}

/// 计算各层分数
fn calculate_layer_scores(vector: &[f32]) -> Vec<f64> {
    if vector.len() < 128 {
        return vec![0.0; 8];
    }

    vec![
        vector[0..16].iter().map(|&x| x.abs()).sum::<f32>() as f64 / 16.0,
        vector[16..32].iter().map(|&x| x.abs()).sum::<f32>() as f64 / 16.0,
        vector[32..48].iter().map(|&x| x.abs()).sum::<f32>() as f64 / 16.0,
        vector[48..72].iter().map(|&x| x.abs()).sum::<f32>() as f64 / 24.0,
        vector[72..88].iter().map(|&x| x.abs()).sum::<f32>() as f64 / 16.0,
        vector[88..104].iter().map(|&x| x.abs()).sum::<f32>() as f64 / 16.0,
        vector[104..116].iter().map(|&x| x.abs()).sum::<f32>() as f64 / 12.0,
        vector[116..128].iter().map(|&x| x.abs()).sum::<f32>() as f64 / 12.0,
    ]
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