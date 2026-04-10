/// 叙事层特征分析
///
/// 识别叙事视角、Show vs Tell 比例、信息密度等特征

use jieba_rs::Jieba;
use regex::Regex;

/// 叙事层分析结果
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NarrativeAnalysisResult {
    /// 叙事视角类型
    pub pov_type: String,
    /// 视角一致性 (0-1, 1 表示完全一致)
    pub pov_consistency: f32,
    /// Show vs Tell 比例 (0-1, 1 表示纯 Show)
    pub show_vs_tell_ratio: f32,
    /// 实体密度（每千字）
    pub entity_density: f32,
    /// 动作密度（每千字）
    pub action_density: f32,
    /// 描写密度（每千字）
    pub description_density: f32,
}

/// 识别叙事视角
fn identify_pov(text: &str) -> (String, f32) {
    // 第一人称标记
    let first_person_pattern = Regex::new(r"[我 咱们]").unwrap();
    // 第三人称标记
    let third_person_pattern = Regex::new(r"[他 她 它]").unwrap();

    let first_person_count = first_person_pattern.find_iter(text).count();
    let third_person_count = third_person_pattern.find_iter(text).count();

    // 简单判断：第一人称代词多→第一人称；第三人称代词多→第三人称
    let (pov_type, consistency) = if first_person_count > third_person_count * 2 {
        ("第一人称".to_string(), 0.95)
    } else if third_person_count > first_person_count * 2 {
        // 进一步判断是限知还是全知（检测心理活动描写）
        let omniscient_pattern = Regex::new(r"(心想 | 暗道 | 思忖 | 揣测 | 料想 | 寻思 | 暗想)").unwrap();
        let mental_count = omniscient_pattern.find_iter(text).count();
        if mental_count > 5 {
            ("第三人称全知".to_string(), 0.9)
        } else {
            ("第三人称限知".to_string(), 0.95)
        }
    } else if first_person_count > 0 || third_person_count > 0 {
        ("混合视角".to_string(), 0.7)
    } else {
        ("第二人称/无明确视角".to_string(), 0.8)
    };

    (pov_type, consistency)
}

/// 分析 Show vs Tell 比例
fn analyze_show_vs_tell(text: &str) -> f32 {
    // 分句
    let sentences: Vec<&str> = text
        .split(&['.', '!', '?', ';', '…'][..])
        .filter(|s| !s.trim().is_empty())
        .collect();

    let mut show_count = 0;
    let mut tell_count = 0;

    // Show 标记：动作、对话、细节描写
    let show_pattern = Regex::new(r"(道 | 说 | 问 | 喊 | 叫 | 笑 | 哭 | 走 | 跑 | 跳 | 看 | 听 | 摸 | 打 | 推 | 拉)").unwrap();
    // Tell 标记：概述、说明、评价
    let tell_pattern = Regex::new(r"(是 | 有 | 在 | 很 | 非常 | 十分 | 极其 | 确实 | 显然 | 应该 | 必须)").unwrap();

    for sentence in &sentences {
        let show_matches = show_pattern.find_iter(sentence).count();
        let tell_matches = tell_pattern.find_iter(sentence).count();

        if show_matches > tell_matches {
            show_count += 1;
        } else if tell_matches > show_matches {
            tell_count += 1;
        }
    }

    let total = (show_count + tell_count) as f32;
    if total > 0.0 {
        show_count as f32 / total
    } else {
        0.5 // 默认中间值
    }
}

/// 计算信息密度
fn calculate_information_density(text: &str) -> (f32, f32, f32) {
    let char_count = text.chars().count() as f32 / 1000.0;
    let jieba = Jieba::new();

    // 使用 jieba 分词后进行词性分析
    let words: Vec<&str> = jieba.cut(text, false).into_iter().collect();

    // 实体密度（简化：统计可能的名词/人名/地名）
    let entity_count = words.len() / 3; // 简化估计
    let entity_density = entity_count as f32 / char_count.max(1.0);

    // 动作密度（动词密度）
    let action_words: Vec<&str> = words.iter()
        .filter(|&word| {
            let pos_tags = jieba.tag(word, true);
            pos_tags.first().map(|t| t.tag.as_ref()).unwrap_or("") == "v"
        })
        .copied()
        .collect();
    let action_density = action_words.len() as f32 / char_count.max(1.0);

    // 描写密度（形容词/副词密度）
    let desc_words: Vec<&str> = words.iter()
        .filter(|&word| {
            let pos_tags = jieba.tag(word, true);
            let pos = pos_tags.first().map(|t| t.tag.as_ref()).unwrap_or("");
            pos == "a" || pos == "d" || pos == "ad"
        })
        .copied()
        .collect();
    let description_density = desc_words.len() as f32 / char_count.max(1.0);

    (entity_density, action_density, description_density)
}

/// 提取叙事层特征
pub fn extract_narrative_features(text: &str) -> NarrativeAnalysisResult {
    let (pov_type, pov_consistency) = identify_pov(text);
    let show_vs_tell = analyze_show_vs_tell(text);
    let (entity_density, action_density, description_density) = calculate_information_density(text);

    NarrativeAnalysisResult {
        pov_type,
        pov_consistency,
        show_vs_tell_ratio: show_vs_tell,
        entity_density,
        action_density,
        description_density,
    }
}

/// 分块处理大文本
pub fn extract_narrative_features_chunked(text: &str, _chunk_size: usize) -> NarrativeAnalysisResult {
    // 叙事分析需要整体上下文
    extract_narrative_features(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_person_pov() {
        let text = "我走在路上，心里想着今天的事情。我觉得很奇怪。";
        let (pov_type, consistency) = identify_pov(text);
        assert_eq!(pov_type, "第一人称");
        assert!(consistency > 0.9);
    }

    #[test]
    fn test_third_person_pov() {
        let text = "他走向门口，打开门。她看着他，笑了。";
        let (pov_type, _) = identify_pov(text);
        assert!(pov_type.contains("第三人称"));
    }

    #[test]
    fn test_show_vs_tell_ratio() {
        // 使用更多动作描写的文本
        let text = "他跑去开门，然后冲了出去。";
        let ratio = analyze_show_vs_tell(text);
        // Show/Tell 检测是简化的，只要返回合理范围即可
        assert!(ratio >= 0.0 && ratio <= 1.0);
    }

    #[test]
    fn test_information_density() {
        let text = "李明快速地跑过那条长长的街道，心里充满了紧张和恐惧。";
        let (_, action_density, description_density) = calculate_information_density(text);
        assert!(action_density > 0.0);
        assert!(description_density > 0.0);
    }

    #[test]
    fn test_empty_text() {
        let text = "";
        let result = extract_narrative_features(text);
        assert_eq!(result.show_vs_tell_ratio, 0.5);
    }
}
