/// 词汇层特征分析
///
/// 使用 jieba-rs 进行中文分词和词性标注，提取词汇特征

use jieba_rs::Jieba;
use std::collections::{HashMap, HashSet};

/// 词汇层分析结果
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VocabularyAnalysisResult {
    /// 常用形容词列表 (词，频率)
    pub common_adjectives: Vec<(String, u32)>,
    /// 常用动词列表 (词，频率)
    pub common_verbs: Vec<(String, u32)>,
    /// 常用副词列表 (词，频率)
    pub common_adverbs: Vec<(String, u32)>,
    /// Type-Token Ratio (词汇丰富度)
    pub ttr: f32,
    /// 修正 TTR (Root TTR)
    pub root_ttr: f32,
    /// 总词数
    pub total_words: usize,
    /// 唯一词数
    pub unique_words: usize,
}

/// 提取词汇层特征
pub fn extract_vocabulary_features(text: &str) -> VocabularyAnalysisResult {
    let jieba = Jieba::new();

    // 分词
    let words: Vec<&str> = jieba.cut(text, false).into_iter().collect();

    // 词性标注和分类
    let mut adj_freq: HashMap<String, u32> = HashMap::new();
    let mut verb_freq: HashMap<String, u32> = HashMap::new();
    let mut adv_freq: HashMap<String, u32> = HashMap::new();

    for word in &words {
        let pos_tags = jieba.tag(word, true);
        let pos = pos_tags.first().map(|t| t.tag.as_ref()).unwrap_or("unknown");

        match pos {
            // 形容词词性：a, ad, an
            "a" | "ad" | "an" => {
                *adj_freq.entry(word.to_string()).or_insert(0) += 1;
            }
            // 动词词性：v, vd, vi, vl, vx
            "v" | "vd" | "vi" | "vl" | "vx" => {
                *verb_freq.entry(word.to_string()).or_insert(0) += 1;
            }
            // 副词词性：d
            "d" => {
                *adv_freq.entry(word.to_string()).or_insert(0) += 1;
            }
            _ => {}
        }
    }

    // 计算 TTR
    let unique_words: HashSet<_> = words.iter().collect();
    let total = words.len().max(1);
    let unique = unique_words.len().max(1);

    let ttr = unique as f32 / total as f32;
    let root_ttr = (unique as f32).sqrt() / (total as f32).sqrt();

    // 提取 Top 50
    let common_adjectives = get_top_n(&adj_freq, 50);
    let common_verbs = get_top_n(&verb_freq, 50);
    let common_adverbs = get_top_n(&adv_freq, 50);

    VocabularyAnalysisResult {
        common_adjectives,
        common_verbs,
        common_adverbs,
        ttr,
        root_ttr,
        total_words: words.len(),
        unique_words: unique_words.len(),
    }
}

/// 获取频率最高的 N 个词
fn get_top_n(freq_map: &HashMap<String, u32>, n: usize) -> Vec<(String, u32)> {
    let mut items: Vec<_> = freq_map.iter()
        .map(|(k, v)| (k.clone(), *v))
        .collect();

    // 按频率降序排序
    items.sort_by(|a, b| b.1.cmp(&a.1));

    items.into_iter().take(n).collect()
}

/// 分块处理大文本，避免内存溢出
pub fn extract_vocabulary_features_chunked(text: &str, _chunk_size: usize) -> VocabularyAnalysisResult {
    let chunks: Vec<&str> = text
        .split('\n')
        .collect();

    let mut all_adj_freq: HashMap<String, u32> = HashMap::new();
    let mut all_verb_freq: HashMap<String, u32> = HashMap::new();
    let mut all_adv_freq: HashMap<String, u32> = HashMap::new();
    let mut total_words = 0;
    let mut unique_words_set: HashSet<String> = HashSet::new();

    let jieba = Jieba::new();

    for chunk in chunks {
        if chunk.trim().is_empty() {
            continue;
        }

        let words: Vec<&str> = jieba.cut(chunk, false).into_iter().collect();
        total_words += words.len();

        for word in &words {
            unique_words_set.insert(word.to_string());

            let pos_tags = jieba.tag(word, true);
            let pos = pos_tags.first().map(|t| t.tag.as_ref()).unwrap_or("unknown");

            match pos {
                "a" | "ad" | "an" => {
                    *all_adj_freq.entry(word.to_string()).or_insert(0) += 1;
                }
                "v" | "vd" | "vi" | "vl" | "vx" => {
                    *all_verb_freq.entry(word.to_string()).or_insert(0) += 1;
                }
                "d" => {
                    *all_adv_freq.entry(word.to_string()).or_insert(0) += 1;
                }
                _ => {}
            }
        }
    }

    let total = total_words.max(1);
    let unique = unique_words_set.len().max(1);
    let ttr = unique as f32 / total as f32;
    let root_ttr = (unique as f32).sqrt() / (total as f32).sqrt();

    VocabularyAnalysisResult {
        common_adjectives: get_top_n(&all_adj_freq, 50),
        common_verbs: get_top_n(&all_verb_freq, 50),
        common_adverbs: get_top_n(&all_adv_freq, 50),
        ttr,
        root_ttr,
        total_words,
        unique_words: unique,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_common_adjectives() {
        let text = "美丽的花园里开满了鲜艳的花朵，芬芳的香气扑鼻而来";
        let result = extract_vocabulary_features(text);
        assert!(!result.common_adjectives.is_empty());
    }

    #[test]
    fn test_ttr_calculation() {
        let text = "重复重复重复不重复";
        let result = extract_vocabulary_features(text);
        assert!(result.ttr > 0.0 && result.ttr <= 1.0);
    }

    #[test]
    fn test_empty_text() {
        let text = "";
        let result = extract_vocabulary_features(text);
        assert_eq!(result.total_words, 0);
        assert_eq!(result.unique_words, 0);
    }

    #[test]
    fn test_chunked_processing() {
        let text = "第一行文本\n第二行文本\n第三行文本";
        let result = extract_vocabulary_features_chunked(text, 100);
        assert!(result.total_words > 0);
    }
}
