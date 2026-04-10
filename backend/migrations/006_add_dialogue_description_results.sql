-- Migration 006: Add dialogue and description analysis results
-- Story 1.6: Dialogue and Description Layer Analysis

-- Add columns for dialogue layer analysis results
ALTER TABLE style_analysis_tasks
ADD COLUMN dialogue_json TEXT;

-- Add columns for description layer analysis results
ALTER TABLE style_analysis_tasks
ADD COLUMN description_json TEXT;
