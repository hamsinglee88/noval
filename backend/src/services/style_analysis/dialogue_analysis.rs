/// 对话层特征分析
///
/// 提取对话比例、角色声音区分度、对话标签使用习惯等特征

use regex::Regex;
use std::collections::HashMap;

/// 对话层分析结果
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DialogueAnalysisResult {
    /// 对话内容比例 (0-1)
    pub dialogue_ratio: f32,
    /// 角色声音区分度 (0-1)
    pub character_voice_distinction: f32,
    /// 对话标签频率（每千字）
    pub dialogue_tag_frequency: f32,
    /// 无标签对话比例 (0-1)
    pub untagged_dialogue_ratio: f32,
    /// 副词修饰对话标签比例 (0-1)
    pub adverb_modifier_ratio: f32,
    /// 平均对话长度（字）
    pub avg_dialogue_length: f32,
}

/// 匹配中文引号内的内容
fn get_chinese_quote_pattern() -> Regex {
    Regex::new(r"[\u300c\u300d][^\u300c\u300d]*[\u300c\u300d]|[\u201c\u201d][^\u201c\u201d]*[\u201c\u201d]|[\u2018\u2019][^\u2018\u2019]*[\u2018\u2019]").unwrap()
}

/// 匹配冒号后的对话（如：他道："..."）
fn get_colon_dialogue_pattern() -> Regex {
    Regex::new(r"[：:][\s]*[\u300c\u300d]+[^\u300c\u300d]+[\u300c\u300d]|[\u201c\u201d][^\u201c\u201d]+[\u201c\u201d]").unwrap()
}

/// 统计对话内容比例
pub fn calculate_dialogue_ratio(text: &str) -> f32 {
    let total_chars = text.chars().count() as f32;
    if total_chars == 0.0 {
        return 0.0;
    }

    let chinese_quote_pattern = get_chinese_quote_pattern();
    let colon_dialogue_pattern = get_colon_dialogue_pattern();

    let quoted_chars: usize = chinese_quote_pattern
        .find_iter(text)
        .map(|m| m.as_str().chars().count())
        .sum();

    let colon_dialogue_chars: usize = colon_dialogue_pattern
        .find_iter(text)
        .map(|m| m.as_str().chars().count())
        .sum();

    let dialogue_chars = quoted_chars.max(colon_dialogue_chars);
    dialogue_chars as f32 / total_chars
}

/// 提取角色和对应的对话
fn extract_character_dialogues(text: &str) -> Vec<(String, String)> {
    let pattern = Regex::new(r"([^\s]{1,4})[道说问喊叫笑道][：:][\s]*[\u300c\u300d\u201c\u201d]?([^\u300c\u300d\u201c\u201d]+)[\u300c\u300d\u201c\u201d]?").unwrap();
    let mut dialogues: HashMap<String, String> = HashMap::new();

    for cap in pattern.captures_iter(text) {
        if let (Some(name), Some(dialogue)) = (cap.get(1), cap.get(2)) {
            let name_str = name.as_str().to_string();
            let dialogue_str = dialogue.as_str().to_string();
            dialogues.entry(name_str).or_insert_with(String::new).push_str(&dialogue_str);
        }
    }

    dialogues.into_iter().collect()
}

/// 提取词频
fn extract_word_frequency(text: &str) -> HashMap<String, usize> {
    let mut freq: HashMap<String, usize> = HashMap::new();
    for word in text.split(|c: char| c.is_whitespace() || matches!(c, ',' | '.' | '!' | '?' | ';' | ':')) {
        if word.len() > 1 {
            *freq.entry(word.to_string()).or_insert(0) += 1;
        }
    }
    freq
}

/// 计算余弦相似度
fn cosine_similarity(freq1: &HashMap<String, usize>, freq2: &HashMap<String, usize>) -> f32 {
    let mut dot_product = 0.0;
    let mut norm1 = 0.0;
    let mut norm2 = 0.0;

    let all_words: std::collections::HashSet<_> = freq1.keys().chain(freq2.keys()).collect();

    for word in all_words {
        let v1 = *freq1.get(word).unwrap_or(&0) as f32;
        let v2 = *freq2.get(word).unwrap_or(&0) as f32;
        dot_product += v1 * v2;
        norm1 += v1 * v1;
        norm2 += v2 * v2;
    }

    if norm1 == 0.0 || norm2 == 0.0 {
        return 0.0;
    }

    dot_product / (norm1.sqrt() * norm2.sqrt())
}

/// 分析角色声音区分度
fn analyze_character_voice_distinction(text: &str) -> f32 {
    let character_dialogues = extract_character_dialogues(text);
    if character_dialogues.len() < 2 {
        return 0.5;
    }

    let mut voice_profiles: Vec<HashMap<String, usize>> = Vec::new();
    for (_character, dialogue) in &character_dialogues {
        let word_freq = extract_word_frequency(dialogue);
        voice_profiles.push(word_freq);
    }

    let mut total_distinction = 0.0;
    let comparison_count = voice_profiles.len() * (voice_profiles.len() - 1) / 2;

    for i in 0..voice_profiles.len() {
        for j in (i + 1)..voice_profiles.len() {
            let similarity = cosine_similarity(&voice_profiles[i], &voice_profiles[j]);
            total_distinction += 1.0 - similarity;
        }
    }

    if comparison_count > 0 {
        total_distinction / comparison_count as f32
    } else {
        0.5
    }
}

fn count_keyword_occurrences(text: &str, keywords: &[&str]) -> usize {
    keywords.iter().map(|&kw| text.matches(kw).count()).sum()
}

/// 分析对话标签使用习惯
fn analyze_dialogue_tags(text: &str) -> (f32, f32, f32) {
    let char_count = text.chars().count() as f32 / 1000.0;
    let tag_words = ["道", "说", "问", "喊", "叫", "笑", "哭", "叹", "喝", "答", "回", "应"];

    let tag_count = count_keyword_occurrences(text, &tag_words);
    let tag_frequency = if char_count > 0.0 {
        tag_count as f32 / char_count
    } else {
        0.0
    };

    let total_dialogues = Regex::new(r"[\u300c\u300d][^\u300c\u300d]+[\u300c\u300d]|[\u201c\u201d][^\u201c\u201d]+[\u201c\u201d]").unwrap()
        .find_iter(text).count();

    let tagged_dialogues = Regex::new(r"[道说问喊叫笑道][：:][\s]*[\u300c\u300d\u201c\u201d]").unwrap()
        .find_iter(text).count();

    let untagged_ratio = if total_dialogues > 0 {
        (total_dialogues - tagged_dialogues) as f32 / total_dialogues as f32
    } else {
        0.0
    };

    let adverb_pattern = Regex::new(r"[地][说问喊叫道笑]").unwrap();
    let adverb_count = adverb_pattern.find_iter(text).count();
    let adverb_ratio = if tag_count > 0 {
        adverb_count as f32 / tag_count as f32
    } else {
        0.0
    };

    (tag_frequency, untagged_ratio, adverb_ratio)
}

/// 计算平均对话长度
fn calculate_avg_dialogue_length(text: &str) -> f32 {
    let dialogue_pattern = Regex::new(r"[\u300c\u300d]([^\u300c\u300d]+)[\u300c\u300d]|[\u201c\u201d]([^\u201c\u201d]+)[\u201c\u201d]").unwrap();
    let dialogues: Vec<&str> = dialogue_pattern
        .captures_iter(text)
        .filter_map(|cap| cap.get(1).or_else(|| cap.get(2)).map(|m| m.as_str()))
        .collect();

    if dialogues.is_empty() {
        return 0.0;
    }

    dialogues.iter().map(|d| d.chars().count()).sum::<usize>() as f32 / dialogues.len() as f32
}

/// 提取对话层特征
pub fn extract_dialogue_features(text: &str) -> DialogueAnalysisResult {
    let dialogue_ratio = calculate_dialogue_ratio(text);
    let voice_distinction = analyze_character_voice_distinction(text);
    let (tag_freq, untagged_ratio, adverb_ratio) = analyze_dialogue_tags(text);
    let avg_dialogue_length = calculate_avg_dialogue_length(text);

    DialogueAnalysisResult {
        dialogue_ratio,
        character_voice_distinction: voice_distinction,
        dialogue_tag_frequency: tag_freq,
        untagged_dialogue_ratio: untagged_ratio,
        adverb_modifier_ratio: adverb_ratio,
        avg_dialogue_length,
    }
}

pub fn extract_dialogue_features_chunked(text: &str, _chunk_size: usize) -> DialogueAnalysisResult {
    extract_dialogue_features(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dialogue_ratio_calculation() {
        let text = "他道：「你好。」她回答：「好久不见。」";
        let ratio = calculate_dialogue_ratio(text);
        assert!(ratio > 0.0 && ratio < 1.0);
    }

    #[test]
    fn test_character_voice_distinction() {
        let text = "张三：「咱们走吧。」李四：「我再待会儿。」";
        let distinction = analyze_character_voice_distinction(text);
        assert!(distinction >= 0.0 && distinction <= 1.0);
    }

    #[test]
    fn test_dialogue_tag_frequency() {
        let text = "他道：'...'。她说：'...'。他们问：'...'。";
        let (tag_freq, _, _) = analyze_dialogue_tags(text);
        assert!(tag_freq > 0.0);
    }

    #[test]
    fn test_untagged_dialogue_ratio() {
        let text = "「你好」，他说。「再见」，她回答。";
        let (_, untagged_ratio, _) = analyze_dialogue_tags(text);
        assert!(untagged_ratio >= 0.0 && untagged_ratio <= 1.0);
    }

    #[test]
    fn test_adverb_modifier_ratio() {
        let text = "他冷冷地说：「走吧。」她兴奋地问：「真的吗？」";
        let (_, _, adverb_ratio) = analyze_dialogue_tags(text);
        assert!(adverb_ratio >= 0.0 && adverb_ratio <= 1.0);
    }

    #[test]
    fn test_avg_dialogue_length() {
        let text = "他道：「你好吗？」她回答：「我很好，谢谢。」";
        let avg_len = calculate_avg_dialogue_length(text);
        assert!(avg_len > 0.0);
    }

    #[test]
    fn test_empty_text() {
        let text = "";
        let result = extract_dialogue_features(text);
        assert_eq!(result.dialogue_ratio, 0.0);
        assert_eq!(result.avg_dialogue_length, 0.0);
    }

    #[test]
    fn test_extract_dialogue_features_complete() {
        let text = "张三：「咱们走吧。」李四：「我再待会儿。」张三：「好吧，那我先走了。」";
        let result = extract_dialogue_features(text);
        assert!(result.dialogue_ratio > 0.0);
        assert!(result.character_voice_distinction >= 0.0 && result.character_voice_distinction <= 1.0);
        assert!(result.avg_dialogue_length > 0.0);
    }
}
