use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct EditReport {
    pub issues: Vec<EditIssue>,
    pub suggestions: Vec<String>,
    pub score: f64,
}

#[derive(Debug, Serialize)]
pub struct EditIssue {
    pub issue_type: String,
    pub description: String,
    pub severity: String,
    pub suggestion: String,
}

pub fn auto_edit_review(text: &str) -> EditReport {
    let mut issues = Vec::new();
    
    // 检查常见问题
    if text.contains("突然") {
        issues.push(EditIssue {
            issue_type: "叙事".to_string(),
            description: "使用了'突然'一词".to_string(),
            severity: "低".to_string(),
            suggestion: "考虑用更具体的描写替代".to_string(),
        });
    }
    
    if text.len() < 100 {
        issues.push(EditIssue {
            issue_type: "长度".to_string(),
            description: "文本过短".to_string(),
            severity: "中".to_string(),
            suggestion: "考虑扩展描写".to_string(),
        });
    }
    
    let score = if issues.is_empty() { 0.9 } else { 0.7 };
    
    EditReport {
        issues,
        suggestions: vec!["保持角色行为一致".to_string()],
        score,
    }
}
