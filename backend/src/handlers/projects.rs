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
pub struct CreateProjectRequest {
    pub title: String,
    pub description: Option<String>,
    pub style_profile_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProjectSummary {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub style_profile_id: Option<String>,
    pub style_name: Option<String>,
    pub chapter_count: i64,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct ProjectDetail {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub style_profile_id: Option<String>,
    pub style_name: Option<String>,
    pub style_data: Option<serde_json::Value>,
    pub chapter_count: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProjectRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub style_profile_id: Option<String>,
}

/// 创建新项目
pub async fn create_project(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<CreateProjectRequest>,
) -> Result<Json<ApiSuccess<ProjectSummary>>, AppError> {
    let user_id = get_user_id_from_token(&state.db, &headers).await?;

    // 验证标题长度
    if req.title.len() < 2 || req.title.len() > 100 {
        return Err(AppError::bad_request(
            "INVALID_TITLE_LENGTH",
            "项目标题长度必须在 2-100 字符之间。",
        ));
    }

    // 验证标题不重复
    let exists: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM novels WHERE user_id = ? AND title = ?"
    )
    .bind(&user_id)
    .bind(&req.title)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to check title uniqueness: {}", e);
        AppError::internal("检查标题唯一性失败")
    })?;

    if exists.0 > 0 {
        return Err(AppError::bad_request(
            "DUPLICATE_TITLE",
            "该项目标题已存在。",
        ));
    }

    // 如果提供了风格档案 ID，验证其存在性
    if let Some(ref style_id) = req.style_profile_id {
        let style_exists: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM style_profiles WHERE id = ? AND user_id = ?"
        )
        .bind(style_id)
        .bind(&user_id)
        .fetch_one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to check style profile: {}", e);
            AppError::internal("检查风格档案失败")
        })?;

        if style_exists.0 == 0 {
            return Err(AppError::bad_request(
                "STYLE_NOT_FOUND",
                "选择的风格档案不存在。",
            ));
        }
    }

    let project_id = uuid::Uuid::new_v4().to_string();

    // 创建项目
    sqlx::query(
        r#"
        INSERT INTO novels (id, user_id, title, description, style_profile_id, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        "#
    )
    .bind(&project_id)
    .bind(&user_id)
    .bind(&req.title)
    .bind(&req.description)
    .bind(&req.style_profile_id)
    .execute(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create project: {}", e);
        AppError::internal("创建项目失败")
    })?;

    // 获取风格名称
    let style_name = if let Some(ref style_id) = req.style_profile_id {
        sqlx::query_as::<_, (String,)>("SELECT name FROM style_profiles WHERE id = ?")
            .bind(style_id)
            .fetch_optional(&state.db)
            .await
            .ok()
            .map(|r| r.map(|v| v.0).unwrap_or_default())
    } else {
        None
    };

    Ok(Json(ApiSuccess {
        success: true,
        data: ProjectSummary {
            id: project_id,
            title: req.title,
            description: req.description,
            style_profile_id: req.style_profile_id,
            style_name,
            chapter_count: 0,
            created_at: chrono::Utc::now().to_rfc3339(),
        },
    }))
}

/// 获取项目列表
pub async fn list_projects(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(params): Query<ListParams>,
) -> Result<Json<ApiSuccess<Vec<ProjectSummary>>>, AppError> {
    let user_id = get_user_id_from_token(&state.db, &headers).await?;

    let sort_by = params.sort_by.unwrap_or_else(|| "created_at".to_string());
    let sort_order = params.sort_order.unwrap_or_else(|| "desc".to_string());

    let order_clause = match sort_by.as_str() {
        "title" => if sort_order == "asc" { "n.title ASC" } else { "n.title DESC" },
        "created_at" => if sort_order == "asc" { "n.created_at ASC" } else { "n.created_at DESC" },
        _ => "n.created_at DESC",
    };

    let projects_result = if let Some(search) = params.search {
        let sql = format!(
            r#"
            SELECT n.id, n.title, n.description, n.style_profile_id, sp.name, n.created_at
            FROM novels n
            LEFT JOIN style_profiles sp ON n.style_profile_id = sp.id
            WHERE n.user_id = ? AND n.title LIKE ?
            ORDER BY {}
            "#,
            order_clause
        );
        
        let search_pattern = format!("%{}%", search);
        sqlx::query_as::<_, (String, String, Option<String>, Option<String>, Option<String>, String)>(&sql)
            .bind(&user_id)
            .bind(&search_pattern)
            .fetch_all(&state.db)
            .await
    } else {
        let sql = format!(
            r#"
            SELECT n.id, n.title, n.description, n.style_profile_id, sp.name, n.created_at
            FROM novels n
            LEFT JOIN style_profiles sp ON n.style_profile_id = sp.id
            WHERE n.user_id = ?
            ORDER BY {}
            "#,
            order_clause
        );
        
        sqlx::query_as::<_, (String, String, Option<String>, Option<String>, Option<String>, String)>(&sql)
            .bind(&user_id)
            .fetch_all(&state.db)
            .await
    };

    let projects = projects_result.map_err(|e| {
        tracing::error!("Failed to fetch projects: {}", e);
        AppError::internal("获取项目列表失败")
    })?;

    // 获取每个项目的章节数
    let mut summaries = Vec::new();
    for p in projects {
        let chapter_count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM chapters WHERE novel_id = ?"
        )
        .bind(&p.0)
        .fetch_one(&state.db)
        .await
        .unwrap_or((0,));

        summaries.push(ProjectSummary {
            id: p.0,
            title: p.1,
            description: p.2,
            style_profile_id: p.3,
            style_name: p.4,
            chapter_count: chapter_count.0,
            created_at: p.5,
        });
    }

    Ok(Json(ApiSuccess {
        success: true,
        data: summaries,
    }))
}

/// 获取项目详情
pub async fn get_project(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<ApiSuccess<ProjectDetail>>, AppError> {
    let user_id = get_user_id_from_token(&state.db, &headers).await?;

    let project: (String, String, Option<String>, Option<String>, Option<String>, String, String, Option<String>) = 
        sqlx::query_as(
            r#"
            SELECT n.id, n.title, n.description, n.style_profile_id, sp.name, n.created_at, n.updated_at,
                   sp.style_vector_json
            FROM novels n
            LEFT JOIN style_profiles sp ON n.style_profile_id = sp.id
            WHERE n.id = ? AND n.user_id = ?
            "#
        )
        .bind(&id)
        .bind(&user_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch project: {}", e);
            AppError::internal("获取项目失败")
        })?
        .ok_or_else(|| AppError::not_found("PROJECT_NOT_FOUND", "项目不存在"))?;

    let chapter_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM chapters WHERE novel_id = ?"
    )
    .bind(&id)
    .fetch_one(&state.db)
    .await
    .unwrap_or((0,));

    let style_data = project.7.and_then(|v| serde_json::from_str(&v).ok());

    Ok(Json(ApiSuccess {
        success: true,
        data: ProjectDetail {
            id: project.0,
            title: project.1,
            description: project.2,
            style_profile_id: project.3,
            style_name: project.4,
            style_data,
            chapter_count: chapter_count.0,
            created_at: project.5,
            updated_at: project.6,
        },
    }))
}

/// 更新项目
pub async fn update_project(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<String>,
    Json(req): Json<UpdateProjectRequest>,
) -> Result<Json<ApiSuccess<ProjectSummary>>, AppError> {
    let user_id = get_user_id_from_token(&state.db, &headers).await?;

    // 验证项目存在
    let exists: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM novels WHERE id = ? AND user_id = ?"
    )
    .bind(&id)
    .bind(&user_id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to check project: {}", e);
        AppError::internal("检查项目失败")
    })?;

    if exists.0 == 0 {
        return Err(AppError::not_found("PROJECT_NOT_FOUND", "项目不存在"));
    }

    // 更新标题
    if let Some(ref title) = req.title {
        if title.len() < 2 || title.len() > 100 {
            return Err(AppError::bad_request(
                "INVALID_TITLE_LENGTH",
                "项目标题长度必须在 2-100 字符之间。",
            ));
        }

        sqlx::query("UPDATE novels SET title = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(title)
            .bind(&id)
            .execute(&state.db)
            .await
            .map_err(|e| {
                tracing::error!("Failed to update title: {}", e);
                AppError::internal("更新标题失败")
            })?;
    }

    // 更新描述
    if let Some(ref description) = req.description {
        sqlx::query("UPDATE novels SET description = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(description)
            .bind(&id)
            .execute(&state.db)
            .await
            .map_err(|e| {
                tracing::error!("Failed to update description: {}", e);
                AppError::internal("更新描述失败")
            })?;
    }

    // 更新风格
    if let Some(ref style_id) = req.style_profile_id {
        sqlx::query("UPDATE novels SET style_profile_id = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(style_id)
            .bind(&id)
            .execute(&state.db)
            .await
            .map_err(|e| {
                tracing::error!("Failed to update style: {}", e);
                AppError::internal("更新风格失败")
            })?;
    }

    // 返回成功消息
    Ok(Json(ApiSuccess {
        success: true,
        data: ProjectSummary {
            id,
            title: req.title.unwrap_or_default(),
            description: req.description,
            style_profile_id: None,
            style_name: None,
            chapter_count: 0,
            created_at: chrono::Utc::now().to_rfc3339(),
        },
    }))
}

/// 删除项目
pub async fn delete_project(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<ApiSuccess<serde_json::Value>>, AppError> {
    let user_id = get_user_id_from_token(&state.db, &headers).await?;

    // 验证项目存在
    let exists: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM novels WHERE id = ? AND user_id = ?"
    )
    .bind(&id)
    .bind(&user_id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to check project: {}", e);
        AppError::internal("检查项目失败")
    })?;

    if exists.0 == 0 {
        return Err(AppError::not_found("PROJECT_NOT_FOUND", "项目不存在"));
    }

    // 删除项目（级联删除章节）
    sqlx::query("DELETE FROM novels WHERE id = ? AND user_id = ?")
        .bind(&id)
        .bind(&user_id)
        .execute(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to delete project: {}", e);
            AppError::internal("删除项目失败")
        })?;

    Ok(Json(ApiSuccess {
        success: true,
        data: serde_json::json!({"message": "项目已删除"}),
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