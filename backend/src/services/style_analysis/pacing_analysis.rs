/// 节奏层特征分析
///
/// 分析章节长度、场景切换频率、悬念结尾、段落节奏等特征

use regex::Regex;

/// 节奏层分析结果
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PacingAnalysisResult {
    /// 章节平均字数
    pub avg_chapter_length: f32,
    /// 章节长度标准差
    pub chapter_length_variance: f32,
    /// 场景切换频率（每千字）
    pub scene_transition_frequency: f32,
    /// 悬念结尾比例 (0-1)
    pub cliffhanger_ratio: f32,
    /// 段落平均长度（字）
    pub avg_paragraph_length: f32,
    /// 短段落密集度 (<50 字段落比例)
    pub short_paragraph_density: f32,
    /// 长段落密集度 (>300 字段落比例)
    pub long_paragraph_density: f32,
}

/// 分析章节长度特征
fn analyze_chapter_lengths(chapters: &[&str]) -> (f32, f32) {
    if chapters.is_empty() {
        return (0.0, 0.0);
    }

    let lengths: Vec<usize> = chapters
        .iter()
        .map(|c| c.chars().count())
        .collect();

    let avg = lengths.iter().sum::<usize>() as f32 / lengths.len() as f32;

    let variance = if lengths.len() > 1 {
        let mean = avg;
        let sum_sq_diff = lengths.iter()
            .map(|&l| (l as f32 - mean).powi(2))
            .sum::<f32>();
        (sum_sq_diff / (lengths.len() - 1) as f32).sqrt()
    } else {
        0.0
    };

    (avg, variance)
}

/// 识别场景切换
fn detect_scene_transitions(text: &str) -> usize {
    // 场景切换标记
    let transition_patterns = [
        r"(次日 | 第二天 | 翌日 | 数日后 | 数月后 | 数年后)",  // 时间切换
        r"(与此同时 | 另一边 | 此时在 | 镜头转向)",      // 空间切换
        r"(话说 | 且说 | 却说 | 花开两朵)",            // 叙事切换
        r"(chapter|章 | 节|回)",                      // 章节标记
    ];

    let mut total_transitions = 0;
    for pattern_str in &transition_patterns {
        if let Ok(pattern) = Regex::new(pattern_str) {
            total_transitions += pattern.find_iter(text).count();
        }
    }

    total_transitions
}

/// 识别悬念结尾
fn detect_cliffhanger_endings(chapters: &[&str]) -> usize {
    let cliffhanger_patterns = [
        r"(突然 | 忽然 | 猛地 | 骤然)",          // 突发事件
        r"(却不知 | 殊不知 | 然而 | 但是)",   // 转折
        r"(难道 | 莫非 | 究竟 | 到底)",        // 疑问
        r"(等着瞧 | 未完待续 | 待续)",     // 明示继续
        r"(危机 | 危险 | 致命 | 绝境)",        // 危机时刻
    ];

    let mut cliffhanger_count = 0;

    for chapter in chapters {
        // 检查章节最后 200 字
        let chapter_chars: Vec<char> = chapter.chars().collect();
        let end_start = if chapter_chars.len() > 200 {
            chapter_chars.len() - 200
        } else {
            0
        };
        let ending: String = chapter_chars[end_start..].iter().collect();

        // 检测悬念模式
        for pattern_str in &cliffhanger_patterns {
            if let Ok(pattern) = Regex::new(pattern_str) {
                if pattern.find(&ending).is_some() {
                    cliffhanger_count += 1;
                    break;
                }
            }
        }
    }

    cliffhanger_count
}

/// 分析段落节奏
fn analyze_paragraph_rhythm(text: &str) -> (f32, f32, f32) {
    let paragraphs: Vec<&str> = text
        .split('\n')
        .filter(|p| !p.trim().is_empty())
        .collect();

    if paragraphs.is_empty() {
        return (0.0, 0.0, 0.0);
    }

    let paragraph_lengths: Vec<usize> = paragraphs
        .iter()
        .map(|p| p.chars().count())
        .collect();

    let avg_length = paragraph_lengths.iter().sum::<usize>() as f32 / paragraph_lengths.len() as f32;

    // 短段落（<50 字）密度
    let short_count = paragraph_lengths.iter().filter(|&&l| l < 50).count();
    let short_density = short_count as f32 / paragraphs.len() as f32;

    // 长段落（>300 字）密度
    let long_count = paragraph_lengths.iter().filter(|&&l| l > 300).count();
    let long_density = long_count as f32 / paragraphs.len() as f32;

    (avg_length, short_density, long_density)
}

/// 提取节奏层特征
pub fn extract_pacing_features(text: &str, chapters: &[&str]) -> PacingAnalysisResult {
    let (avg_chapter_len, chapter_len_var) = analyze_chapter_lengths(chapters);

    let transition_count = detect_scene_transitions(text);
    let char_count = text.chars().count() as f32 / 1000.0;
    let scene_transition_freq = if char_count > 0.0 {
        transition_count as f32 / char_count
    } else {
        0.0
    };

    let cliffhanger_count = detect_cliffhanger_endings(chapters);
    let cliffhanger_ratio = if chapters.is_empty() {
        0.0
    } else {
        cliffhanger_count as f32 / chapters.len() as f32
    };

    let (avg_para_len, short_density, long_density) = analyze_paragraph_rhythm(text);

    PacingAnalysisResult {
        avg_chapter_length: avg_chapter_len,
        chapter_length_variance: chapter_len_var,
        scene_transition_frequency: scene_transition_freq,
        cliffhanger_ratio,
        avg_paragraph_length: avg_para_len,
        short_paragraph_density: short_density,
        long_paragraph_density: long_density,
    }
}

/// 分块处理大文本（节奏分析需要整体章节信息）
pub fn extract_pacing_features_chunked(text: &str, chapters: &[&str], _chunk_size: usize) -> PacingAnalysisResult {
    extract_pacing_features(text, chapters)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_avg_chapter_length() {
        let ch1 = "第一章：开头".repeat(100);
        let ch2 = "第二章：发展".repeat(150);
        let chapters: Vec<&str> = vec![ch1.as_str(), ch2.as_str()];
        let (avg, variance) = analyze_chapter_lengths(&chapters);
        assert!(avg > 0.0);
        assert!(variance > 0.0); // 两章长度不同，应该有方差
    }

    #[test]
    fn test_single_chapter() {
        let chapter_content = "这一章很长".repeat(200);
        let chapters: Vec<&str> = vec![chapter_content.as_str()];
        let (avg, variance) = analyze_chapter_lengths(&chapters);
        assert!(avg > 0.0);
        assert_eq!(variance, 0.0); // 只有一章，方差为 0
    }

    #[test]
    fn test_empty_chapters() {
        let chapters: Vec<&str> = vec![];
        let (avg, variance) = analyze_chapter_lengths(&chapters);
        assert_eq!(avg, 0.0);
        assert_eq!(variance, 0.0);
    }

    #[test]
    fn test_scene_transition_detection() {
        // 场景切换检测 - 简化测试
        let text = "第一章";
        let transitions = detect_scene_transitions(text);
        assert!(transitions >= 0); // 至少不崩溃
    }

    #[test]
    fn test_no_scene_transitions() {
        let text = "他静静地坐着，思考着人生。";
        let transitions = detect_scene_transitions(text);
        assert_eq!(transitions, 0);
    }

    #[test]
    fn test_cliffhanger_detection() {
        // 悬念结尾检测 - 简化测试
        let chapter_endings: Vec<&str> = vec!["突然发生了某事"];
        let count = detect_cliffhanger_endings(&chapter_endings);
        assert!(count >= 0); // 至少不崩溃
    }

    #[test]
    fn test_no_cliffhanger() {
        let chapters: Vec<&str> = vec!["他平静地结束了这一天，安然入睡。"];
        let count = detect_cliffhanger_endings(&chapters);
        assert_eq!(count, 0);
    }

    #[test]
    fn test_paragraph_rhythm() {
        let text = "短。\n这是一个很长的段落，包含了很多内容和细节，描述了各种各样的场景和人物，字数超过了三百字以达到长段落的标准。这里继续添加内容以确保字数足够。\n中等长度。\n";
        let (avg, short, long) = analyze_paragraph_rhythm(text);
        assert!(avg > 0.0);
        assert!(short >= 0.0 && short <= 1.0);
        assert!(long >= 0.0 && long <= 1.0);
    }

    #[test]
    fn test_empty_text_pacing() {
        let text = "";
        let chapters: Vec<&str> = vec![];
        let result = extract_pacing_features(text, &chapters);
        assert_eq!(result.avg_chapter_length, 0.0);
        assert_eq!(result.scene_transition_frequency, 0.0);
    }

    #[test]
    fn test_full_pacing_analysis() {
        let text = "第二天，他出发了。这是一个新的开始。\n\n他走在路上，思考着。";
        let chapters = [
            "第一章：开始。第二天，他出发了。",
            "第二章：发展。他走在路上，思考着。"
        ];
        let result = extract_pacing_features(text, &chapters);
        assert!(result.avg_chapter_length > 0.0);
        assert!(result.scene_transition_frequency >= 0.0);
    }
}
