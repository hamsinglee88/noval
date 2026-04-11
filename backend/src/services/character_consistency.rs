use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ConsistencyCheckResult {
    pub is_consistent: bool,
    pub issues: Vec<ConsistencyIssue>,
    pub score: f64,
}

#[derive(Debug, Serialize)]
pub struct ConsistencyIssue {
    pub character_name: String,
    pub issue_type: String,
    pub description: String,
    pub location: String,
}

pub fn check_character_consistency(text: &str, character_traits: &[&str]) -> ConsistencyCheckResult {
    let mut issues = Vec::new();
    
    // 简化的一致性检查
    // 实际应该使用更复杂的 NLP 分析
    for trait_word in character_traits {
        if !text.contains(trait_word) {
            issues.push(ConsistencyIssue {
                character_name: "未知".to_string(),
                issue_type: "缺失特征".to_string(),
                description: format!("未找到角色特征: {}", trait_word),
                location: "全文".to_string(),
            });
        }
    }
    
    let score = if issues.is_empty() { 1.0 } else { 1.0 - (issues.len() as f64 * 0.1) };
    
    ConsistencyCheckResult {
        is_consistent: issues.is_empty(),
        issues,
        score: score.max(0.0),
    }
}
