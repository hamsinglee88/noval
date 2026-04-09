use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// 风格分析任务状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Pending,
    Processing,
    Failed,
    Completed,
}

impl Default for TaskStatus {
    fn default() -> Self {
        TaskStatus::Pending
    }
}

/// 风格分析任务请求
#[derive(Debug, Deserialize)]
pub struct StyleAnalysisRequest {
    #[serde(default)]
    pub source_name: Option<String>,
}

/// 风格分析任务响应
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct StyleAnalysisTaskResponse {
    pub task_id: String,
    pub status: String,
    pub progress: f64,
    pub source_file_path: String,
    pub status_message: Option<String>,
}

/// 风格分析任务数据库模型
#[derive(Debug, Clone, FromRow)]
pub struct StyleAnalysisTask {
    pub id: String,
    pub user_id: String,
    pub source_file_path: String,
    pub source_filename: String,
    pub file_size: i64,
    pub status: String,
    pub progress: f64,
    pub status_message: Option<String>,
    pub result_profile_id: Option<String>,
    pub error_message: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl StyleAnalysisTask {
    pub fn to_response(&self) -> StyleAnalysisTaskResponse {
        StyleAnalysisTaskResponse {
            task_id: self.id.clone(),
            status: self.status.clone(),
            progress: self.progress,
            source_file_path: self.source_file_path.clone(),
            status_message: self.status_message.clone(),
        }
    }
}

/// 风格档案数据库模型（预留给后续 Story）
#[derive(Debug, Clone, FromRow)]
pub struct StyleProfile {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub source_novels: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub vocabulary_json: Option<String>,
    pub sentence_json: Option<String>,
    pub rhetoric_json: Option<String>,
    pub narrative_json: Option<String>,
    pub emotional_json: Option<String>,
    pub pacing_json: Option<String>,
    pub dialogue_json: Option<String>,
    pub description_json: Option<String>,
    pub style_vector: Option<String>,
    pub example_passages: Option<String>,
}
