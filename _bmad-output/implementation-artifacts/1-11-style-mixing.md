---
status: ready-for-dev
epic: 1
story: 11
story_key: 1-11-style-mixing
last_updated: 2026-04-09
---

# Story 1.11: 混合多个风格档案

## Story Header

| 属性 | 值 |
|------|-----|
| **Epic** | Epic 1: 项目初始化与风格管理 |
| **Story ID** | 1.11 |
| **Story Key** | 1-11-style-mixing |
| **优先级** | P1 |
| **估算复杂度** | 高 |
| **状态** | ready-for-dev |
| **前置依赖** | Story 1.7 (生成风格向量化表示), Story 1.9 (保存和管理风格档案) |

---

## User Story Statement

**As a** 作家，  
**I want** 混合多个风格档案生成新风格，  
**So that** 我可以创造独特的混合风格用于创作。

---

## Acceptance Criteria (BDD Format)

### AC1: 进入风格混合器

**Given** 用户有至少两个风格档案  
**When** 用户进入风格混合器页面  
**Then** 显示可用风格档案列表供选择  
**And** 提供"开始混合"入口

### AC2: 选择多个风格

**Given** 用户正在使用风格混合器  
**When** 用户选择风格档案  
**Then** 支持选择 2-5 个风格档案  
**And** 每个风格显示名称、来源、雷达图预览  
**And** 用户可以移除已选择的风格

### AC3: 设置混合权重

**Given** 用户已选择多个风格  
**When** 用户调整混合权重  
**Then** 每个风格有滑块设置权重（0-100%）  
**And** 权重总和自动计算并显示  
**And** 权重总和必须为 100% 才能继续

### AC4: 预览混合风格雷达图

**Given** 用户已设置权重  
**When** 用户点击"预览混合效果"  
**Then** 实时计算混合风格的雷达图  
**And** 显示混合后各层特征数值  
**And** 支持对比原风格雷达图

### AC5: 命名并保存混合风格

**Given** 用户对混合效果满意  
**When** 用户点击"保存为新风格"  
**Then** 输入新风格名称  
**And** 系统验证名称不重复  
**And** 保存混合风格到数据库

### AC6: 混合风格数据计算

**Given** 用户确认保存混合风格  
**When** 系统执行混合计算  
**Then** 按权重加权平均各层特征数据  
**Then** 生成混合的 128 维风格向量  
**And** 存储完整的七层特征数据

### AC7: 取消混合

**Given** 用户正在进行混合操作  
**When** 用户点击"取消"  
**Then** 放弃当前混合设置  
**And** 返回风格库页面

### AC8: 混合历史（可选）

**Given** 用户保存了混合风格  
**When** 用户查看混合历史  
**Then** 显示该混合风格的源风格和权重  
**And** 支持重新编辑混合设置

---

## Story Requirements

### 功能需求

| ID | 需求 | 优先级 |
|----|------|--------|
| FR1.11.1 | 用户可以选择 2-5 个风格进行混合 | P0 |
| FR1.11.2 | 用户可以设置每个风格的权重 | P0 |
| FR1.11.3 | 系统实时预览混合效果 | P0 |
| FR1.11.4 | 系统计算混合风格的七层特征 | P0 |
| FR1.11.5 | 系统生成混合风格向量 | P0 |
| FR1.11.6 | 系统保存混合风格为新档案 | P0 |
| FR1.11.7 | 系统记录混合历史 | P1 |

### 非功能需求

| ID | 需求 | 优先级 |
|----|------|--------|
| NFR1.11.1 | 混合预览响应时间 < 1 秒 | P0 |
| NFR1.11.2 | 权重计算精确到小数点后 2 位 | P0 |
| NFR1.11.3 | 混合风格数据格式与原风格一致 | P0 |

---

## Developer Context Section

### Story Scope Guardrails

**本 Story 必须完成：**
- 风格混合器 UI（选择、权重设置、预览）
- 混合风格计算逻辑（向量加权平均）
- 混合风格保存功能
- 雷达图对比展示

**本 Story 明确不做：**
- 风格相似度计算（Story 1.12）
- 风格市场/分享功能（Phase 3）

### 技术栈要求

**前端（Vue 3）：**
- **框架：** Vue 3.4+ Composition API
- **语言：** TypeScript 5.x
- **UI 组件：** Naive UI（深色主题）
- **图表库：** Chart.js（雷达图对比）
- **状态管理：** Pinia

**后端（Rust）：**
- **框架：** Axum
- **数值计算：** ndarray（向量加权）
- **数据库：** SQLite + SQLx

### 架构合规要求

1. **向量计算准确性** - 混合向量 = Σ(权重 × 原向量)
2. **特征数据一致性** - 混合后的七层特征应与混合向量一致
3. **权重验证** - 后端需验证权重总和为 100%
4. **数据格式兼容** - 混合风格档案格式与原风格一致

### 核心算法实现

```rust
// services/style_analysis/style_mixer.rs

use ndarray::Array1;
use crate::models::style_profile::StyleProfile;
use crate::models::style_vector::StyleVector;

pub struct StyleMixer;

impl StyleMixer {
    /// 混合多个风格档案
    /// 
    /// # Arguments
    /// * `profiles` - 风格档案列表
    /// * `weights` - 对应的权重列表（总和应为 1.0）
    /// 
    /// # Returns
    /// * `StyleProfile` - 混合后的风格档案
    pub fn mix_styles(
        profiles: &[StyleProfile],
        weights: &[f32],
    ) -> Result<StyleProfile, String> {
        // 验证输入
        if profiles.len() < 2 {
            return Err("至少需要两个风格档案".to_string());
        }
        
        if profiles.len() != weights.len() {
            return Err("风格数量与权重数量不匹配".to_string());
        }
        
        let weight_sum: f32 = weights.iter().sum();
        if (weight_sum - 1.0).abs() > 0.01 {
            return Err(format!("权重总和应为 100%，当前为 {:.1}%", weight_sum * 100.0));
        }
        
        // 混合风格向量
        let mixed_vector = Self::mix_vectors(profiles, weights)?;
        
        // 混合七层特征数据
        let mixed_vocabulary = Self::mix_vocabulary(profiles, weights);
        let mixed_sentence = Self::mix_sentence(profiles, weights);
        let mixed_rhetoric = Self::mix_rhetoric(profiles, weights);
        let mixed_narrative = Self::mix_narrative(profiles, weights);
        let mixed_emotion = Self::mix_emotion(profiles, weights);
        let mixed_pacing = Self::mix_pacing(profiles, weights);
        let mixed_dialogue = Self::mix_dialogue(profiles, weights);
        let mixed_description = Self::mix_description(profiles, weights);
        
        // 生成混合风格名称
        let mixed_name = Self::generate_mixed_name(profiles);
        
        Ok(StyleProfile {
            name: mixed_name,
            vocabulary: mixed_vocabulary,
            sentence: mixed_sentence,
            rhetoric: mixed_rhetoric,
            narrative: mixed_narrative,
            emotion: mixed_emotion,
            pacing: mixed_pacing,
            dialogue: mixed_dialogue,
            description: mixed_description,
            style_vector: mixed_vector,
            example_passages: Self::select_example_passages(profiles, weights),
        })
    }
    
    /// 混合风格向量
    fn mix_vectors(profiles: &[StyleProfile], weights: &[f32]) -> Result<StyleVector, String> {
        let mut mixed = Array1::zeros(128);
        
        for (profile, &weight) in profiles.iter().zip(weights.iter()) {
            let vector = profile.parse_style_vector()?;
            mixed += &vector.mapv(|x| x * weight);
        }
        
        Ok(StyleVector::from_array(mixed))
    }
    
    /// 混合词汇层特征
    fn mix_vocabulary(profiles: &[StyleProfile], weights: &[f32]) -> VocabularyAnalysisResult {
        // 数值型字段加权平均
        let ttr = profiles.iter().zip(weights.iter())
            .map(|(p, &w)| p.vocabulary.ttr * w)
            .sum();
        
        let root_ttr = profiles.iter().zip(weights.iter())
            .map(|(p, &w)| p.vocabulary.root_ttr * w)
            .sum();
        
        // 词频列表：取加权平均或合并
        let common_adjectives = merge_word_freqs(
            &profiles.iter().map(|p| &p.vocabulary.common_adjectives).collect::<Vec<_>>(),
            weights
        );
        
        VocabularyAnalysisResult {
            ttr,
            root_ttr,
            common_adjectives,
            // ... 其他字段
            ..Default::default()
        }
    }
    
    /// 混合句式层特征
    fn mix_sentence(profiles: &[StyleProfile], weights: &[f32]) -> SentenceAnalysisResult {
        SentenceAnalysisResult {
            avg_sentence_length: profiles.iter().zip(weights.iter())
                .map(|(p, &w)| p.sentence.avg_sentence_length * w)
                .sum(),
            short_sentence_ratio: profiles.iter().zip(weights.iter())
                .map(|(p, &w)| p.sentence.short_sentence_ratio * w)
                .sum(),
            // ... 其他字段
            ..Default::default()
        }
    }
    
    /// 混合修辞层特征
    fn mix_rhetoric(profiles: &[StyleProfile], weights: &[f32]) -> RhetoricAnalysisResult {
        RhetoricAnalysisResult {
            metaphor_frequency: profiles.iter().zip(weights.iter())
                .map(|(p, &w)| p.rhetoric.metaphor_frequency * w)
                .sum(),
            simile_frequency: profiles.iter().zip(weights.iter())
                .map(|(p, &w)| p.rhetoric.simile_frequency * w)
                .sum(),
            // ... 其他字段
            ..Default::default()
        }
    }
    
    /// 混合叙事层特征
    fn mix_narrative(profiles: &[StyleProfile], weights: &[f32]) -> NarrativeAnalysisResult {
        // 叙事视角：取权重最高的风格的视角
        let max_weight_idx = weights.iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(i, _)| i)
            .unwrap_or(0);
        
        let pov_type = profiles[max_weight_idx].narrative.pov_type.clone();
        
        NarrativeAnalysisResult {
            pov_type,
            pov_consistency: profiles.iter().zip(weights.iter())
                .map(|(p, &w)| p.narrative.pov_consistency * w)
                .sum(),
            show_vs_tell_ratio: profiles.iter().zip(weights.iter())
                .map(|(p, &w)| p.narrative.show_vs_tell_ratio * w)
                .sum(),
            // ... 其他字段
            ..Default::default()
        }
    }
    
    /// 混合情感层特征
    fn mix_emotion(profiles: &[StyleProfile], weights: &[f32]) -> EmotionAnalysisResult {
        // 情感基调：取权重最高的风格的情感
        let max_weight_idx = weights.iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(i, _)| i)
            .unwrap_or(0);
        
        let overall_tone = profiles[max_weight_idx].emotion.overall_tone.clone();
        
        EmotionAnalysisResult {
            overall_tone,
            tone_confidence: profiles.iter().zip(weights.iter())
                .map(|(p, &w)| p.emotion.tone_confidence * w)
                .sum(),
            emotional_amplitude: profiles.iter().zip(weights.iter())
                .map(|(p, &w)| p.emotion.emotional_amplitude * w)
                .sum(),
            // ... 其他字段
            ..Default::default()
        }
    }
    
    /// 混合节奏层特征
    fn mix_pacing(profiles: &[StyleProfile], weights: &[f32]) -> PacingAnalysisResult {
        PacingAnalysisResult {
            avg_chapter_length: profiles.iter().zip(weights.iter())
                .map(|(p, &w)| p.pacing.avg_chapter_length * w)
                .sum(),
            scene_transition_frequency: profiles.iter().zip(weights.iter())
                .map(|(p, &w)| p.pacing.scene_transition_frequency * w)
                .sum(),
            cliffhanger_ratio: profiles.iter().zip(weights.iter())
                .map(|(p, &w)| p.pacing.cliffhanger_ratio * w)
                .sum(),
            // ... 其他字段
            ..Default::default()
        }
    }
    
    /// 混合对话层特征
    fn mix_dialogue(profiles: &[StyleProfile], weights: &[f32]) -> DialogueAnalysisResult {
        DialogueAnalysisResult {
            dialogue_ratio: profiles.iter().zip(weights.iter())
                .map(|(p, &w)| p.dialogue.dialogue_ratio * w)
                .sum(),
            character_voice_distinction: profiles.iter().zip(weights.iter())
                .map(|(p, &w)| p.dialogue.character_voice_distinction * w)
                .sum(),
            // ... 其他字段
            ..Default::default()
        }
    }
    
    /// 混合描写层特征
    fn mix_description(profiles: &[StyleProfile], weights: &[f32]) -> DescriptionAnalysisResult {
        DescriptionAnalysisResult {
            description_ratio: profiles.iter().zip(weights.iter())
                .map(|(p, &w)| p.description.description_ratio * w)
                .sum(),
            detail_granularity: profiles.iter().zip(weights.iter())
                .map(|(p, &w)| p.description.detail_granularity * w)
                .sum(),
            // ... 其他字段
            ..Default::default()
        }
    }
    
    /// 生成混合风格名称
    fn generate_mixed_name(profiles: &[StyleProfile]) -> String {
        let names: Vec<&str> = profiles.iter()
            .map(|p| p.name.split_whitespace().next().unwrap_or(&p.name))
            .collect();
        
        format!("{} + {} 混合风格", 
            names.first().unwrap_or(&"风格"),
            names.get(1).unwrap_or(&"风格"))
    }
    
    /// 选择示例段落
    fn select_example_passages(profiles: &[StyleProfile], weights: &[f32]) -> Vec<ExamplePassage> {
        // 从权重最高的风格中选择示例段落
        let max_weight_idx = weights.iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(i, _)| i)
            .unwrap_or(0);
        
        profiles[max_weight_idx].example_passages.clone()
    }
}

/// 合并词频列表（加权）
fn merge_word_freqs(
    freq_lists: &[&Vec<(String, u32)>],
    weights: &[f32],
) -> Vec<(String, u32)> {
    use std::collections::HashMap;
    
    let mut merged: HashMap<String, f32> = HashMap::new();
    
    for (list, &weight) in freq_lists.iter().zip(weights.iter()) {
        for (word, freq) in list {
            *merged.entry(word.clone()).or_insert(0.0) += *freq as f32 * weight;
        }
    }
    
    let mut result: Vec<(String, u32)> = merged
        .into_iter()
        .map(|(word, freq)| (word, freq.round() as u32))
        .collect();
    
    result.sort_by(|a, b| b.1.cmp(&a.1));
    result.into_iter().take(50).collect()
}
```

### API 设计

```rust
// handlers/style_mixing.rs

/// 混合风格请求
#[derive(Debug, Deserialize)]
pub struct MixStylesRequest {
    pub style_ids: Vec<String>,
    pub weights: Vec<f32>,
    pub custom_name: Option<String>,
}

/// 预览混合效果
#[post("/api/styles/mix/preview")]
pub async fn preview_mix(
    Json(req): Json<MixStylesRequest>,
    db: Extension<SqlitePool>,
) -> Result<Json<StyleProfilePreview>> {
    // 1. 验证风格 ID 有效
    // 2. 验证权重总和为 100%
    // 3. 计算混合风格（不保存）
    // 4. 返回预览数据
}

/// 保存混合风格
#[post("/api/styles/mix/save")]
pub async fn save_mixed_style(
    Json(req): Json<MixStylesRequest>,
    db: Extension<SqlitePool>,
) -> Result<Json<SaveMixedStyleResponse>> {
    // 1. 验证风格 ID 有效
    // 2. 验证权重总和
    // 3. 验证名称不重复
    // 4. 计算并保存混合风格
    // 5. 记录混合历史
}

/// 获取混合历史
#[get("/api/styles/mix/history")]
pub async fn get_mix_history(
    db: Extension<SqlitePool>,
) -> Result<Json<Vec<MixHistoryEntry>>> {
    // 返回用户的混合历史记录
}
```

### 数据库 Schema

```sql
-- 混合历史记录表
CREATE TABLE style_mix_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    result_style_id UUID REFERENCES style_profiles(id),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    source_styles JSONB NOT NULL,  -- [{style_id, weight, name}, ...]
    mixing_params JSONB            -- 其他混合参数
);

-- 索引
CREATE INDEX idx_style_mix_history_result ON style_mix_history(result_style_id);
```

---

## File Structure Requirements

### 前端文件结构

```
src/
├── views/
│   └── StyleMixerView.vue           # 风格混合器页面（NEW）
├── components/
│   └── style/
│       ├── StyleSelector.vue        # 风格选择器（扩展）
│       ├── WeightSlider.vue         # 权重滑块组件（NEW）
│       ├── MixedRadarChart.vue      # 混合雷达图对比（NEW）
│       └── StyleMixerPreview.vue    # 混合预览卡片（NEW）
├── stores/
│   └── style.ts                     # 风格状态管理（扩展）
└── services/
    └── style.ts                     # 风格 API 客户端（扩展）
```

### 后端文件结构

```
src/
├── handlers/
│   └── style_mixing.rs              # 风格混合处理器（NEW）
├── services/
│   └── style_mixer.rs               # 风格混合器服务（NEW）
└── db/
    └── migrations/
        └── 010_create_style_mix_history.sql
```

---

## Testing Requirements

### 后端测试（Rust）

```rust
// tests/style_mixer_test.rs

#[test]
fn test_mix_two_styles_equal_weight() {
    // 测试 50% + 50% 混合
}

#[test]
fn test_mix_styles_invalid_weights() {
    // 测试权重和不为 100% 被拒绝
}

#[test]
fn test_mix_styles_single_style_rejected() {
    // 测试单个风格被拒绝
}

#[test]
fn test_mix_vector_calculation() {
    // 测试向量混合计算准确性
}

#[test]
fn test_mix_narrative_pov_selection() {
    // 测试叙事视角取权重最高的
}
```

### 前端测试（Vitest）

```typescript
// tests/StyleMixerView.test.ts

describe('StyleMixerView', () => {
  it('should allow selecting 2-5 styles', () => {
    // 测试选择 2-5 个风格
  });

  it('should validate weight sum equals 100%', () => {
    // 测试权重验证
  });

  it('should display mixed radar chart preview', async () => {
    // 测试雷达图预览
  });

  it('should save mixed style successfully', async () => {
    // 测试保存成功
  });
});
```

---

## Story Completion Status

- [ ] 后端：风格混合器服务实现
- [ ] 后端：混合风格计算逻辑
- [ ] 后端：混合预览接口
- [ ] 后端：保存混合风格接口
- [ ] 后端：数据库迁移（混合历史）
- [ ] 前端：风格混合器页面
- [ ] 前端：权重滑块组件
- [ ] 前端：混合雷达图对比
- [ ] 测试：后端单元测试
- [ ] 测试：前端组件测试

---

**Story created:** 2026-04-09  
**Last updated:** 2026-04-09  
**Status:** ready-for-dev
