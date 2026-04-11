-- Migration 012: Add foreshadows table
-- Story 4.1: AI 自动识别伏笔

CREATE TABLE IF NOT EXISTS foreshadows (
    id TEXT PRIMARY KEY,
    novel_id TEXT NOT NULL REFERENCES novels(id) ON DELETE CASCADE,
    chapter_id TEXT NOT NULL REFERENCES chapters(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    foreshadow_type TEXT NOT NULL,  -- Plot, Character, World, Emotional
    status TEXT NOT NULL DEFAULT 'Active',  -- Active, Resolved, Abandoned, Overdue
    expected_resolution_chapter INTEGER,
    resolution_chapter_id TEXT REFERENCES chapters(id),
    confidence_score REAL DEFAULT 0.5,
    created_at TEXT DEFAULT (datetime('now')),
    resolved_at TEXT
);

CREATE INDEX IF NOT EXISTS idx_foreshadows_novel ON foreshadows(novel_id);
CREATE INDEX IF NOT EXISTS idx_foreshadows_chapter ON foreshadows(chapter_id);
CREATE INDEX IF NOT EXISTS idx_foreshadows_status ON foreshadows(status);