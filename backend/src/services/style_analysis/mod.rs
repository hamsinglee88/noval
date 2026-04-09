/// 风格分析模块
///
/// 包含词汇层、句式层等特征分析功能

pub mod vocabulary_analysis;
pub mod sentence_analysis;
pub mod style_analyzer;

pub use vocabulary_analysis::{extract_vocabulary_features, extract_vocabulary_features_chunked, VocabularyAnalysisResult};
pub use sentence_analysis::{extract_sentence_features, extract_sentence_features_chunked, SentenceAnalysisResult};
pub use style_analyzer::{StyleAnalyzer, AnalyzerConfig};
