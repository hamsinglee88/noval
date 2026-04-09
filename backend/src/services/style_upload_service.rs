use axum::extract::Multipart;
use sqlx::SqlitePool;
use std::path::{Path, PathBuf};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::style::{StyleAnalysisTask, TaskStatus},
};

const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024; // 10MB
const SUPPORTED_EXTENSIONS: &[&str] = &["txt", "epub"];

#[derive(Clone)]
pub struct StyleUploadService {
    db: SqlitePool,
    upload_dir: PathBuf,
}

impl StyleUploadService {
    pub fn new(db: SqlitePool) -> Self {
        let upload_dir = std::env::var("NOVAL_UPLOAD_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("data/style-sources"));

        // 确保上传目录存在
        if let Err(e) = std::fs::create_dir_all(&upload_dir) {
            tracing::error!("Failed to create upload directory: {}", e);
        }

        Self { db, upload_dir }
    }

    /// 处理文件上传并创建分析任务
    pub async fn upload_and_create_task(
        &self,
        mut multipart: Multipart,
        user_id: &str,
    ) -> Result<StyleAnalysisTask, AppError> {
        // 获取文件字段
        let field = multipart.next_field().await?;
        let Some(field) = field else {
            return Err(AppError::bad_request(
                "NO_FILE",
                "未找到上传文件。",
            ));
        };

        let file_name = field.file_name().unwrap_or("unknown.txt").to_string();
        let _content_type = field.content_type().map(|ct| ct.to_string());

        // 验证文件扩展名
        let extension = Path::new(&file_name)
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_lowercase())
            .ok_or_else(|| AppError::bad_request(
                "INVALID_FILENAME",
                "无法识别文件格式，请使用 .txt 或 .epub 扩展名。",
            ))?;

        if !SUPPORTED_EXTENSIONS.contains(&extension.as_str()) {
            return Err(AppError::bad_request(
                "UNSUPPORTED_FILE_TYPE",
                format!("仅支持 TXT 和 EPUB 文件格式。"),
            ));
        }

        // 流式读取文件并计算大小
        let task_id = Uuid::new_v4().to_string();
        let task_dir = self.upload_dir.join(&task_id);
        std::fs::create_dir_all(&task_dir).map_err(|e| {
            AppError::internal(format!("无法创建存储目录：{}", e))
        })?;

        let file_path = task_dir.join(format!("source.{}", extension));
        let mut file = tokio::fs::File::create(&file_path).await.map_err(|e| {
            AppError::internal(format!("无法创建文件：{}", e))
        })?;

        // 流式写入文件
        let mut file_size: u64 = 0;
        let mut stream = field;
        while let Some(chunk) = stream.chunk().await? {
            file_size += chunk.len() as u64;

            // 检查文件大小限制
            if file_size > MAX_FILE_SIZE {
                // 清理已写入的文件
                drop(file);
                tokio::fs::remove_file(&file_path).await.ok();
                std::fs::remove_dir(&task_dir).ok();

                return Err(AppError::bad_request(
                    "FILE_TOO_LARGE",
                    format!("文件大小超过 {}MB 限制。", MAX_FILE_SIZE / 1024 / 1024),
                ));
            }

            use tokio::io::AsyncWriteExt;
            file.write_all(&chunk).await.map_err(|e| {
                AppError::internal(format!("写入文件失败：{}", e))
            })?;
        }

        // 验证文件不为空
        if file_size == 0 {
            tokio::fs::remove_file(&file_path).await.ok();
            std::fs::remove_dir(&task_dir).ok();
            return Err(AppError::bad_request(
                "EMPTY_FILE",
                "上传的文件为空。",
            ));
        }

        // 创建任务记录
        let now = iso_timestamp_now();
        let task = sqlx::query_as::<_, StyleAnalysisTask>(
            "INSERT INTO style_analysis_tasks
             (user_id, source_file_path, source_filename, file_size, status, progress, status_message, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(user_id)
        .bind(file_path.to_str().ok_or_else(|| AppError::internal("无效文件路径"))?)
        .bind(&file_name)
        .bind(file_size as i64)
        .bind(TaskStatus::Processing.to_string())
        .bind(0.0)
        .bind("文件上传完成，准备分析...")
        .bind(&now)
        .bind(&now)
        .fetch_one(&self.db)
        .await?;

        Ok(task)
    }

    /// 获取任务状态
    pub async fn get_task(&self, task_id: &str, user_id: &str) -> Result<StyleAnalysisTask, AppError> {
        sqlx::query_as::<_, StyleAnalysisTask>(
            "SELECT id, user_id, source_file_path, source_filename, file_size,
                    status, progress, status_message, result_profile_id, error_message,
                    created_at, updated_at
             FROM style_analysis_tasks
             WHERE id = ? AND user_id = ?"
        )
        .bind(task_id)
        .bind(user_id)
        .fetch_optional(&self.db)
        .await?
        .ok_or_else(|| AppError::not_found(
            "TASK_NOT_FOUND",
            "未找到该分析任务。",
        ))
    }

    /// 更新任务状态
    pub async fn update_task_status(
        &self,
        task_id: &str,
        status: TaskStatus,
        progress: f64,
        message: Option<&str>,
    ) -> Result<(), AppError> {
        let now = iso_timestamp_now();
        let rows = sqlx::query(
            "UPDATE style_analysis_tasks
             SET status = ?, progress = ?, status_message = ?, updated_at = ?
             WHERE id = ?"
        )
        .bind(status.to_string())
        .bind(progress)
        .bind(message)
        .bind(&now)
        .bind(task_id)
        .execute(&self.db)
        .await?;

        if rows.rows_affected() == 0 {
            return Err(AppError::not_found(
                "TASK_NOT_FOUND",
                "未找到该分析任务。",
            ));
        }

        Ok(())
    }

    /// 标记任务失败
    pub async fn mark_task_failed(&self, task_id: &str, error_message: &str) -> Result<(), AppError> {
        self.update_task_status(task_id, TaskStatus::Failed, 0.0, Some(error_message)).await
    }
}

fn iso_timestamp_now() -> String {
    use time::format_description::well_known::Rfc3339;
    OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .expect("rfc3339 timestamp")
}

impl TaskStatus {
    fn to_string(&self) -> String {
        match self {
            TaskStatus::Pending => "pending".to_string(),
            TaskStatus::Processing => "processing".to_string(),
            TaskStatus::Failed => "failed".to_string(),
            TaskStatus::Completed => "completed".to_string(),
        }
    }
}
