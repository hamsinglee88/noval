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

/// 伏笔关键词模式
struct Pattern {
    keyword: &'static str,
    ftype: ForeshadowType,
    weight: f64,
    context_keywords: Vec<&'static str>,  // 上下文关键词，提高置信度
}

fn get_patterns() -> Vec<Pattern> {
    vec![
        // 情节伏笔 - 高置信度模式
        Pattern {
            keyword: "将来",
            ftype: ForeshadowType::Plot,
            weight: 0.7,
            context_keywords: vec!["会", "一定", "必然", "注定"],
        },
        Pattern {
            keyword: "以后会",
            ftype: ForeshadowType::Plot,
            weight: 0.8,
            context_keywords: vec![],
        },
        Pattern {
            keyword: "终有一天",
            ftype: ForeshadowType::Plot,
            weight: 0.9,
            context_keywords: vec![],
        },
        Pattern {
            keyword: "迟早",
            ftype: ForeshadowType::Plot,
            weight: 0.85,
            context_keywords: vec![],
        },
        // 角色伏笔
        Pattern {
            keyword: "记住",
            ftype: ForeshadowType::Character,
            weight: 0.7,
            context_keywords: vec!["我的话", "这一点", "这件事"],
        },
        Pattern {
            keyword: "承诺",
            ftype: ForeshadowType::Character,
            weight: 0.8,
            context_keywords: vec!["一定", "绝不", "发誓"],
        },
        Pattern {
            keyword: "誓言",
            ftype: ForeshadowType::Character,
            weight: 0.9,
            context_keywords: vec![],
        },
        Pattern {
            keyword: "约定",
            ftype: ForeshadowType::Character,
            weight: 0.75,
            context_keywords: vec![],
        },
        // 世界观伏笔
        Pattern {
            keyword: "预言",
            ftype: ForeshadowType::World,
            weight: 0.85,
            context_keywords: vec!["说", "记载", "流传"],
        },
        Pattern {
            keyword: "传说",
            ftype: ForeshadowType::World,
            weight: 0.7,
            context_keywords: vec!["古老的", "相传", "据说"],
        },
        Pattern {
            keyword: "诅咒",
            ftype: ForeshadowType::World,
            weight: 0.8,
            context_keywords: vec![],
        },
        // 情感伏笔
        Pattern {
            keyword: "暗下决心",
            ftype: ForeshadowType::Emotional,
            weight: 0.75,
            context_keywords: vec![],
        },
        Pattern {
            keyword: "默默发誓",
            ftype: ForeshadowType::Emotional,
            weight: 0.8,
            context_keywords: vec![],
        },
        // 复合模式 - 需要多个关键词同时出现
        Pattern {
            keyword: "秘密",
            ftype: ForeshadowType::Plot,
            weight: 0.6,
            context_keywords: vec!["隐藏", "不能说", "保守"],
        },
        Pattern {
            keyword: "命运",
            ftype: ForeshadowType::World,
            weight: 0.7,
            context_keywords: vec!["注定", "改变", "选择"],
        },
    ]
}

/// 检测文本中的伏笔
pub fn detect_foreshadows(text: &str, chapter_id: &str) -> DetectionResult {
    let patterns = get_patterns();
    let mut foreshadows = Vec::new();
    let mut seen_contents = std::collections::HashSet::new();
    
    // 按段落分割
    let paragraphs: Vec<&str> = text.split('\n').filter(|p| !p.trim().is_empty()).collect();
    
    for paragraph in &paragraphs {
        // 按句子分割
        let sentences: Vec<&str> = paragraph.split(|c| c == '。' || c == '！' || c == '？' || c == '…').collect();
        
        for sentence in sentences {
            let sentence = sentence.trim();
            if sentence.len() < 10 || sentence.len() > 500 {
                continue;
            }
            
            for pattern in &patterns {
                if sentence.contains(pattern.keyword) {
                    // 计算置信度
                    let confidence = calculate_confidence_with_context(sentence, pattern);
                    
                    // 只保留高置信度的伏笔
                    if confidence >= 0.6 {
                        // 去重
                        if seen_contents.contains(sentence) {
                            continue;
                        }
                        seen_contents.insert(sentence.to_string());
                        
                        let foreshadow = Foreshadow {
                            id: uuid::Uuid::new_v4().to_string(),
                            content: sentence.to_string(),
                            chapter_id: chapter_id.to_string(),
                            foreshadow_type: pattern.ftype.clone(),
                            status: ForeshadowStatus::Active,
                            expected_resolution_chapter: None,
                            resolution_chapter: None,
                            confidence_score: confidence,
                            created_at: chrono::Utc::now().to_rfc3339(),
                        };
                        foreshadows.push(foreshadow);
                    }
                }
            }
        }
    }
    
    // 按置信度排序
    foreshadows.sort_by(|a, b| b.confidence_score.partial_cmp(&a.confidence_score).unwrap());
    
    // 限制数量
    foreshadows.truncate(20);
    
    let count = foreshadows.len();
    let suggestions = generate_suggestions(&foreshadows);
    
    DetectionResult {
        foreshadows,
        count,
        suggestions,
    }
}

/// 计算包含上下文的置信度
fn calculate_confidence_with_context(sentence: &str, pattern: &Pattern) -> f64 {
    let mut confidence = pattern.weight;
    
    // 检查上下文关键词
    for context_keyword in &pattern.context_keywords {
        if sentence.contains(context_keyword) {
            confidence += 0.1;
        }
    }
    
    // 长度惩罚 - 太短或太长的句子置信度降低
    let len = sentence.len();
    if len < 20 {
        confidence *= 0.8;
    } else if len > 200 {
        confidence *= 0.9;
    }
    
    // 包含引号增加置信度（可能是对话中的承诺）
    if sentence.contains('"') || sentence.contains('"') || sentence.contains('「') {
        confidence += 0.05;
    }
    
    confidence.min(1.0_f64)
}

/// 生成建议
fn generate_suggestions(foreshadows: &[Foreshadow]) -> Vec<String> {
    let mut suggestions = Vec::new();
    
    let high_confidence_count = foreshadows.iter().filter(|f| f.confidence_score >= 0.8).count();
    let plot_count = foreshadows.iter().filter(|f| matches!(f.foreshadow_type, ForeshadowType::Plot)).count();
    let character_count = foreshadows.iter().filter(|f| matches!(f.foreshadow_type, ForeshadowType::Character)).count();
    
    if high_confidence_count > 3 {
        suggestions.push(format!("检测到 {} 个高置信度伏笔，建议重点关注", high_confidence_count));
    }
    
    if plot_count > character_count * 2 {
        suggestions.push("情节伏笔较多，建议注意角色发展的一致性".to_string());
    }
    
    if foreshadows.len() > 10 {
        suggestions.push("伏笔数量较多，建议分批处理以保持故事连贯性".to_string());
    }
    
    suggestions
}

/// 计算伏笔置信度（简化版，用于外部调用）
pub fn calculate_confidence(text: &str) -> f64 {
    let patterns = get_patterns();
    let mut max_confidence = 0.0_f64;
    
    for pattern in &patterns {
        if text.contains(pattern.keyword) {
            let confidence = calculate_confidence_with_context(text, pattern);
            max_confidence = max_confidence.max(confidence);
        }
    }
    
    max_confidence
}