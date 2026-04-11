-- Migration 010: Add style mix history
-- Story 1.11: Style Mixing

-- 混合历史记录表
CREATE TABLE IF NOT EXISTS style_mix_history (
    id TEXT PRIMARY KEY,
    result_style_id TEXT REFERENCES style_profiles(id) ON DELETE CASCADE,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    source_styles TEXT NOT NULL,  -- JSON: [{style_id, weight}, ...]
    created_at TEXT DEFAULT (datetime('now'))
);

-- 索引
CREATE INDEX IF NOT EXISTS idx_style_mix_history_result ON style_mix_history(result_style_id);
CREATE INDEX IF NOT EXISTS idx_style_mix_history_user ON style_mix_history(user_id);