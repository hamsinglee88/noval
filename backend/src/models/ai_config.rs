use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutonomyLevel {
    ConfirmBefore,  // 每次确认
    BatchConfirm,   // 批量确认
    AutoAccept,     // 自动接受
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    pub autonomy_level: AutonomyLevel,
    pub max_tokens: i64,
    pub temperature: f32,
    pub style_strictness: f32, // 0.0-1.0
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            autonomy_level: AutonomyLevel::ConfirmBefore,
            max_tokens: 2000,
            temperature: 0.7,
            style_strictness: 0.8,
        }
    }
}
