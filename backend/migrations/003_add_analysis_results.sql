-- Migration 003: Add analysis result storage (Story 1.3)

-- 为 style_analysis_tasks 添加分析结果存储字段
ALTER TABLE style_analysis_tasks
ADD COLUMN vocabulary_json TEXT;

ALTER TABLE style_analysis_tasks
ADD COLUMN sentence_json TEXT;

-- 添加状态索引，支持词汇层和句式层完成状态
CREATE INDEX IF NOT EXISTS idx_style_tasks_progress ON style_analysis_tasks(progress);
