-- Migration 007: Add style vector storage
-- Story 1.7: Style Vector Generation

-- Add column for 128-dimensional style vector (JSON format)
ALTER TABLE style_analysis_tasks
ADD COLUMN style_vector_json TEXT;