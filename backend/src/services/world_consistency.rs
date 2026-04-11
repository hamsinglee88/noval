use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct WorldConsistencyResult {
    pub is_consistent: bool,
    pub contradictions: Vec<Contradiction>,
    pub score: f64,
}

#[derive(Debug, Serialize)]
pub struct Contradiction {
    pub rule: String,
    pub description: String,
    pub location: String,
}

pub fn check_world_consistency(text: &str, world_rules: &[&str]) -> WorldConsistencyResult {
    let mut contradictions = Vec::new();
    
    // 检查世界观规则是否被违反
    for rule in world_rules {
        // 简化检查
        if text.contains("违反") && text.contains(rule) {
            contradictions.push(Contradiction {
                rule: rule.to_string(),
                description: format!("可能违反规则: {}", rule),
                location: "待确认".to_string(),
            });
        }
    }
    
    let score = if contradictions.is_empty() { 1.0 } else { 0.7 };
    
    WorldConsistencyResult {
        is_consistent: contradictions.is_empty(),
        contradictions,
        score,
    }
}
