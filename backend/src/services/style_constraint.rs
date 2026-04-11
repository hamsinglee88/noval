use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleConstraint {
    pub style_id: String,
    pub vocabulary_weight: f32,
    pub sentence_weight: f32,
    pub tone_weight: f32,
}

impl Default for StyleConstraint {
    fn default() -> Self {
        Self {
            style_id: String::new(),
            vocabulary_weight: 0.3,
            sentence_weight: 0.3,
            tone_weight: 0.4,
        }
    }
}

impl StyleConstraint {
    pub fn apply_to_prompt(&self, prompt: &str, style_data: &str) -> String {
        format!(
            "请按照以下风格约束生成内容：\n\n风格数据：{}\n\n任务：{}\n\n请保持风格一致性。",
            style_data, prompt
        )
    }
}
