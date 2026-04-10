-- Migration 005: Add emotion and pacing analysis results
-- Story 1.5: Emotion and Rhythm Analysis

-- Add columns for emotion layer analysis results
ALTER TABLE style_analysis_tasks
ADD COLUMN emotion_json TEXT;

-- Add columns for pacing layer analysis results
ALTER TABLE style_analysis_tasks
ADD COLUMN pacing_json TEXT;
