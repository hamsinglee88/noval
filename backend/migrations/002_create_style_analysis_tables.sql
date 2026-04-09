-- Migration 002: Create style analysis tables (Story 1.2)

-- 风格档案表（预留给后续 Story 完成特征提取和向量生成）
CREATE TABLE IF NOT EXISTS style_profiles (
    id UUID PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    user_id UUID NOT NULL REFERENCES users(id),
    name TEXT NOT NULL,
    source_novels TEXT,  -- JSON array of file paths
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    vocabulary_json TEXT,      -- 词汇层特征（预留）
    sentence_json TEXT,        -- 句式层特征（预留）
    rhetoric_json TEXT,        -- 修辞层特征（预留）
    narrative_json TEXT,       -- 叙事层特征（预留）
    emotional_json TEXT,       -- 情感层特征（预留）
    pacing_json TEXT,          -- 节奏层特征（预留）
    dialogue_json TEXT,        -- 对话层特征（预留）
    description_json TEXT,     -- 描写层特征（预留）
    style_vector TEXT,         -- 128 维向量（预留）
    example_passages TEXT      -- JSON array of example passages
);

-- 风格分析任务表
CREATE TABLE IF NOT EXISTS style_analysis_tasks (
    id UUID PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    user_id UUID NOT NULL REFERENCES users(id),
    source_file_path TEXT NOT NULL,
    source_filename TEXT NOT NULL,
    file_size INTEGER NOT NULL,  -- bytes
    status TEXT DEFAULT 'pending',  -- pending, processing, failed, completed
    progress REAL DEFAULT 0.0,
    status_message TEXT,
    result_profile_id UUID REFERENCES style_profiles(id),
    error_message TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_style_profiles_user ON style_profiles(user_id);
CREATE INDEX IF NOT EXISTS idx_style_tasks_user ON style_analysis_tasks(user_id);
CREATE INDEX IF NOT EXISTS idx_style_tasks_status ON style_analysis_tasks(status);
