/// 风格分析统一入口
///
/// 协调词汇层、句式层等分析流程

use crate::errors::AppError;
use crate::services::style_analysis::{
    extract_vocabulary_features, extract_sentence_features,
    extract_vocabulary_features_chunked, extract_sentence_features_chunked,
    VocabularyAnalysisResult, SentenceAnalysisResult,
};
use sqlx::SqlitePool;
use serde_json;

/// 分析器配置
pub struct AnalyzerConfig {
    pub chunk_size: usize,
    pub use_chunked: bool,
}

impl Default for AnalyzerConfig {
    fn default() -> Self {
        Self {
            chunk_size: 100_000, // 10 万字
            use_chunked: true,
        }
    }
}

/// 风格分析器
pub struct StyleAnalyzer {
    db: SqlitePool,
    config: AnalyzerConfig,
}

impl StyleAnalyzer {
    pub fn new(db: SqlitePool) -> Self {
        Self {
            db,
            config: AnalyzerConfig::default(),
        }
    }

    pub fn with_config(db: SqlitePool, config: AnalyzerConfig) -> Self {
        Self { db, config }
    }

    /// 执行词汇层和句式层分析
    pub async fn analyze_vocabulary_and_sentence(
        &self,
        task_id: &str,
        text: &str,
    ) -> Result<(VocabularyAnalysisResult, SentenceAnalysisResult), AppError> {
        // Step 1: 执行词汇层分析
        let vocab_result = if self.config.use_chunked {
            extract_vocabulary_features_chunked(text, self.config.chunk_size)
        } else {
            extract_vocabulary_features(text)
        };

        // 更新进度到 12.5%
        self.update_task_progress(task_id, 0.125, "vocabulary_completed", "词汇层分析完成").await?;

        // 保存词汇分析结果
        self.save_vocabulary_result(task_id, &vocab_result).await?;

        // Step 2: 执行句式层分析
        let sentence_result = if self.config.use_chunked {
            extract_sentence_features_chunked(text, self.config.chunk_size)
        } else {
            extract_sentence_features(text)
        };

        // 更新进度到 25%
        self.update_task_progress(task_id, 0.25, "sentence_completed", "句式层分析完成").await?;

        // 保存句式分析结果
        self.save_sentence_result(task_id, &sentence_result).await?;

        Ok((vocab_result, sentence_result))
    }

    /// 更新任务进度
    async fn update_task_progress(
        &self,
        task_id: &str,
        progress: f64,
        status: &str,
        message: &str,
    ) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE style_analysis_tasks
            SET progress = ?, status = ?, status_message = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#,
        )
        .bind(progress)
        .bind(status)
        .bind(message)
        .bind(task_id)
        .execute(&self.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to update task progress: {}", e);
            AppError::internal(format!("更新任务进度失败：{}", e))
        })?;

        Ok(())
    }

    /// 保存词汇分析结果
    async fn save_vocabulary_result(
        &self,
        task_id: &str,
        result: &VocabularyAnalysisResult,
    ) -> Result<(), AppError> {
        let json = serde_json::to_string(result).map_err(|e| {
            AppError::serialization_error(format!("词汇分析结果序列化失败：{}", e))
        })?;

        sqlx::query(
            r#"
            UPDATE style_analysis_tasks
            SET vocabulary_json = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#,
        )
        .bind(json)
        .bind(task_id)
        .execute(&self.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to save vocabulary result: {}", e);
            AppError::internal(format!("保存词汇分析结果失败：{}", e))
        })?;

        Ok(())
    }

    /// 保存句式分析结果
    async fn save_sentence_result(
        &self,
        task_id: &str,
        result: &SentenceAnalysisResult,
    ) -> Result<(), AppError> {
        let json = serde_json::to_string(result).map_err(|e| {
            AppError::serialization_error(format!("句式分析结果序列化失败：{}", e))
        })?;

        sqlx::query(
            r#"
            UPDATE style_analysis_tasks
            SET sentence_json = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#,
        )
        .bind(json)
        .bind(task_id)
        .execute(&self.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to save sentence result: {}", e);
            AppError::internal(format!("保存句式分析结果失败：{}", e))
        })?;

        Ok(())
    }

    /// 从文件读取文本并分析
    pub async fn analyze_from_file(
        &self,
        task_id: &str,
        file_path: &str,
    ) -> Result<(VocabularyAnalysisResult, SentenceAnalysisResult), AppError> {
        // 读取文件内容
        let text = tokio::fs::read_to_string(file_path)
            .await
            .map_err(|e| {
                tracing::error!("Failed to read file {}: {}", file_path, e);
                AppError::file_io_error(format!("读取文件失败：{}", e))
            })?;

        // 执行分析
        self.analyze_vocabulary_and_sentence(task_id, &text).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyzer_config_default() {
        let config = AnalyzerConfig::default();
        assert_eq!(config.chunk_size, 100_000);
        assert!(config.use_chunked);
    }
}
