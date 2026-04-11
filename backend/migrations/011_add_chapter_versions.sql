-- Migration 011: Add chapter versions
-- Story 2.5: Version History

CREATE TABLE IF NOT EXISTS chapter_versions (
    id TEXT PRIMARY KEY,
    chapter_id TEXT NOT NULL REFERENCES chapters(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    word_count INTEGER DEFAULT 0,
    version_number INTEGER NOT NULL,
    created_at TEXT DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_chapter_versions_chapter ON chapter_versions(chapter_id);
CREATE INDEX IF NOT EXISTS idx_chapter_versions_number ON chapter_versions(chapter_id, version_number);