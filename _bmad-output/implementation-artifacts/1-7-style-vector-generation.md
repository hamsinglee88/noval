---
status: ready-for-dev
epic: 1
story: 7
story_key: 1-7-style-vector-generation
last_updated: 2026-04-09
---

# Story 1.7: 生成风格向量化表示

## Story Header

| 属性 | 值 |
|------|-----|
| **Epic** | Epic 1: 项目初始化与风格管理 |
| **Story ID** | 1.7 |
| **Story Key** | 1-7-style-vector-generation |
| **优先级** | P0 |
| **估算复杂度** | 高 |
| **状态** | ready-for-dev |
| **前置依赖** | Story 1.6 (对话层和描写层特征提取) |

---

## User Story Statement

**As a** 系统，  
**I want** 将七层风格特征转换为 128 维向量，  
**So that** 支持风格相似度计算、混合和检索。

---

## Acceptance Criteria (BDD Format)

### AC1: 七层特征数据加载

**Given** 对话层和描写层分析完成  
**When** 系统准备向量化  
**Then** 从 `style_analysis_tasks` 加载所有七层分析结果  
**And** 验证所有必需的特征字段存在  
**And** 处理缺失字段的默认值

### AC2: 词汇层向量化（16 维）

**Given** 七层特征数据加载完成  
**When** 系统执行词汇层向量化  
**Then** 将 TTR、修正 TTR 等指标编码为 16 维向量  
**And** 将常用形容词/动词/副词分布编码为向量分量  
**And** 向量分量归一化到 [-1, 1] 范围

### AC3: 句式层向量化（16 维）

**Given** 词汇层向量化完成  
**When** 系统执行句式层向量化  
**Then** 将平均句长、句长方差编码为向量  
**And** 将句式类型比例（短句/中句/长句）编码为向量  
**And** 向量分量归一化

### AC4: 修辞层向量化（16 维）

**Given** 句式层向量化完成  
**When** 系统执行修辞层向量化  
**Then** 将隐喻/明喻/排比频率编码为向量  
**And** 将感官偏好分布编码为向量分量  
**And** 向量分量归一化

### AC5: 叙事层向量化（24 维）

**Given** 修辞层向量化完成  
**When** 系统执行叙事层向量化  
**Then** 将叙事视角类型编码为 one-hot 向量  
**And** 将 Show vs Tell 比例编码为向量  
**And** 将信息密度指标编码为向量  
**And** 叙事层使用 24 维（更多维度捕捉复杂性）

### AC6: 情感层向量化（16 维）

**Given** 叙事层向量化完成  
**When** 系统执行情感层向量化  
**Then** 将情感基调类型编码为向量  
**And** 将情感波动幅度/频率编码为向量  
**And** 将直接/间接表达比例编码为向量

### AC7: 节奏层向量化（16 维）

**Given** 情感层向量化完成  
**When** 系统执行节奏层向量化  
**Then** 将章节长度特征编码为向量  
**And** 将场景切换频率编码为向量  
**And** 将悬念结尾比例编码为向量

### AC8: 对话层向量化（12 维）

**Given** 节奏层向量化完成  
**When** 系统执行对话层向量化  
**Then** 将对话比例编码为向量  
**And** 将角色声音区分度编码为向量  
**And** 将对话标签使用习惯编码为向量

### AC9: 描写层向量化（12 维）

**Given** 对话层向量化完成  
**When** 系统执行描写层向量化  
**Then** 将描写比例编码为向量  
**And** 将描写详细程度编码为向量  
**And** 将描写偏好分布编码为向量

### AC10: 风格向量存储

**Given** 128 维向量生成完成  
**When** 系统存储向量  
**Then** 将向量序列化为 JSON 字符串（或二进制格式）  
**And** 保存到 `style_profiles.style_vector` 字段  
**And** 关联到对应的风格档案

### AC11: 向量化失败处理

**Given** 向量化过程中出错  
**When** 错误发生  
**Then** 记录错误信息到 `style_analysis_tasks.error_message`  
**And** 更新状态为 `vectorization_failed`  
**And** 保留已完成的七层分析结果

---

## Story Requirements

### 功能需求

| ID | 需求 | 优先级 |
|----|------|--------|
| FR1.7.1 | 系统加载七层分析结果 | P0 |
| FR1.7.2 | 词汇层编码为 16 维向量 | P0 |
| FR1.7.3 | 句式层编码为 16 维向量 | P0 |
| FR1.7.4 | 修辞层编码为 16 维向量 | P0 |
| FR1.7.5 | 叙事层编码为 24 维向量 | P0 |
| FR1.7.6 | 情感层编码为 16 维向量 | P0 |
| FR1.7.7 | 节奏层编码为 16 维向量 | P0 |
| FR1.7.8 | 对话层编码为 12 维向量 | P0 |
| FR1.7.9 | 描写层编码为 12 维向量 | P0 |
| FR1.7.10 | 系统拼接生成 128 维风格向量 | P0 |
| FR1.7.11 | 系统存储向量到数据库 | P0 |
| FR1.7.12 | 系统更新任务状态为 completed | P0 |

### 非功能需求

| ID | 需求 | 优先级 |
|----|------|--------|
| NFR1.7.1 | 向量化过程 < 5 秒 | P0 |
| NFR1.7.2 | 向量分量归一化到 [-1, 1] | P0 |
| NFR1.7.3 | 支持向量相似度计算（余弦相似度） | P0 |
| NFR1.7.4 | 向量存储格式可扩展 | P1 |

---

## Developer Context Section

### Story Scope Guardrails

**本 Story 必须完成：**
- 七层特征到 128 维向量的映射
- 各层向量的归一化处理
- 风格向量持久化存储
- 任务状态更新为 completed（100%）

**本 Story 明确不做：**
- 风格雷达图可视化（Story 1.8）
- 风格档案确认保存 UI（Story 1.9）
- 风格混合功能（Story 1.11）

### 技术栈要求

**后端（Rust）：**
- **框架：** Axum
- **数值计算：** ndarray（用于向量操作）
- **数据存储：** SQLite + SQLx
- **序列化：** serde_json

**前端（Vue 3）：**
- 本 Story 主要是后端分析逻辑
- 前端显示向量化完成状态

### 架构合规要求

1. **模块化设计** - 各层向量化应该是独立的函数
2. **可测试性** - 向量化函数应该易于单元测试
3. **数值稳定性** - 归一化处理避免数值溢出
4. **可扩展性** - 向量维度调整不应破坏现有代码

### 核心算法实现

```rust
// services/style_analysis/style_vectorizer.rs

use ndarray::{Array1, Axis};
use serde::{Deserialize, Serialize};

/// 128 维风格向量
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleVector {
    pub vocabulary_dims: Array1<f32>,  // 0-15 (16 维)
    pub sentence_dims: Array1<f32>,    // 16-31 (16 维)
    pub rhetoric_dims: Array1<f32>,    // 32-47 (16 维)
    pub narrative_dims: Array1<f32>,   // 48-71 (24 维)
    pub emotion_dims: Array1<f32>,     // 72-87 (16 维)
    pub pacing_dims: Array1<f32>,      // 88-103 (16 维)
    pub dialogue_dims: Array1<f32>,    // 104-115 (12 维)
    pub description_dims: Array1<f32>, // 116-127 (12 维)
}

impl StyleVector {
    /// 合并为完整的 128 维向量
    pub fn to_full_vector(&self) -> Array1<f32> {
        let mut full = Array1::zeros(128);
        
        full.slice_mut(s![0..16]).assign(&self.vocabulary_dims);
        full.slice_mut(s![16..32]).assign(&self.sentence_dims);
        full.slice_mut(s![32..48]).assign(&self.rhetoric_dims);
        full.slice_mut(s![48..72]).assign(&self.narrative_dims);
        full.slice_mut(s![72..88]).assign(&self.emotion_dims);
        full.slice_mut(s![88..104]).assign(&self.pacing_dims);
        full.slice_mut(s![104..116]).assign(&self.dialogue_dims);
        full.slice_mut(s![116..128]).assign(&self.description_dims);
        
        full
    }
    
    /// 计算两个风格向量的余弦相似度
    pub fn cosine_similarity(&self, other: &StyleVector) -> f32 {
        let v1 = self.to_full_vector();
        let v2 = other.to_full_vector();
        
        let dot = v1.dot(&v2);
        let norm1 = v1.norm();
        let norm2 = v2.norm();
        
        if norm1 == 0.0 || norm2 == 0.0 {
            return 0.0;
        }
        
        dot / (norm1 * norm2)
    }
}

/// 向量化器
pub struct StyleVectorizer {
    // 各层权重（可选，用于调整各层重要性）
    pub weights: StyleWeights,
}

#[derive(Debug)]
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
    fn vectorize_vocabulary(&self, vocab: &VocabularyAnalysisResult) -> Array1<f32> {
        let mut dims = Array1::zeros(16);
        
        // 0-1: TTR 相关
        dims[0] = normalize(vocab.ttr, 0.0, 1.0);
        dims[1] = normalize(vocab.root_ttr, 0.0, 1.0);
        
        // 2-5: 词频分布（简化：用 top 词频归一化）
        dims[2] = normalize_word_freq(&vocab.common_adjectives);
        dims[3] = normalize_word_freq(&vocab.common_verbs);
        dims[4] = normalize_word_freq(&vocab.common_adverbs);
        
        // 5-15: 词汇分布特征
        dims[5] = normalize(vocab.total_words as f32 / 10000.0, 0.0, 10.0);
        dims[6] = normalize(vocab.unique_words as f32 / 5000.0, 0.0, 5.0);
        // ... 更多维度
        
        dims
    }
    
    /// 句式层向量化（16 维）
    fn vectorize_sentence(&self, sentence: &SentenceAnalysisResult) -> Array1<f32> {
        let mut dims = Array1::zeros(16);
        
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
        
        // ... 更多维度
        
        dims
    }
    
    /// 修辞层向量化（16 维）
    fn vectorize_rhetoric(&self, rhetoric: &RhetoricAnalysisResult) -> Array1<f32> {
        let mut dims = Array1::zeros(16);
        
        // 0-2: 修辞频率
        dims[0] = normalize(rhetoric.metaphor_frequency, 0.0, 50.0);
        dims[1] = normalize(rhetoric.simile_frequency, 0.0, 50.0);
        dims[2] = normalize(rhetoric.parallelism_frequency, 0.0, 20.0);
        
        // 3-7: 感官偏好
        if let Some(visual) = rhetoric.sensory_preferences.get("visual") {
            dims[3] = normalize(*visual, 0.0, 100.0);
        }
        if let Some(auditory) = rhetoric.sensory_preferences.get("auditory") {
            dims[4] = normalize(*auditory, 0.0, 100.0);
        }
        if let Some(tactile) = rhetoric.sensory_preferences.get("tactile") {
            dims[5] = normalize(*tactile, 0.0, 100.0);
        }
        if let Some(olfactory) = rhetoric.sensory_preferences.get("olfactory") {
            dims[6] = normalize(*olfactory, 0.0, 100.0);
        }
        if let Some(gustatory) = rhetoric.sensory_preferences.get("gustatory") {
            dims[7] = normalize(*gustatory, 0.0, 100.0);
        }
        
        // ... 更多维度
        
        dims
    }
    
    /// 叙事层向量化（24 维）
    fn vectorize_narrative(&self, narrative: &NarrativeAnalysisResult) -> Array1<f32> {
        let mut dims = Array1::zeros(24);
        
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
        
        // ... 更多维度
        
        dims
    }
    
    /// 情感层向量化（16 维）
    fn vectorize_emotion(&self, emotion: &EmotionAnalysisResult) -> Array1<f32> {
        let mut dims = Array1::zeros(16);
        
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
        
        // ... 更多维度
        
        dims
    }
    
    /// 节奏层向量化（16 维）
    fn vectorize_pacing(&self, pacing: &PacingAnalysisResult) -> Array1<f32> {
        let mut dims = Array1::zeros(16);
        
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
        
        // ... 更多维度
        
        dims
    }
    
    /// 对话层向量化（12 维）
    fn vectorize_dialogue(&self, dialogue: &DialogueAnalysisResult) -> Array1<f32> {
        let mut dims = Array1::zeros(12);
        
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
        
        // ... 更多维度
        
        dims
    }
    
    /// 描写层向量化（12 维）
    fn vectorize_description(&self, description: &DescriptionAnalysisResult) -> Array1<f32> {
        let mut dims = Array1::zeros(12);
        
        // 0: 描写比例
        dims[0] = normalize(description.description_ratio, 0.0, 1.0);
        
        // 1: 详细程度
        dims[1] = normalize(description.detail_granularity, 0.0, 1.0);
        
        // 2: 修饰词密度
        dims[2] = normalize(description.modifier_density, 0.0, 1.0);
        
        // 3-6: 描写偏好
        dims[3] = normalize(description.action_description_ratio, 0.0, 1.0);
        dims[4] = normalize(description.environment_description_ratio, 0.0, 1.0);
        dims[5] = normalize(description.psychological_description_ratio, 0.0, 1.0);
        dims[6] = normalize(description.appearance_description_ratio, 0.0, 1.0);
        
        // ... 更多维度
        
        dims
    }
}

/// 归一化辅助函数
fn normalize(value: f32, min: f32, max: f32) -> f32 {
    if max - min == 0.0 {
        return 0.5;
    }
    let normalized = (value - min) / (max - min);
    // 裁剪到 [-1, 1] 范围
    normalized.clamp(-1.0, 1.0)
}

fn normalize_word_freq(freq_list: &[(String, u32)]) -> f32 {
    if freq_list.is_empty() {
        return 0.0;
    }
    // 用 top 词频的集中度作为特征
    let total: u32 = freq_list.iter().map(|(_, f)| f).sum();
    let top_freq = freq_list.first().map(|(_, f)| *f).unwrap_or(0) as f32;
    normalize(top_freq / total.max(1) as f32, 0.0, 1.0)
}

use ndarray::s;
```

### 数据库 Schema

```sql
-- style_profiles 表的 style_vector 字段存储 JSON 字符串
-- 格式：{"vocabulary_dims": [...], "sentence_dims": [...], ...}

-- 或者使用数组类型（如果 SQLite 支持）
ALTER TABLE style_profiles 
ADD COLUMN style_vector_binary BLOB;  -- 存储二进制向量数据
```

### API 端点

```
# 获取风格向量
GET    /api/style-analysis/:id/vector      # 获取 128 维向量
POST   /api/styles/:id/vectorize           # 触发向量化（如果分析已完成）

# 风格相似度计算
POST   /api/styles/similarity              # 计算两个风格的相似度
{
  "style_id_1": "uuid",
  "style_id_2": "uuid"
}
```

---

## File Structure Requirements

### 后端文件结构

```
src/
├── services/
│   └── style_analysis/
│       ├── mod.rs                     # 模块导出
│       ├── vocabulary_analysis.rs     # 词汇层分析
│       ├── sentence_analysis.rs       # 句式层分析
│       ├── rhetoric_analysis.rs       # 修辞层分析
│       ├── narrative_analysis.rs      # 叙事层分析
│       ├── emotion_analysis.rs        # 情感层分析
│       ├── pacing_analysis.rs         # 节奏层分析
│       ├── dialogue_analysis.rs       # 对话层分析
│       ├── description_analysis.rs    # 描写层分析
│       ├── style_vectorizer.rs        # 向量化器（NEW）
│       └── style_analyzer.rs          # 统一分析入口
├── models/
│   ├── style_features.rs              # 特征数据结构
│   └── style_vector.rs                # 风格向量模型（NEW）
├── handlers/
│   └── style_analysis.rs              # 分析结果查询接口（扩展）
└── db/
    └── migrations/
        └── 007_add_style_vector_storage.sql
```

---

## Testing Requirements

### 后端测试（Rust）

```rust
// tests/style_vectorizer_test.rs

#[test]
fn test_vectorize_all_layers() {
    // 构造七层分析结果
    let vocab = VocabularyAnalysisResult { ... };
    let sentence = SentenceAnalysisResult { ... };
    // ...
    
    let vectorizer = StyleVectorizer::new();
    let style_vector = vectorizer.vectorize(&vocab, &sentence, &rhetoric, &narrative, &emotion, &pacing, &dialogue, &description);
    
    let full_vector = style_vector.to_full_vector();
    assert_eq!(full_vector.len(), 128);
}

#[test]
fn test_cosine_similarity() {
    let v1 = StyleVector { ... };
    let v2 = StyleVector { ... };
    
    let similarity = v1.cosine_similarity(&v2);
    assert!(similarity >= 0.0 && similarity <= 1.0);
}

#[test]
fn test_normalize_function() {
    assert_eq!(normalize(0.5, 0.0, 1.0), 0.5);
    assert_eq!(normalize(0.0, 0.0, 1.0), 0.0);
    assert_eq!(normalize(1.0, 0.0, 1.0), 1.0);
}

#[test]
fn test_vector_storage_and_retrieval() {
    // 测试向量存储到数据库并正确检索
}
```

---

## Story Completion Status

- [ ] 后端：向量化器实现
- [ ] 后端：128 维向量生成逻辑
- [ ] 后端：余弦相似度计算
- [ ] 后端：向量存储到数据库
- [ ] 后端：任务状态更新为 completed
- [ ] 测试：向量化单元测试
- [ ] 测试：相似度计算测试
- [ ] 测试：集成测试（验证 100% 进度）

---

**Story created:** 2026-04-09  
**Last updated:** 2026-04-09  
**Status:** ready-for-dev
