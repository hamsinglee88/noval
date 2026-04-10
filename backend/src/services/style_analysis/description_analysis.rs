/// 描写层特征分析
///
/// 提取描写比例、详细程度、修饰词密度、描写偏好等特征

/// 描写层分析结果
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DescriptionAnalysisResult {
    /// 描写内容比例 (0-1)
    pub description_ratio: f32,
    /// 描写详细程度（细节密度）
    pub detail_granularity: f32,
    /// 修饰词密度（形容词/副词）
    pub modifier_density: f32,
    /// 动作描写比例 (0-1)
    pub action_description_ratio: f32,
    /// 环境描写比例 (0-1)
    pub environment_description_ratio: f32,
    /// 心理描写比例 (0-1)
    pub psychological_description_ratio: f32,
    /// 外貌描写比例 (0-1)
    pub appearance_description_ratio: f32,
}

/// 统计描写内容比例（与对话相对）
pub fn calculate_description_ratio(text: &str) -> f32 {
    // 描写比例 ≈ 1 - 对话比例
    let dialogue_ratio = super::dialogue_analysis::calculate_dialogue_ratio(text);
    1.0 - dialogue_ratio
}

/// 分析描写详细程度
fn analyze_detail_granularity(text: &str) -> f32 {
    let detail_indicators = ["微微", "缓缓", "轻轻", "仔细", "认真", "专注",
                             "细致", "精细", "清晰", "分明", "层层", "片片",
                             "点点", "丝丝", "阵阵", "片片", "朵朵", "棵棵"];

    let detail_count = count_keyword_occurrences(text, &detail_indicators);
    let total_chars = text.chars().count() as f32;

    if total_chars > 0.0 {
        detail_count as f32 / total_chars
    } else {
        0.0
    }
}

/// 分析修饰词密度
fn analyze_modifier_density(text: &str) -> f32 {
    let modifiers = ["美丽", "漂亮", "好看", "英俊", "潇洒", "优雅", "温柔",
                     "快速", "迅速", "慢慢", "渐渐", "悄悄", "默默", "静静",
                     "轻轻", "重重", "狠狠", "微微", "稍稍", "十分", "非常",
                     "极其", "格外", "特别", "相当", "颇为", "愈发", "更加"];

    let modifier_count = count_keyword_occurrences(text, &modifiers);
    let total_chars = text.chars().count() as f32;

    if total_chars > 0.0 {
        modifier_count as f32 / total_chars
    } else {
        0.0
    }
}

fn count_keyword_occurrences(text: &str, keywords: &[&str]) -> usize {
    keywords.iter().map(|&kw| text.matches(kw).count()).sum()
}

/// 分析描写偏好
fn analyze_description_preferences(text: &str) -> (f32, f32, f32, f32) {
    let action_words = ["打", "斗", "战", "冲", "跑", "跳", "飞", "掠", "劈", "砍",
                        "闪", "躲", "避", "攻", "守", "跃", "翻", "转", "旋",
                        "踢", "捶", "推", "拉", "拽", "扯", "拔", "握", "抓"];

    let environment_words = ["天", "地", "山", "水", "风", "云", "日", "月", "星", "辰",
                             "花", "草", "树", "木", "江", "河", "湖", "海", "景", "色",
                             "雨", "雪", "霜", "雾", "霞", "虹", "泉", "石", "林", "田"];

    let psychological_words = ["想", "思", "念", "忆", "悟", "忖", "揣", "摩", "觉", "感",
                               "心", "神", "魂", "魄", "意", "志", "情", "绪",
                               "愁", "怨", "悔", "憾", "愿", "盼", "冀", "祈"];

    let appearance_words = ["眉", "眼", "鼻", "口", "耳", "发", "脸", "容", "貌", "身",
                            "形", "姿", "态", "衣", "袍", "衫", "裙", "履", "剑", "刀",
                            "眸", "颊", "唇", "齿", "颈", "肩", "腰", "腿", "足", "手"];

    let action_count = count_keyword_occurrences(text, &action_words);
    let env_count = count_keyword_occurrences(text, &environment_words);
    let psycho_count = count_keyword_occurrences(text, &psychological_words);
    let appear_count = count_keyword_occurrences(text, &appearance_words);

    let total = (action_count + env_count + psycho_count + appear_count) as f32;

    if total > 0.0 {
        (
            action_count as f32 / total,
            env_count as f32 / total,
            psycho_count as f32 / total,
            appear_count as f32 / total,
        )
    } else {
        (0.25, 0.25, 0.25, 0.25)
    }
}

/// 提取描写层特征
pub fn extract_description_features(text: &str) -> DescriptionAnalysisResult {
    let description_ratio = calculate_description_ratio(text);
    let detail_granularity = analyze_detail_granularity(text);
    let modifier_density = analyze_modifier_density(text);
    let (action_ratio, env_ratio, psycho_ratio, appear_ratio) = analyze_description_preferences(text);

    DescriptionAnalysisResult {
        description_ratio,
        detail_granularity,
        modifier_density,
        action_description_ratio: action_ratio,
        environment_description_ratio: env_ratio,
        psychological_description_ratio: psycho_ratio,
        appearance_description_ratio: appear_ratio,
    }
}

pub fn extract_description_features_chunked(text: &str, _chunk_size: usize) -> DescriptionAnalysisResult {
    extract_description_features(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_description_ratio() {
        let text = "天空湛蓝，白云飘飘。他静静地站着。";
        let ratio = calculate_description_ratio(text);
        assert!(ratio > 0.0 && ratio <= 1.0);
    }

    #[test]
    fn test_detail_granularity() {
        let text = "他轻轻地、缓缓地将手放在桌上，细致地观察着每一个细节。";
        let granularity = analyze_detail_granularity(text);
        assert!(granularity > 0.0);
    }

    #[test]
    fn test_modifier_density() {
        let text = "美丽的花朵在微风中轻轻地摇曳，非常优雅。";
        let density = analyze_modifier_density(text);
        assert!(density > 0.0);
    }

    #[test]
    fn test_description_preferences_action() {
        let text = "他打斗起来，动作迅猛，一脚踢飞了对手。";
        let (action, env, _psycho, _appear) = analyze_description_preferences(text);
        assert!(action > env || action > _psycho);
    }

    #[test]
    fn test_description_preferences_environment() {
        let text = "周围环境优美，山水如画，蓝天白云，花草茂盛。";
        let (action, env, _psycho, _appear) = analyze_description_preferences(text);
        assert!(env > action);
    }

    #[test]
    fn test_description_preferences_psychological() {
        let text = "他心中想着，思念着远方的亲人，心情复杂。";
        let (action, env, psycho, _appear) = analyze_description_preferences(text);
        assert!(psycho > action && psycho > env);
    }

    #[test]
    fn test_description_preferences_appearance() {
        let text = "她眉清目秀，眼眸明亮，嘴唇微翘，身姿优雅。";
        let (action, env, _psycho, appear) = analyze_description_preferences(text);
        assert!(appear > action && appear > env);
    }

    #[test]
    fn test_empty_text() {
        let text = "";
        let result = extract_description_features(text);
        assert_eq!(result.description_ratio, 1.0);
        assert_eq!(result.detail_granularity, 0.0);
    }

    #[test]
    fn test_extract_description_features_complete() {
        let text = "天空湛蓝如洗，白云朵朵。他轻轻地走着，心中想着心事。";
        let result = extract_description_features(text);
        assert!(result.description_ratio > 0.5);
        assert!(result.environment_description_ratio > 0.0 || result.psychological_description_ratio > 0.0);
    }
}
