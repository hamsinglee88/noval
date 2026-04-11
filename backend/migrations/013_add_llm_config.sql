-- Migration 013: Add LLM configuration
-- Story 5.1: LLM 路由配置

CREATE TABLE IF NOT EXISTS llm_configs (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    provider TEXT NOT NULL,  -- claude, openai, ollama
    model TEXT NOT NULL,
    api_key TEXT,
    base_url TEXT,
    is_default INTEGER DEFAULT 0,
    is_active INTEGER DEFAULT 1,
    max_tokens INTEGER DEFAULT 2000,
    temperature REAL DEFAULT 0.7,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS llm_usage_logs (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    config_id TEXT REFERENCES llm_configs(id),
    provider TEXT NOT NULL,
    model TEXT NOT NULL,
    action TEXT NOT NULL,
    prompt_tokens INTEGER DEFAULT 0,
    completion_tokens INTEGER DEFAULT 0,
    total_tokens INTEGER DEFAULT 0,
    cost_usd REAL DEFAULT 0,
    latency_ms INTEGER DEFAULT 0,
    success INTEGER DEFAULT 1,
    error_message TEXT,
    created_at TEXT DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_llm_configs_user ON llm_configs(user_id);
CREATE INDEX IF NOT EXISTS idx_llm_usage_user ON llm_usage_logs(user_id);
CREATE INDEX IF NOT EXISTS idx_llm_usage_created ON llm_usage_logs(created_at);