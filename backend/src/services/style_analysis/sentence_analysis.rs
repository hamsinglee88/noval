/// 句式层特征分析
///
/// 基于正则表达式进行中文分句，统计句式特征

use regex::Regex;

/// 句式层分析结果
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SentenceAnalysisResult {
    /// 平均句长（字符数）
    pub avg_sentence_length: f32,
    /// 句长标准差
    pub sentence_length_variance: f32,
    /// 短句比例（< 10 字）
    pub short_sentence_ratio: f32,
    /// 中句比例（10-30 字）
    pub medium_sentence_ratio: f32,
    /// 长句比例（> 30 字）
    pub long_sentence_ratio: f32,
    /// 复合句比例（包含连接词）
    pub complex_sentence_ratio: f32,
    /// 问句比例
    pub question_ratio: f32,
    /// 感叹句比例
    pub exclamation_ratio: f32,
    /// 段落平均句数
    pub paragraph_avg_sentences: f32,
}

/// 提取句式层特征
pub fn extract_sentence_features(text: &str) -> SentenceAnalysisResult {
    // 使用正则匹配句子（排除句末标点）
    // 注意：中文 fullwidth 标点 (？！。) 和英文标点都要处理
    let sentence_pattern = Regex::new(r"[^.!?!.?\n]+").unwrap();
    let sentences: Vec<&str> = sentence_pattern
        .find_iter(text)
        .map(|m| m.as_str())
        .filter(|s| !s.trim().is_empty())
        .collect();

    // 句长统计
    let sentence_lengths: Vec<usize> = sentences
        .iter()
        .map(|s| s.chars().count())
        .collect();

    let avg_length = mean(&sentence_lengths);
    let variance = calc_variance(&sentence_lengths);

    // 句式比例
    let total = sentences.len() as f32;
    if total == 0.0 {
        return SentenceAnalysisResult {
            avg_sentence_length: 0.0,
            sentence_length_variance: 0.0,
            short_sentence_ratio: 0.0,
            medium_sentence_ratio: 0.0,
            long_sentence_ratio: 0.0,
            complex_sentence_ratio: 0.0,
            question_ratio: 0.0,
            exclamation_ratio: 0.0,
            paragraph_avg_sentences: 0.0,
        };
    }

    let short_count = sentence_lengths.iter().filter(|&&l| l < 10).count();
    let medium_count = sentence_lengths.iter().filter(|&&l| l >= 10 && l <= 30).count();
    let long_count = sentence_lengths.iter().filter(|&&l| l > 30).count();

    // 问句/感叹句 - 统计原文中问号和感叹号的数量（每个标点代表一个句子类型）
    // 同时检测中文 fullwidth 和英文标点
    let question_mark_count = text.chars().filter(|&c| c == '?' || c == '?').count();
    let exclamation_mark_count = text.chars().filter(|&c| c == '!' || c == '!').count();

    // 复合句（简单检测连接词）
    let conjunctions = ["因为", "所以", "虽然", "但是", "如果", "即使", "尽管", "然而", "可是", "因此"];
    let complex_count = sentences.iter()
        .filter(|s| conjunctions.iter().any(|c| s.contains(c)))
        .count();

    // 段落平均句数
    let paragraph_avg = calculate_paragraph_avg(text);

    SentenceAnalysisResult {
        avg_sentence_length: avg_length,
        sentence_length_variance: variance,
        short_sentence_ratio: short_count as f32 / total,
        medium_sentence_ratio: medium_count as f32 / total,
        long_sentence_ratio: long_count as f32 / total,
        complex_sentence_ratio: complex_count as f32 / total,
        question_ratio: question_mark_count as f32 / total,
        exclamation_ratio: exclamation_mark_count as f32 / total,
        paragraph_avg_sentences: paragraph_avg,
    }
}

/// 计算平均值
fn mean(values: &[usize]) -> f32 {
    if values.is_empty() {
        return 0.0;
    }
    values.iter().sum::<usize>() as f32 / values.len() as f32
}

/// 计算方差
fn calc_variance(values: &[usize]) -> f32 {
    if values.is_empty() {
        return 0.0;
    }
    let m = mean(values);
    values.iter().map(|&v| (v as f32 - m).powi(2)).sum::<f32>() / values.len() as f32
}

/// 计算段落平均句数
fn calculate_paragraph_avg(text: &str) -> f32 {
    let paragraphs: Vec<&str> = text
        .split('\n')
        .filter(|p| !p.trim().is_empty())
        .collect();

    if paragraphs.is_empty() {
        return 0.0;
    }

    let sentence_pattern = Regex::new(r"[^.!?!\n]+").unwrap();
    let mut total_sentences = 0;

    for paragraph in &paragraphs {
        let count = sentence_pattern
            .find_iter(paragraph)
            .map(|m| m.as_str())
            .filter(|s| !s.trim().is_empty())
            .count();
        total_sentences += count;
    }

    total_sentences as f32 / paragraphs.len() as f32
}

/// 分块处理大文本
pub fn extract_sentence_features_chunked(text: &str, _chunk_size: usize) -> SentenceAnalysisResult {
    // 对于句式分析，整体处理更准确，因为分句可能跨块
    // 这里直接委托给主函数
    extract_sentence_features(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_avg_sentence_length() {
        let text = "今天天气很好。我们去公园玩。";
        let result = extract_sentence_features(text);
        assert!(result.avg_sentence_length > 0.0);
    }

    #[test]
    fn test_sentence_type_ratio() {
        // 使用 ASCII 标点符号测试
        let text = "Is it good? Really! Ok.";
        let result = extract_sentence_features(text);
        assert!(result.question_ratio > 0.0);
        assert!(result.exclamation_ratio > 0.0);
    }

    #[test]
    fn test_empty_text() {
        let text = "";
        let result = extract_sentence_features(text);
        assert_eq!(result.avg_sentence_length, 0.0);
        assert_eq!(result.short_sentence_ratio, 0.0);
    }

    #[test]
    fn test_complex_sentence_detection() {
        let text = "因为下雨，所以我们取消了计划。但是如果天气好转，我们还是会去的。";
        let result = extract_sentence_features(text);
        assert!(result.complex_sentence_ratio > 0.0);
    }

    #[test]
    fn test_paragraph_avg() {
        let text = "第一段。句子一。句子二。\n\n第二段。句子三。";
        let result = extract_sentence_features(text);
        assert!(result.paragraph_avg_sentences > 0.0);
    }
}
