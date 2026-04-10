/// 风格向量化模块
///
/// 将七层风格特征转换为 128 维向量

use serde::{Deserialize, Serialize};

use super::{
    VocabularyAnalysisResult, SentenceAnalysisResult, RhetoricAnalysisResult,
    NarrativeAnalysisResult, EmotionAnalysisResult, PacingAnalysisResult,
    DialogueAnalysisResult, DescriptionAnalysisResult,
};

/// 128 维风格向量
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleVector {
    pub vocabulary_dims: Vec<f32>,  // 0-15 (16 维)
    pub sentence_dims: Vec<f32>,    // 16-31 (16 维)
    pub rhetoric_dims: Vec<f32>,    // 32-47 (16 维)
    pub narrative_dims: Vec<f32>,   // 48-71 (24 维)
    pub emotion_dims: Vec<f32>,     // 72-87 (16 维)
    pub pacing_dims: Vec<f32>,      // 88-103 (16 维)
    pub dialogue_dims: Vec<f32>,    // 104-115 (12 维)
    pub description_dims: Vec<f32>, // 116-127 (12 维)
}

impl StyleVector {
    /// 合并为完整的 128 维向量
    pub fn to_full_vector(&self) -> Vec<f32> {
        let mut full = Vec::with_capacity(128);
        full.extend_from_slice(&self.vocabulary_dims);
        full.extend_from_slice(&self.sentence_dims);
        full.extend_from_slice(&self.rhetoric_dims);
        full.extend_from_slice(&self.narrative_dims);
        full.extend_from_slice(&self.emotion_dims);
        full.extend_from_slice(&self.pacing_dims);
        full.extend_from_slice(&self.dialogue_dims);
        full.extend_from_slice(&self.description_dims);
        full
    }
    
    /// 计算两个风格向量的余弦相似度
    pub fn cosine_similarity(&self, other: &StyleVector) -> f32 {
        let v1 = self.to_full_vector();
        let v2 = other.to_full_vector();
        
        let dot: f32 = v1.iter().zip(v2.iter()).map(|(a, b)| a * b).sum();
        let norm1: f32 = v1.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm2: f32 = v2.iter().map(|x| x * x).sum::<f32>().sqrt();
        
        if norm1 == 0.0 || norm2 == 0.0 {
            return 0.0;
        }
        
        dot / (norm1 * norm2)
    }
}

/// 向量化器配置
#[derive(Debug, Clone)]
pub struct StyleWeights {
    pub vocabulary_weight: f32,  // 0.15
    pub sentence_weight: f32,    // 0.15
    pub rhetoric_weight: f32,    // 0.10
    pub narrative_weight: f32,   // 0.20
    pub emotion_weight: f32,     // 0.15
    pub pacing_weight: f32,      // 0.10
    pub dialogue_weight: f32,    // 0.10
    pub description_weight: f32, // 0.05
}

impl Default for StyleWeights {
    fn default() -> Self {
        Self {
            vocabulary_weight: 0.15,
            sentence_weight: 0.15,
            rhetoric_weight: 0.10,
            narrative_weight: 0.20,
            emotion_weight: 0.15,
            pacing_weight: 0.10,
            dialogue_weight: 0.10,
            description_weight: 0.05,
        }
    }
}

/// 风格向量化器
pub struct StyleVectorizer {
    #[allow(dead_code)]
    weights: StyleWeights,
}

impl StyleVectorizer {
    pub fn new() -> Self {
        Self {
            weights: StyleWeights::default(),
        }
    }
    
    /// 将七层分析结果转换为风格向量
    pub fn vectorize(
        &self,
        vocab: &VocabularyAnalysisResult,
        sentence: &SentenceAnalysisResult,
        rhetoric: &RhetoricAnalysisResult,
        narrative: &NarrativeAnalysisResult,
        emotion: &EmotionAnalysisResult,
        pacing: &PacingAnalysisResult,
        dialogue: &DialogueAnalysisResult,
        description: &DescriptionAnalysisResult,
    ) -> StyleVector {
        StyleVector {
            vocabulary_dims: self.vectorize_vocabulary(vocab),
            sentence_dims: self.vectorize_sentence(sentence),
            rhetoric_dims: self.vectorize_rhetoric(rhetoric),
            narrative_dims: self.vectorize_narrative(narrative),
            emotion_dims: self.vectorize_emotion(emotion),
            pacing_dims: self.vectorize_pacing(pacing),
            dialogue_dims: self.vectorize_dialogue(dialogue),
            description_dims: self.vectorize_description(description),
        }
    }
    
    /// 词汇层向量化（16 维）
    fn vectorize_vocabulary(&self, vocab: &VocabularyAnalysisResult) -> Vec<f32> {
        let mut dims = vec![0.0; 16];
        
        // 0-1: TTR 相关
        dims[0] = normalize(vocab.ttr, 0.0, 1.0);
        dims[1] = normalize(vocab.root_ttr, 0.0, 1.0);
        
        // 2-4: 词频分布（简化：用 top 词频归一化）
        dims[2] = normalize_word_freq(&vocab.common_adjectives);
        dims[3] = normalize_word_freq(&vocab.common_verbs);
        dims[4] = normalize_word_freq(&vocab.common_adverbs);
        
        // 5-6: 词汇量特征
        dims[5] = normalize(vocab.total_words as f32 / 10000.0, 0.0, 10.0);
        dims[6] = normalize(vocab.unique_words as f32 / 5000.0, 0.0, 5.0);
        
        dims
    }
    
    /// 句式层向量化（16 维）
    fn vectorize_sentence(&self, sentence: &SentenceAnalysisResult) -> Vec<f32> {
        let mut dims = vec![0.0; 16];
        
        // 0-1: 句长特征
        dims[0] = normalize(sentence.avg_sentence_length, 0.0, 50.0);
        dims[1] = normalize(sentence.sentence_length_variance, 0.0, 20.0);
        
        // 2-5: 句式比例
        dims[2] = normalize(sentence.short_sentence_ratio, 0.0, 1.0);
        dims[3] = normalize(sentence.medium_sentence_ratio, 0.0, 1.0);
        dims[4] = normalize(sentence.long_sentence_ratio, 0.0, 1.0);
        dims[5] = normalize(sentence.complex_sentence_ratio, 0.0, 1.0);
        
        // 6-7: 特殊句式
        dims[6] = normalize(sentence.question_ratio, 0.0, 0.5);
        dims[7] = normalize(sentence.exclamation_ratio, 0.0, 0.5);
        
        dims
    }
    
    /// 修辞层向量化（16 维）
    fn vectorize_rhetoric(&self, rhetoric: &RhetoricAnalysisResult) -> Vec<f32> {
        let mut dims = vec![0.0; 16];
        
        // 0-2: 修辞频率
        dims[0] = normalize(rhetoric.metaphor_frequency, 0.0, 50.0);
        dims[1] = normalize(rhetoric.simile_frequency, 0.0, 50.0);
        dims[2] = normalize(rhetoric.parallelism_frequency, 0.0, 20.0);
        
        // 3-7: 感官偏好
        if let Some(visual) = rhetoric.sensory_preferences.get("visual") {
            dims[3] = normalize(*visual as f32, 0.0, 100.0);
        }
        if let Some(auditory) = rhetoric.sensory_preferences.get("auditory") {
            dims[4] = normalize(*auditory as f32, 0.0, 100.0);
        }
        if let Some(tactile) = rhetoric.sensory_preferences.get("tactile") {
            dims[5] = normalize(*tactile as f32, 0.0, 100.0);
        }
        if let Some(olfactory) = rhetoric.sensory_preferences.get("olfactory") {
            dims[6] = normalize(*olfactory as f32, 0.0, 100.0);
        }
        if let Some(gustatory) = rhetoric.sensory_preferences.get("gustatory") {
            dims[7] = normalize(*gustatory as f32, 0.0, 100.0);
        }
        
        dims
    }
    
    /// 叙事层向量化（24 维）
    fn vectorize_narrative(&self, narrative: &NarrativeAnalysisResult) -> Vec<f32> {
        let mut dims = vec![0.0; 24];
        
        // 0-2: 叙事视角 one-hot 编码
        match narrative.pov_type.as_str() {
            "第一人称" => { dims[0] = 1.0; }
            "第三人称限知" => { dims[1] = 1.0; }
            "第三人称全知" => { dims[2] = 1.0; }
            _ => { dims[0] = 0.33; dims[1] = 0.33; dims[2] = 0.34; }
        }
        
        // 3: 视角一致性
        dims[3] = normalize(narrative.pov_consistency, 0.0, 1.0);
        
        // 4: Show vs Tell
        dims[4] = normalize(narrative.show_vs_tell_ratio, 0.0, 1.0);
        
        // 5-7: 信息密度
        dims[5] = normalize(narrative.entity_density, 0.0, 50.0);
        dims[6] = normalize(narrative.action_density, 0.0, 100.0);
        dims[7] = normalize(narrative.description_density, 0.0, 100.0);
        
        dims
    }
    
    /// 情感层向量化（16 维）
    fn vectorize_emotion(&self, emotion: &EmotionAnalysisResult) -> Vec<f32> {
        let mut dims = vec![0.0; 16];
        
        // 0-4: 情感基调 one-hot（简化为 5 种）
        match emotion.overall_tone.as_str() {
            "史诗感" => { dims[0] = 1.0; }
            "压抑" => { dims[1] = 1.0; }
            "轻松" => { dims[2] = 1.0; }
            "激昂" => { dims[3] = 1.0; }
            "悲伤" => { dims[4] = 1.0; }
            _ => {}
        }
        
        // 5: 基调置信度
        dims[5] = normalize(emotion.tone_confidence, 0.0, 1.0);
        
        // 6-7: 情感波动
        dims[6] = normalize(emotion.emotional_amplitude, 0.0, 2.0);
        dims[7] = normalize(emotion.emotional_frequency, 0.0, 2.0);
        
        // 8-9: 表达方式
        dims[8] = normalize(emotion.direct_expression_ratio, 0.0, 1.0);
        dims[9] = normalize(emotion.indirect_expression_ratio, 0.0, 1.0);
        
        dims
    }
    
    /// 节奏层向量化（16 维）
    fn vectorize_pacing(&self, pacing: &PacingAnalysisResult) -> Vec<f32> {
        let mut dims = vec![0.0; 16];
        
        // 0-1: 章节长度
        dims[0] = normalize(pacing.avg_chapter_length, 0.0, 10000.0);
        dims[1] = normalize(pacing.chapter_length_variance, 0.0, 5000.0);
        
        // 2: 场景切换频率
        dims[2] = normalize(pacing.scene_transition_frequency, 0.0, 20.0);
        
        // 3: 悬念结尾比例
        dims[3] = normalize(pacing.cliffhanger_ratio, 0.0, 1.0);
        
        // 4-6: 段落节奏
        dims[4] = normalize(pacing.avg_paragraph_length, 0.0, 500.0);
        dims[5] = normalize(pacing.short_paragraph_density, 0.0, 1.0);
        dims[6] = normalize(pacing.long_paragraph_density, 0.0, 1.0);
        
        dims
    }
    
    /// 对话层向量化（12 维）
    fn vectorize_dialogue(&self, dialogue: &DialogueAnalysisResult) -> Vec<f32> {
        let mut dims = vec![0.0; 12];
        
        // 0: 对话比例
        dims[0] = normalize(dialogue.dialogue_ratio, 0.0, 1.0);
        
        // 1: 角色声音区分度
        dims[1] = normalize(dialogue.character_voice_distinction, 0.0, 1.0);
        
        // 2: 对话标签频率
        dims[2] = normalize(dialogue.dialogue_tag_frequency, 0.0, 50.0);
        
        // 3: 无标签对话比例
        dims[3] = normalize(dialogue.untagged_dialogue_ratio, 0.0, 1.0);
        
        // 4: 副词修饰比例
        dims[4] = normalize(dialogue.adverb_modifier_ratio, 0.0, 1.0);
        
        // 5: 平均对话长度
        dims[5] = normalize(dialogue.avg_dialogue_length, 0.0, 200.0);
        
        dims
    }
    
    /// 描写层向量化（12 维）
    fn vectorize_description(&self, description: &DescriptionAnalysisResult) -> Vec<f32> {
        let mut dims = vec![0.0; 12];
        
        // 0: 描写比例
        dims[0] = normalize(description.description_ratio, 0.0, 1.0);
        
        // 1: 详细程度
        dims[1] = normalize(description.detail_granularity, 0.0, 1.0);
        
        // 2: 修饰词密度
        dims[2] = normalize(description.modifier_density, 0.0, 1.0);
        
        // 3-6: 描写偏好分布
        dims[3] = normalize(description.action_description_ratio, 0.0, 1.0);
        dims[4] = normalize(description.environment_description_ratio, 0.0, 1.0);
        dims[5] = normalize(description.psychological_description_ratio, 0.0, 1.0);
        dims[6] = normalize(description.appearance_description_ratio, 0.0, 1.0);
        
        dims
    }
}

/// 归一化函数：将值映射到 [-1, 1] 范围
fn normalize(value: f32, min: f32, max: f32) -> f32 {
    if max == min {
        return 0.0;
    }
    let normalized = (value - min) / (max - min);
    // 映射到 [-1, 1]
    normalized * 2.0 - 1.0
}

/// 归一化词频特征
fn normalize_word_freq(freq_list: &[(String, u32)]) -> f32 {
    if freq_list.is_empty() {
        return 0.0;
    }
    let total: u32 = freq_list.iter().map(|(_, f)| f).sum();
    let avg = total as f32 / freq_list.len() as f32;
    normalize(avg, 0.0, 100.0)
}