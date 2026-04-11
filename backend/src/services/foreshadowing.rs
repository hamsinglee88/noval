use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ForeshadowType {
    Plot,       // 情节伏笔
    Character,  // 角色伏笔
    World,      // 世界观伏笔
    Emotional,  // 情感伏笔
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ForeshadowStatus {
    Active,     // 活跃（未回收）
    Resolved,   // 已回收
    Abandoned,  // 已废弃
    Overdue,    // 逾期
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Foreshadow {
    pub id: String,
    pub content: String,
    pub chapter_id: String,
    pub foreshadow_type: ForeshadowType,
    pub status: ForeshadowStatus,
    pub expected_resolution_chapter: Option<i64>,
    pub resolution_chapter: Option<String>,
    pub confidence_score: f64,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct DetectionResult {
    pub foreshadows: Vec<Foreshadow>,
    pub count: usize,
    pub suggestions: Vec<String>,
}

/// 检测文本中的伏笔
pub fn detect_foreshadows(text: &str, chapter_id: &str) -> DetectionResult {
    let mut foreshadows = Vec::new();
    
    // 伏笔关键词模式
    let patterns = [
        ("也许", ForeshadowType::Plot, 0.6),
        ("或许", ForeshadowType::Plot, 0.6),
        ("将来", ForeshadowType::Plot, 0.7),
        ("以后会", ForeshadowType::Plot, 0.8),
        ("记住", ForeshadowType::Character, 0.7),
        ("承诺", ForeshadowType::Character, 0.8),
        ("誓言", ForeshadowType::Character, 0.9),
        ("预言", ForeshadowType::World, 0.85),
        ("传说", ForeshadowType::World, 0.7),
        ("秘密", ForeshadowType::Plot, 0.75),
        ("隐藏", ForeshadowType::Plot, 0.7),
        ("暗示", ForeshadowType::Emotional, 0.65),
    ];
    
    // 按段落分割
    let paragraphs: Vec<&str> = text.split('\n').collect();
    
    for (i, paragraph) in paragraphs.iter().enumerate() {
        for (keyword, ftype, confidence) in &patterns {
            if paragraph.contains(keyword) {
                // 提取包含关键词的句子
                let sentences: Vec<&str> = paragraph.split('。').collect();
                for sentence in sentences {
                    if sentence.contains(keyword) && sentence.len() > 10 {
                        let foreshadow = Foreshadow {
                            id: uuid::Uuid::new_v4().to_string(),
                            content: sentence.trim().to_string(),
                            chapter_id: chapter_id.to_string(),
                            foreshadow_type: ftype.clone(),
                            status: ForeshadowStatus::Active,
                            expected_resolution_chapter: None,
                            resolution_chapter: None,
                            confidence_score: *confidence,
                            created_at: chrono::Utc::now().to_rfc3339(),
                        };
                        foreshadows.push(foreshadow);
                    }
                }
            }
        }
    }
    
    // 去重（简单实现）
    foreshadows.dedup_by(|a, b| a.content == b.content);
    
    let count = foreshadows.len();
    let suggestions = if count > 5 {
        vec!["检测到多个伏笔，建议重点关注高置信度的伏笔".to_string()]
    } else {
        vec![]
    };
    
    DetectionResult {
        foreshadows,
        count,
        suggestions,
    }
}

/// 计算伏笔置信度
pub fn calculate_confidence(text: &str) -> f64 {
    let indicators = [
        ("也许", 0.1_f64),
        ("或许", 0.1_f64),
        ("将来", 0.15_f64),
        ("以后", 0.1_f64),
        ("记住", 0.15_f64),
        ("承诺", 0.2_f64),
        ("誓言", 0.25_f64),
        ("预言", 0.2_f64),
        ("秘密", 0.15_f64),
    ];
    
    let mut score: f64 = 0.0;
    for (keyword, weight) in &indicators {
        if text.contains(keyword) {
            score += weight;
        }
    }
    
    score.min(1.0_f64)
}