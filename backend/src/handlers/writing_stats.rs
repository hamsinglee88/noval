use axum::{Json, extract::State, http::HeaderMap};
use serde::Serialize;
use crate::{app_state::AppState, errors::AppError, auth_utils::{get_user_id_from_token, ApiSuccess}};

#[derive(Debug, Serialize)]
pub struct WritingStats {
    pub total_novels: i64,
    pub total_chapters: i64,
    pub total_words: i64,
    pub avg_chapter_length: f64,
    pub writing_days: i64,
    pub words_per_day: f64,
    pub recent_activity: Vec<DailyActivity>,
}

#[derive(Debug, Serialize)]
pub struct DailyActivity {
    pub date: String,
    pub words_written: i64,
    pub chapters_updated: i64,
}

pub async fn get_writing_stats(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<ApiSuccess<WritingStats>>, AppError> {
    let user_id = get_user_id_from_token(&state.db, &headers).await?;
    
    // 获取小说数
    let novel_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM novels WHERE user_id = ?")
        .bind(&user_id)
        .fetch_one(&state.db)
        .await
        .unwrap_or((0,));
    
    // 获取章节数和总字数
    let chapter_stats: (i64, i64) = sqlx::query_as(
        "SELECT COUNT(*), COALESCE(SUM(word_count), 0) FROM chapters c JOIN novels n ON c.novel_id = n.id WHERE n.user_id = ?"
    )
    .bind(&user_id)
    .fetch_one(&state.db)
    .await
    .unwrap_or((0, 0));
    
    let avg_chapter = if chapter_stats.0 > 0 {
        chapter_stats.1 as f64 / chapter_stats.0 as f64
    } else {
        0.0
    };
    
    Ok(Json(ApiSuccess::ok(WritingStats {
        total_novels: novel_count.0,
        total_chapters: chapter_stats.0,
        total_words: chapter_stats.1,
        avg_chapter_length: avg_chapter,
        writing_days: 0,
        words_per_day: 0.0,
        recent_activity: vec![],
    })))
}
