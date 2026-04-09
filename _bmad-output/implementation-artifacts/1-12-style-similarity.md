---
status: ready-for-dev
epic: 1
story: 12
story_key: 1-12-style-similarity
last_updated: 2026-04-09
---

# Story 1.12: 计算两段文本的风格相似度

## Story Header

| 属性 | 值 |
|------|-----|
| **Epic** | Epic 1: 项目初始化与风格管理 |
| **Story ID** | 1.12 |
| **Story Key** | 1-12-style-similarity |
| **优先级** | P1 |
| **估算复杂度** | 中 |
| **状态** | ready-for-dev |
| **前置依赖** | Story 1.7 (生成风格向量化表示) |

---

## User Story Statement

**As a** 系统，  
**I want** 计算两段文本或两个风格的相似度，  
**So that** 可以评估生成内容与目标风格的匹配度或发现风格相近的作品。

---

## Acceptance Criteria (BDD Format)

### AC1: 计算两个风格档案的相似度

**Given** 用户有两个风格档案  
**When** 用户请求计算相似度  
**Then** 系统加载两个风格的 128 维向量  
**And** 使用余弦相似度算法计算  
**And** 返回 0-1 之间的相似度分数

### AC2: 计算文本与风格的相似度

**Given** 用户有一段文本和一个风格档案  
**When** 用户请求计算相似度  
**Then** 系统对文本进行简式风格分析  
**And** 提取关键特征并与风格向量对比  
**And** 返回相似度分数和匹配详情

### AC3: 风格相似度分级显示

**Given** 相似度计算完成  
**When** 系统显示结果  
**Then** 用颜色编码相似度等级  
**And** 提供文字描述（如"高度匹配""中等匹配"）

### AC4: 批量相似度计算

**Given** 用户有多个风格档案  
**When** 用户请求批量计算  
**Then** 计算一个风格与多个风格的相似度  
**And** 按相似度排序返回结果

### AC5: 相似度计算 API

**Given** 外部系统调用相似度 API  
**When** 发送有效的风格 ID 对  
**Then** 返回标准 JSON 格式的相似度结果

### AC6: 七层特征相似度分解

**Given** 相似度计算完成  
**When** 用户查看详细结果  
**Then** 显示各层特征的相似度（词汇/句式/修辞等）  
**And** 帮助用户理解相似度来源

### AC7: 高相似度阈值告警

**Given** 两段文本相似度超过阈值（如 0.95）  
**When** 系统检测到极高相似度  
**Then** 提示可能存在抄袭风险  
**And** 建议用户审查

### AC8: 缓存相似度结果

**Given** 相似度已被计算过  
**When** 用户再次请求相同计算  
**Then** 系统返回缓存结果  
**And** 标注结果来源（缓存/实时计算）

---

## Story Requirements

### 功能需求

| ID | 需求 | 优先级 |
|----|------|--------|
| FR1.12.1 | 系统计算两个风格档案的相似度 | P0 |
| FR1.12.2 | 系统计算文本与风格的相似度 | P0 |
| FR1.12.3 | 系统显示相似度分级和描述 | P0 |
| FR1.12.4 | 系统支持批量相似度计算 | P1 |
| FR1.12.5 | 系统提供相似度计算 API | P0 |
| FR1.12.6 | 系统显示七层特征相似度分解 | P1 |
| FR1.12.7 | 系统检测极高相似度并告警 | P1 |
| FR1.12.8 | 系统缓存相似度计算结果 | P1 |

### 非功能需求

| ID | 需求 | 优先级 |
|----|------|--------|
| NFR1.12.1 | 单次相似度计算时间 < 100ms | P0 |
| NFR1.12.2 | 批量计算响应时间 < 1 秒（10 个以内） | P0 |
| NFR1.12.3 | 相似度分数精度到小数点后 4 位 | P0 |
| NFR1.12.4 | API 支持每秒 100 次请求 | P1 |

---

## Developer Context Section

### Story Scope Guardrails

**本 Story 必须完成：**
- 余弦相似度算法实现
- 两个风格档案的相似度计算
- 相似度结果分级显示
- 七层特征相似度分解
- 相似度计算 API

**本 Story 明确不做：**
- 完整的抄袭检测系统（仅需告警提示）
- 风格市场/推荐功能（Phase 3）

### 技术栈要求

**后端（Rust）：**
- **框架：** Axum
- **数值计算：** ndarray（向量操作）
- **缓存：** 内存缓存或 Redis（可选）
- **数据库：** SQLite + SQLx

**前端（Vue 3）：**
- **框架：** Vue 3.4+ Composition API
- **语言：** TypeScript 5.x
- **UI 组件：** Naive UI（深色主题）
- **图表库：** Chart.js（雷达图对比）

### 架构合规要求

1. **算法准确性** - 余弦相似度实现需经过单元测试验证
2. **数值稳定性** - 处理零向量和极小值情况
3. **性能优化** - 批量计算使用并行处理
4. **结果缓存** - 避免重复计算相同请求

### 核心算法实现

```rust
// services/style_analysis/similarity_calculator.rs

use ndarray::Array1;

pub struct SimilarityCalculator;

impl SimilarityCalculator {
    /// 计算两个风格向量的余弦相似度
    /// 
    /// # Arguments
    /// * `v1` - 第一个风格向量（128 维）
    /// * `v2` - 第二个风格向量（128 维）
    /// 
    /// # Returns
    /// * `f32` - 相似度分数（0.0 - 1.0）
    pub fn cosine_similarity(v1: &Array1<f32>, v2: &Array1<f32>) -> f32 {
        let dot_product = v1.dot(v2);
        let norm1 = v1.norm();
        let norm2 = v2.norm();
        
        if norm1 == 0.0 || norm2 == 0.0 {
            return 0.0;
        }
        
        let similarity = dot_product / (norm1 * norm2);
        
        // 余弦相似度范围是 [-1, 1]，但风格向量应为正值，所以映射到 [0, 1]
        ((similarity + 1.0) / 2.0).clamp(0.0, 1.0)
    }
    
    /// 计算两个风格档案的相似度
    pub fn calculate_style_similarity(
        profile1: &StyleProfile,
        profile2: &StyleProfile,
    ) -> Result<SimilarityResult, String> {
        let v1 = profile1.parse_style_vector()?;
        let v2 = profile2.parse_style_vector()?;
        
        let overall_similarity = Self::cosine_similarity(&v1, &v2);
        
        // 计算各层特征的相似度
        let layer_similarities = Self::calculate_layer_similarities(profile1, profile2);
        
        Ok(SimilarityResult {
            overall_similarity,
            layer_similarities,
            similarity_level: Self::classify_similarity(overall_similarity),
            is_cached: false,
        })
    }
    
    /// 计算七层特征的相似度分解
    fn calculate_layer_similarities(
        profile1: &StyleProfile,
        profile2: &StyleProfile,
    ) -> LayerSimilarities {
        LayerSimilarities {
            vocabulary: Self::cosine_similarity(
                &profile1.vocabulary.to_vector(),
                &profile2.vocabulary.to_vector(),
            ),
            sentence: Self::cosine_similarity(
                &profile1.sentence.to_vector(),
                &profile2.sentence.to_vector(),
            ),
            rhetoric: Self::cosine_similarity(
                &profile1.rhetoric.to_vector(),
                &profile2.rhetoric.to_vector(),
            ),
            narrative: Self::cosine_similarity(
                &profile1.narrative.to_vector(),
                &profile2.narrative.to_vector(),
            ),
            emotion: Self::cosine_similarity(
                &profile1.emotion.to_vector(),
                &profile2.emotion.to_vector(),
            ),
            pacing: Self::cosine_similarity(
                &profile1.pacing.to_vector(),
                &profile2.pacing.to_vector(),
            ),
            dialogue: Self::cosine_similarity(
                &profile1.dialogue.to_vector(),
                &profile2.dialogue.to_vector(),
            ),
            description: Self::cosine_similarity(
                &profile1.description.to_vector(),
                &profile2.description.to_vector(),
            ),
        }
    }
    
    /// 计算文本与风格的相似度
    pub fn calculate_text_style_similarity(
        text: &str,
        style_profile: &StyleProfile,
    ) -> Result<SimilarityResult, String> {
        // 1. 对文本进行简式风格分析（仅提取关键特征）
        let text_vector = Self::extract_text_style_vector(text)?;
        
        // 2. 计算与目标风格的相似度
        let style_vector = style_profile.parse_style_vector()?;
        let overall_similarity = Self::cosine_similarity(&text_vector, &style_vector);
        
        Ok(SimilarityResult {
            overall_similarity,
            layer_similarities: LayerSimilarities::default(), // 简式分析不分解
            similarity_level: Self::classify_similarity(overall_similarity),
            is_cached: false,
        })
    }
    
    /// 从文本提取简式风格向量
    fn extract_text_style_vector(text: &str) -> Result<Array1<f32>, String> {
        // 简式分析：仅提取关键特征用于快速相似度计算
        // 这里调用风格分析模块的简化版本
        
        let vocab = extract_vocabulary_features_fast(text);
        let sentence = extract_sentence_features_fast(text);
        let rhetoric = extract_rhetoric_features_fast(text);
        // ...
        
        // 合并为 128 维向量
        Ok(vectorizer.vectorize(&vocab, &sentence, &rhetoric, ...).to_full_vector())
    }
    
    /// 相似度分级
    fn classify_similarity(similarity: f32) -> SimilarityLevel {
        if similarity >= 0.95 {
            SimilarityLevel::ExtremelyHigh // 可能抄袭
        } else if similarity >= 0.85 {
            SimilarityLevel::VeryHigh // 高度匹配
        } else if similarity >= 0.70 {
            SimilarityLevel::High // 较高匹配
        } else if similarity >= 0.50 {
            SimilarityLevel::Medium // 中等匹配
        } else if similarity >= 0.30 {
            SimilarityLevel::Low // 较低匹配
        } else {
            SimilarityLevel::VeryLow // 差异很大
        }
    }
    
    /// 批量计算相似度
    pub fn batch_similarity(
        base_profile: &StyleProfile,
        target_profiles: &[StyleProfile],
    ) -> Vec<SimilarityResult> {
        use rayon::prelude::*;
        
        target_profiles
            .par_iter()
            .map(|target| Self::calculate_style_similarity(base_profile, target))
            .filter_map(|result| result.ok())
            .collect()
    }
}

/// 相似度结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityResult {
    pub overall_similarity: f32,           // 总体相似度
    pub layer_similarities: LayerSimilarities, // 各层相似度
    pub similarity_level: SimilarityLevel,     // 相似度等级
    pub is_cached: bool,                       // 是否来自缓存
}

/// 相似度等级
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SimilarityLevel {
    ExtremelyHigh, // 0.95 - 1.00 (可能抄袭)
    VeryHigh,      // 0.85 - 0.95 (高度匹配)
    High,          // 0.70 - 0.85 (较高匹配)
    Medium,        // 0.50 - 0.70 (中等匹配)
    Low,           // 0.30 - 0.50 (较低匹配)
    VeryLow,       // 0.00 - 0.30 (差异很大)
}

impl SimilarityLevel {
    pub fn to_display(&self) -> &'static str {
        match self {
            SimilarityLevel::ExtremelyHigh => "极高（可能抄袭）",
            SimilarityLevel::VeryHigh => "很高",
            SimilarityLevel::High => "较高",
            SimilarityLevel::Medium => "中等",
            SimilarityLevel::Low => "较低",
            SimilarityLevel::VeryLow => "很低",
        }
    }
    
    pub fn to_color(&self) -> &'static str {
        match self {
            SimilarityLevel::ExtremelyHigh => "#F48771", // 红色（警告）
            SimilarityLevel::VeryHigh => "#D19A66",      // 橙色
            SimilarityLevel::High => "#D19A66",          // 橙色
            SimilarityLevel::Medium => "#569CD6",        // 蓝色
            SimilarityLevel::Low => "#6A9955",           // 绿色
            SimilarityLevel::VeryLow => "#6A9955",       // 绿色
        }
    }
}

/// 七层特征相似度
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LayerSimilarities {
    pub vocabulary: f32,
    pub sentence: f32,
    pub rhetoric: f32,
    pub narrative: f32,
    pub emotion: f32,
    pub pacing: f32,
    pub dialogue: f32,
    pub description: f32,
}
```

### API 设计

```rust
// handlers/similarity.rs

/// 计算两个风格的相似度
#[derive(Debug, Deserialize)]
pub struct CalculateSimilarityRequest {
    pub style_id_1: String,
    pub style_id_2: String,
}

#[post("/api/styles/similarity")]
pub async fn calculate_similarity(
    Json(req): Json<CalculateSimilarityRequest>,
    db: Extension<SqlitePool>,
) -> Result<Json<SimilarityResult>> {
    // 1. 验证两个风格 ID 有效
    // 2. 检查缓存
    // 3. 计算相似度
    // 4. 缓存结果
    // 5. 返回结果
}

/// 批量计算相似度
#[post("/api/styles/similarity/batch")]
pub async fn batch_similarity(
    Json(req): Json<BatchSimilarityRequest>,
    db: Extension<SqlitePool>,
) -> Result<Json<Vec<SimilarityResult>>> {
    // 1. 验证基础风格 ID 有效
    // 2. 验证目标风格 ID 列表
    // 3. 批量计算
    // 4. 按相似度排序
    // 5. 返回结果
}

/// 计算文本与风格的相似度
#[post("/api/styles/similarity/text")]
pub async fn calculate_text_style_similarity(
    Json(req): Json<TextStyleSimilarityRequest>,
    db: Extension<SqlitePool>,
) -> Result<Json<SimilarityResult>> {
    // 1. 验证风格 ID 有效
    // 2. 对文本进行简式分析
    // 3. 计算相似度
    // 4. 返回结果
}

/// 获取相似度分解详情
#[get("/api/styles/similarity/{id}/details")]
pub async fn get_similarity_details(
    path: Path<String>,
    db: Extension<SqlitePool>,
) -> Result<Json<SimilarityResult>> {
    // 返回缓存的相似度详情（含七层分解）
}
```

### 数据库 Schema

```sql
-- 相似度结果缓存表
CREATE TABLE similarity_cache (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    style_id_1 UUID REFERENCES style_profiles(id),
    style_id_2 UUID REFERENCES style_profiles(id),
    similarity_result JSONB NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP,
    
    UNIQUE(style_id_1, style_id_2)
);

-- 索引
CREATE INDEX idx_similarity_cache_pair ON similarity_cache(style_id_1, style_id_2);
CREATE INDEX idx_similarity_cache_expires ON similarity_cache(expires_at);
```

---

## File Structure Requirements

### 前端文件结构

```
src/
├── views/
│   └── StyleSimilarityView.vue        # 风格相似度页面（NEW）
├── components/
│   └── style/
│       ├── SimilarityResult.vue       # 相似度结果组件（NEW）
│       ├── LayerSimilarityChart.vue   # 七层相似度雷达图（NEW）
│       └── StyleComparison.vue        # 风格对比组件（NEW）
├── stores/
│   └── style.ts                       # 风格状态管理（扩展）
└── services/
    └── similarity.ts                  # 相似度 API 客户端（NEW）
```

### 后端文件结构

```
src/
├── handlers/
│   └── similarity.rs                  # 相似度处理器（NEW）
├── services/
│   └── similarity_calculator.rs       # 相似度计算器（NEW）
└── db/
    └── migrations/
        └── 011_create_similarity_cache.sql
```

---

## Testing Requirements

### 后端测试（Rust）

```rust
// tests/similarity_test.rs

#[test]
fn test_cosine_similarity_identical_vectors() {
    let v1 = Array1::from(vec![1.0, 2.0, 3.0]);
    let v2 = Array1::from(vec![1.0, 2.0, 3.0]);
    let similarity = SimilarityCalculator::cosine_similarity(&v1, &v2);
    assert!((similarity - 1.0).abs() < 0.0001);
}

#[test]
fn test_cosine_similarity_orthogonal_vectors() {
    let v1 = Array1::from(vec![1.0, 0.0, 0.0]);
    let v2 = Array1::from(vec![0.0, 1.0, 0.0]);
    let similarity = SimilarityCalculator::cosine_similarity(&v1, &v2);
    assert!((similarity - 0.0).abs() < 0.0001);
}

#[test]
fn test_style_similarity_same_profile() {
    // 测试同一风格相似度为 1
}

#[test]
fn test_style_similarity_different_styles() {
    // 测试不同风格相似度
}

#[test]
fn test_text_style_similarity() {
    // 测试文本与风格相似度
}

#[test]
fn test_similarity_cache_hit() {
    // 测试缓存命中
}
```

### 前端测试（Vitest）

```typescript
// tests/StyleSimilarityView.test.ts

describe('StyleSimilarityView', () => {
  it('should display similarity result', async () => {
    // 测试相似度结果显示
  });

  it('should display layer similarities breakdown', async () => {
    // 测试七层相似度分解
  });

  it('should display similarity level with color', async () => {
    // 测试相似度等级颜色
  });

  it('should handle batch similarity calculation', async () => {
    // 测试批量计算
  });
});
```

---

## Story Completion Status

- [ ] 后端：余弦相似度算法实现
- [ ] 后端：风格相似度计算服务
- [ ] 后端：文本 - 风格相似度计算
- [ ] 后端：批量相似度计算
- [ ] 后端：相似度缓存机制
- [ ] 后端：相似度计算 API
- [ ] 后端：数据库迁移（缓存表）
- [ ] 前端：风格相似度页面
- [ ] 前端：相似度结果组件
- [ ] 前端：七层相似度雷达图
- [ ] 测试：后端单元测试
- [ ] 测试：前端组件测试

---

**Story created:** 2026-04-09  
**Last updated:** 2026-04-09  
**Status:** ready-for-dev
