---
status: ready-for-dev
epic: 1
story: 3
story_key: 1-3-vocabulary-sentence-analysis
last_updated: 2026-04-09
---

# Story 1.3: 词汇层和句式层特征提取

## Story Header

| 属性 | 值 |
|------|-----|
| **Epic** | Epic 1: 项目初始化与风格管理 |
| **Story ID** | 1.3 |
| **Story Key** | 1-3-vocabulary-sentence-analysis |
| **优先级** | P0 |
| **估算复杂度** | 中 |
| **状态** | ready-for-dev |
| **前置依赖** | Story 1.2 (上传小说进行风格分析) |

---

## User Story Statement

**As a** 系统，  
**I want** 提取文本的词汇层和句式层特征，  
**So that** 量化用户的写作风格。

---

## Acceptance Criteria (BDD Format)

### AC1: 词汇层分析 - 词性统计

**Given** 文本上传完成并解析为纯文本  
**When** 系统执行词汇层分析  
**Then** 对文本进行分词和词性标注  
**And** 提取常用形容词列表（按频率排序，top 50）  
**And** 提取常用动词列表（按频率排序，top 50）  
**And** 提取常用副词列表（按频率排序，top 50）

### AC2: 词汇丰富度计算

**Given** 分词完成  
**When** 系统计算词汇丰富度  
**Then** 计算 Type-Token Ratio (TTR = 唯一词数 / 总词数)  
**And** 计算修正 TTR (Root TTR 或 Log TTR，避免文本长度影响)  
**And** 存储到词汇层特征数据结构

### AC3: 句式层分析 - 句长统计

**Given** 词汇分析完成  
**When** 系统执行句式层分析  
**Then** 基于标点符号进行分句（。！？；……）  
**Then** 统计平均句长（字符数）  
**And** 计算句长标准差（衡量句长变化）

### AC4: 句式类型比例

**Given** 分句完成  
**When** 系统分析句式类型  
**Then** 计算短句比例（<10 字）  
**And** 计算中句比例（10-30 字）  
**And** 计算长句比例（>30 字）  
**And** 计算复合句比例（包含连接词的句子）

### AC5: 特殊句式识别

**Given** 句式分析进行中  
**When** 系统识别特殊句式  
**Then** 统计问句比例（包含问号）  
**And** 统计感叹句比例（包含感叹号）  
**And** 统计段落平均句数

### AC6: 分析结果存储

**Given** 词汇层和句式层分析完成  
**When** 系统存储分析结果  
**Then** 将特征数据保存到 `style_analysis_tasks.progress_data`  
**And** 更新任务状态为 `vocabulary_sentence_completed`  
**And** 进度更新为 25%（七层中的两层完成）

### AC7: 分析失败处理

**Given** 文本解析失败或分析过程出错  
**When** 错误发生  
**Then** 记录错误信息到 `style_analysis_tasks.error_message`  
**And** 更新状态为 `failed`  
**And** 保留已完成的中间结果（如有）

---

## Story Requirements

### 功能需求

| ID | 需求 | 优先级 |
|----|------|--------|
| FR1.3.1 | 系统支持中文分词和词性标注 | P0 |
| FR1.3.2 | 系统提取 top 50 形容词/动词/副词 | P0 |
| FR1.3.3 | 系统计算 TTR 和修正 TTR | P0 |
| FR1.3.4 | 系统统计平均句长和句长分布 | P0 |
| FR1.3.5 | 系统计算短句/中句/长句比例 | P0 |
| FR1.3.6 | 系统识别问句/感叹句比例 | P1 |
| FR1.3.7 | 系统存储分析结果到数据库 | P0 |
| FR1.3.8 | 系统更新分析任务进度 | P0 |

### 非功能需求

| ID | 需求 | 优先级 |
|----|------|--------|
| NFR1.3.1 | 10 万字文本分析时间 < 30 秒 | P0 |
| NFR1.3.2 | 分词准确率 > 95% | P0 |
| NFR1.3.3 | 支持 UTF-8 和 GBK 编码 | P0 |
| NFR1.3.4 | 内存占用 < 500MB（10 万字） | P1 |

---

## Developer Context Section

### Story Scope Guardrails

**本 Story 必须完成：**
- 中文分词和词性标注（使用 jieba 或类似库）
- 词汇层特征提取（形容词/动词/副词列表，TTR）
- 句式层特征提取（句长统计，句式比例）
- 分析结果持久化到数据库
- 进度更新机制

**本 Story 明确不做：**
- 修辞层、叙事层、情感层、节奏层、对话层、描写层分析
- 128 维风格向量生成
- 风格雷达图可视化
- 风格报告展示 UI

### 技术栈要求

**后端（Rust）：**
- **框架：** Axum
- **中文分词：** jieba-rs 或类似 Rust 中文分词库
- **文本处理：** regex, unicode-segmentation
- **数据存储：** SQLite + SQLx

**前端（Vue 3）：**
- 本 Story 主要是后端分析逻辑
- 前端只需要显示分析进度（复用 Story 1.2 的进度组件）

### 架构合规要求

1. **模块化设计** - 词汇分析和句式分析应该是独立的可测试函数
2. **流式处理** - 大文本应该分块处理，避免一次性加载到内存
3. **进度可追踪** - 每一步都应该更新任务进度
4. **错误可恢复** - 分析失败应该保留已完成的中间结果

### 核心算法实现

#### 词汇层分析

```rust
// services/style_analysis/vocabulary_analysis.rs

use jieba_rs::Jieba;
use std::collections::HashMap;

pub struct VocabularyAnalysisResult {
    pub common_adjectives: Vec<(String, u32)>,  // (词，频率)
    pub common_verbs: Vec<(String, u32)>,
    pub common_adverbs: Vec<(String, u32)>,
    pub ttr: f32,                               // Type-Token Ratio
    pub root_ttr: f32,                          // 修正 TTR
    pub total_words: usize,
    pub unique_words: usize,
}

pub fn extract_vocabulary_features(text: &str) -> VocabularyAnalysisResult {
    let jieba = Jieba::new();
    
    // 分词
    let words: Vec<&str> = jieba.cut(text, false).collect();
    
    // 词性标注和分类
    let mut adj_freq: HashMap<String, u32> = HashMap::new();
    let mut verb_freq: HashMap<String, u32> = HashMap::new();
    let mut adv_freq: HashMap<String, u32> = HashMap::new();
    
    for word in &words {
        let pos = jieba.tag(word).unwrap_or("unknown");
        match pos {
            "a" | "adjective" => {
                *adj_freq.entry(word.to_string()).or_insert(0) += 1;
            }
            "v" | "verb" => {
                *verb_freq.entry(word.to_string()).or_insert(0) += 1;
            }
            "d" | "adverb" => {
                *adv_freq.entry(word.to_string()).or_insert(0) += 1;
            }
            _ => {}
        }
    }
    
    // 计算 TTR
    let unique_words: HashSet<_> = words.iter().collect();
    let ttr = unique_words.len() as f32 / words.len() as f32;
    let root_ttr = (unique_words.len() as f32).sqrt() / (words.len() as f32).sqrt();
    
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

fn get_top_n(freq_map: &HashMap<String, u32>, n: usize) -> Vec<(String, u32)> {
    let mut items: Vec<_> = freq_map.iter()
        .map(|(k, v)| (k.clone(), *v))
        .collect();
    items.sort_by(|a, b| b.1.cmp(&a.1));
    items.into_iter().take(n).collect()
}
```

#### 句式层分析

```rust
// services/style_analysis/sentence_analysis.rs

use regex::Regex;

pub struct SentenceAnalysisResult {
    pub avg_sentence_length: f32,
    pub sentence_length_variance: f32,
    pub short_sentence_ratio: f32,    // < 10 字
    pub medium_sentence_ratio: f32,   // 10-30 字
    pub long_sentence_ratio: f32,     // > 30 字
    pub complex_sentence_ratio: f32,  // 包含连接词
    pub question_ratio: f32,
    pub exclamation_ratio: f32,
    pub paragraph_avg_sentences: f32,
}

pub fn extract_sentence_features(text: &str) -> SentenceAnalysisResult {
    // 分句（基于中文标点）
    let sentence_pattern = Regex::new(r"[.!.??;;……\n]+").unwrap();
    let sentences: Vec<&str> = sentence_pattern
        .split(text)
        .filter(|s| !s.trim().is_empty())
        .collect();
    
    // 句长统计
    let sentence_lengths: Vec<usize> = sentences
        .iter()
        .map(|s| s.chars().count())
        .collect();
    
    let avg_length = mean(&sentence_lengths);
    let variance = variance(&sentence_lengths);
    
    // 句式比例
    let short_count = sentence_lengths.iter().filter(|&&l| l < 10).count();
    let medium_count = sentence_lengths.iter().filter(|&&l| l >= 10 && l <= 30).count();
    let long_count = sentence_lengths.iter().filter(|&&l| l > 30).count();
    let total = sentences.len() as f32;
    
    // 问句/感叹句
    let question_count = sentences.iter().filter(|s| s.contains('?')).count();
    let exclamation_count = sentences.iter().filter(|s| s.contains('!')).count();
    
    // 复合句（简单检测连接词）
    let conjunctions = ["因为", "所以", "虽然", "但是", "如果", "即使", "尽管"];
    let complex_count = sentences.iter()
        .filter(|s| conjunctions.iter().any(|c| s.contains(c)))
        .count();
    
    SentenceAnalysisResult {
        avg_sentence_length: avg_length,
        sentence_length_variance: variance,
        short_sentence_ratio: short_count as f32 / total,
        medium_sentence_ratio: medium_count as f32 / total,
        long_sentence_ratio: long_count as f32 / total,
        complex_sentence_ratio: complex_count as f32 / total,
        question_ratio: question_count as f32 / total,
        exclamation_ratio: exclamation_count as f32 / total,
        paragraph_avg_sentences: calculate_paragraph_avg(text),
    }
}

fn mean(values: &[usize]) -> f32 {
    if values.is_empty() { return 0.0; }
    values.iter().sum::<usize>() as f32 / values.len() as f32
}

fn variance(values: &[usize]) -> f32 {
    if values.is_empty() { return 0.0; }
    let m = mean(values);
    values.iter().map(|&v| (v as f32 - m).powi(2)).sum::<f32>() / values.len() as f32
}
```

### 数据库 Schema

```sql
-- 更新 style_analysis_tasks 表，添加中间结果存储
ALTER TABLE style_analysis_tasks 
ADD COLUMN vocabulary_json TEXT,
ADD COLUMN sentence_json TEXT;

-- 或者使用中间结果表
CREATE TABLE analysis_intermediate_results (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    task_id UUID REFERENCES style_analysis_tasks(id) ON DELETE CASCADE,
    layer_type TEXT NOT NULL,  -- 'vocabulary', 'sentence', etc.
    result_data JSONB NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_analysis_results_task ON analysis_intermediate_results(task_id);
CREATE INDEX idx_analysis_results_layer ON analysis_intermediate_results(layer_type);
```

### API 端点扩展

```
# 风格分析进度查询
GET    /api/style-analysis/:id      # 获取任务详情（包含各层进度）
GET    /api/style-analysis/:id/vocabulary  # 获取词汇层分析结果
GET    /api/style-analysis/:id/sentence    # 获取句式层分析结果
```

### 响应格式

```json
{
  "success": true,
  "data": {
    "task_id": "uuid",
    "status": "vocabulary_sentence_completed",
    "progress": 0.25,
    "vocabulary": {
      "common_adjectives": [["苍凉", 120], ["雄浑", 98]],
      "common_verbs": [["纵身", 85], ["提气", 72]],
      "common_adverbs": [["猛然", 45], ["缓缓", 38]],
      "ttr": 0.65,
      "root_ttr": 0.78,
      "total_words": 50000,
      "unique_words": 32500
    },
    "sentence": {
      "avg_sentence_length": 23.5,
      "sentence_length_variance": 12.3,
      "short_sentence_ratio": 0.35,
      "medium_sentence_ratio": 0.45,
      "long_sentence_ratio": 0.20,
      "complex_sentence_ratio": 0.25,
      "question_ratio": 0.05,
      "exclamation_ratio": 0.08
    }
  }
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
│       └── style_analyzer.rs          # 统一分析入口
├── models/
│   └── style_features.rs              # 特征数据结构
├── handlers/
│   └── style_analysis.rs              # 分析结果查询接口
└── db/
    └── migrations/
        └── 003_add_analysis_results.sql
```

### 前端扩展

```
src/
└── components/
    └── style/
        └── AnalysisProgress.vue     # 复用，增加词汇/句式步骤显示
```

---

## Testing Requirements

### 后端测试（Rust）

```rust
// tests/vocabulary_analysis_test.rs

#[test]
fn test_extract_common_adjectives() {
    let text = "美丽的花园里开满了鲜艳的花朵...";
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
fn test_large_text_vocabulary_analysis() {
    let text = std::fs::read_to_string("test_data/large_sample.txt").unwrap();
    let result = extract_vocabulary_features(&text);
    assert_eq!(result.common_adjectives.len(), 50);
}

// tests/sentence_analysis_test.rs

#[test]
fn test_avg_sentence_length() {
    let text = "今天天气很好。我们去公园玩。";
    let result = extract_sentence_features(text);
    assert!(result.avg_sentence_length > 0.0);
}

#[test]
fn test_sentence_type_ratio() {
    let text = "今天天气很好？真的吗！当然。";
    let result = extract_sentence_features(text);
    assert!(result.question_ratio > 0.0);
    assert!(result.exclamation_ratio > 0.0);
}
```

### 前端测试（Vitest）

```typescript
// tests/analysis-progress.test.ts

describe('AnalysisProgress', () => {
  it('should display vocabulary analysis step', () => {
    // 测试词汇层步骤显示
  });

  it('should display sentence analysis step', () => {
    // 测试句式层步骤显示
  });

  it('should update progress when layer completes', async () => {
    // 测试进度更新
  });
});
```

---

## UX Design Requirements

### 分析步骤显示

```
正在分析您的风格...

  ✓ 词汇层分析完成
     - 提取 50 个常用形容词
     - 词汇丰富度：0.65
  
  ✓ 句式层分析完成
     - 平均句长：23.5 字
     - 短句比例：35%
  
  → 修辞层分析中... (30%)
  
  ○ 叙事层分析
  ○ 情感层分析
  ○ 节奏层分析
  ○ 对话层分析
  ○ 描写层分析
```

### 进度状态映射

| 状态 | 进度 | 显示 |
|------|------|------|
| pending | 0% | 等待中 |
| vocabulary_completed | 12.5% | 词汇层完成 ✓ |
| sentence_completed | 25% | 句式层完成 ✓ |
| vocabulary_sentence_completed | 25% | 两层都完成 |

---

## Implementation Notes

### 关键实现细节

1. **中文分词选择：**
   - 推荐使用 `jieba-rs`（Rust 版本的 jieba）
   - 支持自定义词典（可扩展武侠/科幻等专业词汇）
   - 支持多模式分词（细粒度/粗粒度）

2. **性能优化：**
   - 大文本分块处理（每 10 万字一块）
   - 并行处理（词汇和句式可以并行）
   - 缓存中间结果（避免重复计算）

3. **编码处理：**
   - 检测文件编码（UTF-8/GBK）
   - 统一转换为 UTF-8 处理
   - 处理 BOM 标记

4. **边界情况：**
   - 空文本处理
   - 纯英文/数字文本
   - 标点符号异常

### 依赖库推荐

```toml
[dependencies]
# 中文分词
jieba-rs = "0.6"

# 正则表达式
regex = "1.10"

# Unicode 处理
unicode-segmentation = "1.10"

# 统计计算
ndarray = "0.15"

# 序列化
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

---

## Story Completion Status

- [ ] 后端：词汇层分析函数实现
- [ ] 后端：句式层分析函数实现
- [ ] 后端：分析结果数据模型
- [ ] 后端：数据库迁移（添加结果存储）
- [ ] 后端：分析进度更新逻辑
- [ ] 后端：查询接口实现
- [ ] 前端：分析步骤显示扩展
- [ ] 测试：词汇分析单元测试
- [ ] 测试：句式分析单元测试
- [ ] 测试：集成测试（上传→分析→查询）

---

**Story created:** 2026-04-09  
**Last updated:** 2026-04-09  
**Status:** ready-for-dev
