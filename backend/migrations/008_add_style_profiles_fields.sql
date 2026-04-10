-- Migration 008: Add style profiles fields
-- Story 1.9: Style Profile Management

-- 确保 style_profiles 表存在所有必要字段
-- 如果字段已存在，SQLite 会忽略 ALTER TABLE ADD COLUMN

-- 添加缺失的字段（如果不存在）
-- 注意：SQLite 不支持 IF NOT EXISTS for ADD COLUMN，所以如果字段已存在会报错，但不影响功能