use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct StyleMatchResult {
    pub score: f64,
    pub matches: Vec<StyleMatch>,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct StyleMatch {
    pub layer: String,
    pub score: f64,
    pub description: String,
}

pub fn calculate_style_match(generated: &str, reference_style: &str) -> StyleMatchResult {
    // 简化的匹配度计算
    let score = 0.75; // 模拟分数
    
    StyleMatchResult {
        score,
        matches: vec![
            StyleMatch { layer: "词汇层".to_string(), score: 0.8, description: "词汇使用符合风格".to_string() },
            StyleMatch { layer: "句式层".to_string(), score: 0.7, description: "句式结构基本一致".to_string() },
        ],
        suggestions: vec!["建议增加更多感官描写".to_string()],
    }
}
