/// 风格分析模块
///
/// 包含词汇层、句式层、修辞层、叙事层、情感层、节奏层等特征分析功能

pub mod vocabulary_analysis;
pub mod sentence_analysis;
pub mod rhetoric_analysis;
pub mod narrative_analysis;
pub mod emotion_analysis;
pub mod pacing_analysis;
pub mod style_analyzer;

pub use vocabulary_analysis::{extract_vocabulary_features, extract_vocabulary_features_chunked, VocabularyAnalysisResult};
pub use sentence_analysis::{extract_sentence_features, extract_sentence_features_chunked, SentenceAnalysisResult};
pub use rhetoric_analysis::{extract_rhetoric_features, extract_rhetoric_features_chunked, RhetoricAnalysisResult};
pub use narrative_analysis::{extract_narrative_features, extract_narrative_features_chunked, NarrativeAnalysisResult};
pub use emotion_analysis::{extract_emotion_features, extract_emotion_features_chunked, EmotionAnalysisResult};
pub use pacing_analysis::{extract_pacing_features, extract_pacing_features_chunked, PacingAnalysisResult};
pub use style_analyzer::{StyleAnalyzer, AnalyzerConfig};
