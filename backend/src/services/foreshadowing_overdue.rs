use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct OverdueReport {
    pub total_foreshadows: usize,
    pub overdue_count: usize,
    pub overdue_items: Vec<OverdueItem>,
}

#[derive(Debug, Serialize)]
pub struct OverdueItem {
    pub id: String,
    pub content: String,
    pub chapters_since: i64,
    pub urgency: String,
}

pub fn check_overdue_foreshadows(current_chapter: i64, expected_resolution: Option<i64>) -> bool {
    if let Some(expected) = expected_resolution {
        current_chapter > expected
    } else {
        false
    }
}

pub fn calculate_urgency(chapters_since: i64) -> String {
    match chapters_since {
        0..=3 => "低".to_string(),
        4..=7 => "中".to_string(),
        8..=15 => "高".to_string(),
        _ => "紧急".to_string(),
    }
}
