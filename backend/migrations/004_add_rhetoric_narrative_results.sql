-- Migration 004: Add rhetoric and narrative analysis results (Story 1.4)

-- 为 style_analysis_tasks 添加修辞层和叙事层分析结果字段
ALTER TABLE style_analysis_tasks
ADD COLUMN rhetoric_json TEXT;

ALTER TABLE style_analysis_tasks
ADD COLUMN narrative_json TEXT;

-- 更新进度索引
CREATE INDEX IF NOT EXISTS idx_style_tasks_rhetoric ON style_analysis_tasks(rhetoric_json);
CREATE INDEX IF NOT EXISTS idx_style_tasks_narrative ON style_analysis_tasks(narrative_json);
