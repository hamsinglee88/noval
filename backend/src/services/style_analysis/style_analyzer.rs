/// 风格分析统一入口
///
/// 协调词汇层、句式层、修辞层、叙事层、情感层、节奏层、对话层、描写层等分析流程

use crate::errors::AppError;
use crate::services::style_analysis::{
    extract_vocabulary_features, extract_sentence_features,
    extract_rhetoric_features, extract_narrative_features,
    extract_emotion_features, extract_pacing_features,
    extract_dialogue_features, extract_description_features,
    extract_vocabulary_features_chunked, extract_sentence_features_chunked,
    extract_rhetoric_features_chunked, extract_narrative_features_chunked,
    extract_emotion_features_chunked, extract_pacing_features_chunked,
    extract_dialogue_features_chunked, extract_description_features_chunked,
    VocabularyAnalysisResult, SentenceAnalysisResult,
    RhetoricAnalysisResult, NarrativeAnalysisResult,
    EmotionAnalysisResult, PacingAnalysisResult,
    DialogueAnalysisResult, DescriptionAnalysisResult,
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

    /// 执行修辞层和叙事层分析（追加到词汇/句式分析之后）
    pub async fn analyze_rhetoric_and_narrative(
        &self,
        task_id: &str,
        text: &str,
    ) -> Result<(RhetoricAnalysisResult, NarrativeAnalysisResult), AppError> {
        // Step 3: 执行修辞层分析
        let rhetoric_result = if self.config.use_chunked {
            extract_rhetoric_features_chunked(text, self.config.chunk_size)
        } else {
            extract_rhetoric_features(text)
        };

        // 更新进度到 37.5%
        self.update_task_progress(task_id, 0.375, "rhetoric_completed", "修辞层分析完成").await?;

        // 保存修辞分析结果
        self.save_rhetoric_result(task_id, &rhetoric_result).await?;

        // Step 4: 执行叙事层分析
        let narrative_result = if self.config.use_chunked {
            extract_narrative_features_chunked(text, self.config.chunk_size)
        } else {
            extract_narrative_features(text)
        };

        // 更新进度到 50%
        self.update_task_progress(task_id, 0.50, "narrative_completed", "叙事层分析完成").await?;

        // 保存叙事分析结果
        self.save_narrative_result(task_id, &narrative_result).await?;

        Ok((rhetoric_result, narrative_result))
    }

    /// 保存修辞分析结果
    async fn save_rhetoric_result(
        &self,
        task_id: &str,
        result: &RhetoricAnalysisResult,
    ) -> Result<(), AppError> {
        let json = serde_json::to_string(result).map_err(|e| {
            AppError::serialization_error(format!("修辞分析结果序列化失败：{}", e))
        })?;

        sqlx::query(
            r#"
            UPDATE style_analysis_tasks
            SET rhetoric_json = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#,
        )
        .bind(json)
        .bind(task_id)
        .execute(&self.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to save rhetoric result: {}", e);
            AppError::internal(format!("保存修辞分析结果失败：{}", e))
        })?;

        Ok(())
    }

    /// 保存叙事分析结果
    async fn save_narrative_result(
        &self,
        task_id: &str,
        result: &NarrativeAnalysisResult,
    ) -> Result<(), AppError> {
        let json = serde_json::to_string(result).map_err(|e| {
            AppError::serialization_error(format!("叙事分析结果序列化失败：{}", e))
        })?;

        sqlx::query(
            r#"
            UPDATE style_analysis_tasks
            SET narrative_json = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#,
        )
        .bind(json)
        .bind(task_id)
        .execute(&self.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to save narrative result: {}", e);
            AppError::internal(format!("保存叙事分析结果失败：{}", e))
        })?;

        Ok(())
    }

    /// 执行情感层和节奏层分析（追加到修辞/叙事分析之后）
    pub async fn analyze_emotion_and_pacing(
        &self,
        task_id: &str,
        text: &str,
        chapters: &[&str],
    ) -> Result<(EmotionAnalysisResult, PacingAnalysisResult), AppError> {
        // Step 5: 执行情感层分析
        let emotion_result = if self.config.use_chunked {
            extract_emotion_features_chunked(text, self.config.chunk_size)
        } else {
            extract_emotion_features(text)
        };

        // 更新进度到 62.5%
        self.update_task_progress(task_id, 0.625, "emotion_completed", "情感层分析完成").await?;

        // 保存情感分析结果
        self.save_emotion_result(task_id, &emotion_result).await?;

        // Step 6: 执行节奏层分析
        let pacing_result = if self.config.use_chunked {
            extract_pacing_features_chunked(text, chapters, self.config.chunk_size)
        } else {
            extract_pacing_features(text, chapters)
        };

        // 更新进度到 75%
        self.update_task_progress(task_id, 0.75, "pacing_completed", "节奏层分析完成").await?;

        // 保存节奏分析结果
        self.save_pacing_result(task_id, &pacing_result).await?;

        Ok((emotion_result, pacing_result))
    }

    /// 保存情感分析结果
    async fn save_emotion_result(
        &self,
        task_id: &str,
        result: &EmotionAnalysisResult,
    ) -> Result<(), AppError> {
        let json = serde_json::to_string(result).map_err(|e| {
            AppError::serialization_error(format!("情感分析结果序列化失败：{}", e))
        })?;

        sqlx::query(
            r#"
            UPDATE style_analysis_tasks
            SET emotion_json = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#,
        )
        .bind(json)
        .bind(task_id)
        .execute(&self.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to save emotion result: {}", e);
            AppError::internal(format!("保存情感分析结果失败：{}", e))
        })?;

        Ok(())
    }

    /// 保存节奏分析结果
    async fn save_pacing_result(
        &self,
        task_id: &str,
        result: &PacingAnalysisResult,
    ) -> Result<(), AppError> {
        let json = serde_json::to_string(result).map_err(|e| {
            AppError::serialization_error(format!("节奏分析结果序列化失败：{}", e))
        })?;

        sqlx::query(
            r#"
            UPDATE style_analysis_tasks
            SET pacing_json = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#,
        )
        .bind(json)
        .bind(task_id)
        .execute(&self.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to save pacing result: {}", e);
            AppError::internal(format!("保存节奏分析结果失败：{}", e))
        })?;

        Ok(())
    }

    /// 执行对话层和描写层分析（追加到情感/节奏分析之后）
    pub async fn analyze_dialogue_and_description(
        &self,
        task_id: &str,
        text: &str,
    ) -> Result<(DialogueAnalysisResult, DescriptionAnalysisResult), AppError> {
        // Step 7: 执行对话层分析
        let dialogue_result = if self.config.use_chunked {
            extract_dialogue_features_chunked(text, self.config.chunk_size)
        } else {
            extract_dialogue_features(text)
        };

        // 更新进度到 87.5%
        self.update_task_progress(task_id, 0.875, "dialogue_completed", "对话层分析完成").await?;

        // 保存对话分析结果
        self.save_dialogue_result(task_id, &dialogue_result).await?;

        // Step 8: 执行描写层分析
        let description_result = if self.config.use_chunked {
            extract_description_features_chunked(text, self.config.chunk_size)
        } else {
            extract_description_features(text)
        };

        // 更新进度到 100%
        self.update_task_progress(task_id, 1.0, "all_layers_completed", "七层分析全部完成").await?;

        // 保存描写分析结果
        self.save_description_result(task_id, &description_result).await?;

        Ok((dialogue_result, description_result))
    }

    /// 保存对话分析结果
    async fn save_dialogue_result(
        &self,
        task_id: &str,
        result: &DialogueAnalysisResult,
    ) -> Result<(), AppError> {
        let json = serde_json::to_string(result).map_err(|e| {
            AppError::serialization_error(format!("对话分析结果序列化失败：{}", e))
        })?;

        sqlx::query(
            r#"
            UPDATE style_analysis_tasks
            SET dialogue_json = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#,
        )
        .bind(json)
        .bind(task_id)
        .execute(&self.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to save dialogue result: {}", e);
            AppError::internal(format!("保存对话分析结果失败：{}", e))
        })?;

        Ok(())
    }

    /// 保存描写分析结果
    async fn save_description_result(
        &self,
        task_id: &str,
        result: &DescriptionAnalysisResult,
    ) -> Result<(), AppError> {
        let json = serde_json::to_string(result).map_err(|e| {
            AppError::serialization_error(format!("描写分析结果序列化失败：{}", e))
        })?;

        sqlx::query(
            r#"
            UPDATE style_analysis_tasks
            SET description_json = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#,
        )
        .bind(json)
        .bind(task_id)
        .execute(&self.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to save description result: {}", e);
            AppError::internal(format!("保存描写分析结果失败：{}", e))
        })?;

        Ok(())
    }

    /// 完整的七层分析流程（词汇、句式、修辞、叙事、情感、节奏、对话、描写）
    pub async fn analyze_all_layers(
        &self,
        task_id: &str,
        file_path: &str,
    ) -> Result<(), AppError> {
        // 读取文件内容
        let text = tokio::fs::read_to_string(file_path)
            .await
            .map_err(|e| {
                tracing::error!("Failed to read file {}: {}", file_path, e);
                AppError::file_io_error(format!("读取文件失败：{}", e))
            })?;

        // 执行词汇层和句式层分析
        self.analyze_vocabulary_and_sentence(task_id, &text).await?;

        // 执行修辞层和叙事层分析
        self.analyze_rhetoric_and_narrative(task_id, &text).await?;

        // 执行情感层和节奏层分析
        // 简单按章节分割：按空行分块作为"章节"
        let chapters: Vec<&str> = text.split("\n\n").filter(|s| !s.is_empty()).collect();
        self.analyze_emotion_and_pacing(task_id, &text, &chapters).await?;

        // 执行对话层和描写层分析
        self.analyze_dialogue_and_description(task_id, &text).await?;

        // 标记为完成（100%）
        self.update_task_progress(task_id, 1.0, "completed", "七层分析全部完成").await?;

        Ok(())
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
