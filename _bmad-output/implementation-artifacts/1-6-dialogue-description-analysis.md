---
status: ready-for-dev
epic: 1
story: 6
story_key: 1-6-dialogue-description-analysis
last_updated: 2026-04-09
---

# Story 1.6: 对话层和描写层特征提取

## Story Header

| 属性 | 值 |
|------|-----|
| **Epic** | Epic 1: 项目初始化与风格管理 |
| **Story ID** | 1.6 |
| **Story Key** | 1-6-dialogue-description-analysis |
| **优先级** | P0 |
| **估算复杂度** | 中高 |
| **状态** | ready-for-dev |
| **前置依赖** | Story 1.5 (情感层和节奏层特征提取) |

---

## User Story Statement

**As a** 系统，  
**I want** 提取文本的对话层和描写层特征，  
**So that** 完成七层风格特征分析的最后两层。

---

## Acceptance Criteria (BDD Format)

### AC1: 对话层分析 - 对话比例统计

**Given** 节奏分析完成  
**When** 系统执行对话层分析  
**Then** 统计对话内容占总文本的比例  
**And** 计算对话密度（对话字数/总字数）  
**And** 存储到对话层特征数据结构

### AC2: 角色声音区分度分析

**Given** 对话比例统计完成  
**When** 系统分析角色声音  
**Then** 识别不同角色的对话内容  
**And** 分析各角色对话的词汇/句式差异  
**And** 计算角色声音区分度评分

### AC3: 对话标签使用习惯

**Given** 角色声音分析完成  
**When** 系统分析对话标签  
**Then** 统计对话引导词使用频率（如"道""说""问""喊"）  
**And** 识别无标签对话比例（直接引语无引导）  
**And** 统计副词修饰对话标签频率（如"冷冷地说"）

### AC4: 描写层分析 - 描写比例统计

**Given** 对话分析完成  
**When** 系统执行描写层分析  
**Then** 统计描写性内容占总文本的比例  
**And** 区分环境描写/动作描写/心理描写比例  
**And** 存储到描写层特征数据结构

### AC5: 描写详细程度分析

**Given** 描写比例统计完成  
**When** 系统分析描写详细程度  
**Then** 计算描写的平均粒度（细节密度）  
**And** 识别简洁描写与繁复描写的比例  
**And** 统计修饰词密度（形容词/副词数量）

### AC6: 描写偏好识别

**Given** 描写详细程度分析完成  
**When** 系统分析描写偏好  
**Then** 识别动作描写偏好（打斗/运动场景）  
**And** 识别环境描写偏好（场景/氛围渲染）  
**And** 识别心理描写偏好（内心独白/情感）  
**And** 识别外貌描写偏好（人物肖像）

### AC7: 分析结果存储

**Given** 对话层和描写层分析完成  
**When** 系统存储分析结果  
**Then** 将特征数据追加到 `style_analysis_tasks` 的进度数据中  
**And** 更新任务状态为 `all_layers_completed`  
**And** 进度更新为 75%（七层全部完成，待向量化）

### AC8: 分析失败处理

**Given** 分析过程出错  
**When** 错误发生  
**Then** 记录错误信息到 `style_analysis_tasks.error_message`  
**And** 更新状态为 `partial_failure`  
**And** 保留已完成的中间结果（前六层）

---

## Story Requirements

### 功能需求

| ID | 需求 | 优先级 |
|----|------|--------|
| FR1.6.1 | 系统统计对话内容比例 | P0 |
| FR1.6.2 | 系统分析角色声音区分度 | P1 |
| FR1.6.3 | 系统统计对话标签使用习惯 | P0 |
| FR1.6.4 | 系统统计描写内容比例 | P0 |
| FR1.6.5 | 系统分析描写详细程度 | P0 |
| FR1.6.6 | 系统识别描写偏好类型 | P1 |
| FR1.6.7 | 系统存储分析结果到数据库 | P0 |
| FR1.6.8 | 系统更新分析任务进度到 75% | P0 |

### 非功能需求

| ID | 需求 | 优先级 |
|----|------|--------|
| NFR1.6.1 | 对话识别准确率 > 85% | P0 |
| NFR1.6.2 | 10 万字文本分析时间 < 50 秒 | P0 |
| NFR1.6.3 | 支持多人对话场景分析 | P0 |
| NFR1.6.4 | 内存占用 < 600MB（10 万字） | P1 |

---

## Developer Context Section

### Story Scope Guardrails

**本 Story 必须完成：**
- 对话层特征提取（对话比例/角色声音/对话标签）
- 描写层特征提取（描写比例/详细程度/描写偏好）
- 分析结果追加到 Story 1.5 的结果之后
- 进度更新到 75%（七层全部完成）

**本 Story 明确不做：**
- 128 维风格向量生成（Story 1.7）
- 风格报告展示 UI（Story 1.8）

### 技术栈要求

**后端（Rust）：**
- **框架：** Axum
- **中文分词：** jieba-rs（词性标注辅助）
- **文本处理：** regex, unicode-segmentation
- **数据存储：** SQLite + SQLx

**前端（Vue 3）：**
- 本 Story 主要是后端分析逻辑
- 前端复用已有的进度显示组件

### 架构合规要求

1. **模块化设计** - 对话分析和描写分析应该是独立的可测试函数
2. **流式处理** - 大文本应该分块处理
3. **进度可追踪** - 每一步都应该更新任务进度
4. **结果可累加** - 对话和描写分析结果应该追加到前六层结果之后
5. **为 Story 1.7 做准备** - 七层分析完成后，数据结构应准备好用于向量化

### 核心算法实现

#### 对话层分析

```rust
// services/style_analysis/dialogue_analysis.rs

use regex::Regex;
use std::collections::HashMap;

pub struct DialogueAnalysisResult {
    pub dialogue_ratio: f32,              // 对话内容比例
    pub character_voice_distinction: f32, // 角色声音区分度 (0-1)
    pub dialogue_tag_frequency: f32,      // 对话标签频率（每千字）
    pub untagged_dialogue_ratio: f32,     // 无标签对话比例
    pub adverb_modifier_ratio: f32,       // 副词修饰对话标签比例
    pub avg_dialogue_length: f32,         // 平均对话长度（字）
}

/// 统计对话内容比例
fn calculate_dialogue_ratio(text: &str) -> f32 {
    // 匹配中文引号内的内容
    let chinese_quote_pattern = Regex::new(r"「[^「」]*」|"[^""]*"|'[^']*'").unwrap();
    // 匹配冒号后的对话（如：他道："..."）
    let colon_dialogue_pattern = Regex::new(r"[：][\s]*[「「""]+[^」」""]+[」」""]").unwrap();
    
    let total_chars = text.chars().count() as f32;
    if total_chars == 0.0 {
        return 0.0;
    }
    
    let quoted_chars: usize = chinese_quote_pattern
        .find_iter(text)
        .map(|m| m.as_str().chars().count())
        .sum();
    
    let colon_dialogue_chars: usize = colon_dialogue_pattern
        .find_iter(text)
        .map(|m| m.as_str().chars().count())
        .sum();
    
    // 避免重复计算（冒号对话可能已包含在引号内）
    let dialogue_chars = quoted_chars.max(colon_dialogue_chars);
    
    dialogue_chars as f32 / total_chars
}

/// 分析角色声音区分度
fn analyze_character_voice_distinction(text: &str) -> f32 {
    // 提取角色名称和对应的对话
    let character_dialogues = extract_character_dialogues(text);
    
    if character_dialogues.len() < 2 {
        return 0.5; // 单个角色或无角色，返回中间值
    }
    
    // 计算各角色对话的词汇差异
    let mut voice_profiles: Vec<HashMap<String, usize>> = Vec::new();
    
    for (_character, dialogue) in &character_dialogues {
        let word_freq = extract_word_frequency(dialogue);
        voice_profiles.push(word_freq);
    }
    
    // 计算声音区分度（各角色词汇分布的差异）
    let mut total_distinction = 0.0;
    let comparison_count = voice_profiles.len() * (voice_profiles.len() - 1) / 2;
    
    for i in 0..voice_profiles.len() {
        for j in (i + 1)..voice_profiles.len() {
            let similarity = cosine_similarity(&voice_profiles[i], &voice_profiles[j]);
            total_distinction += 1.0 - similarity; // 差异度 = 1 - 相似度
        }
    }
    
    if comparison_count > 0 {
        total_distinction / comparison_count as f32
    } else {
        0.5
    }
}

fn extract_character_dialogues(text: &str) -> Vec<(String, String)> {
    // 简化：匹配"XXX 道/说/问："后的对话
    let pattern = Regex::new(r"([^\s]{1,4})[道说问喊叫笑道]：[」「""]+([^」」""]+)[」」""]").unwrap();
    
    let mut dialogues: HashMap<String, String> = HashMap::new();
    
    for cap in pattern.captures_iter(text) {
        if let (Some(name), Some(dialogue)) = (cap.get(1), cap.get(2)) {
            let name_str = name.as_str().to_string();
            let dialogue_str = dialogue.as_str().to_string();
            
            dialogues.entry(name_str).or_insert_with(String::new).push_str(&dialogue_str);
        }
    }
    
    dialogues.into_iter().collect()
}

fn extract_word_frequency(text: &str) -> HashMap<String, usize> {
    let jieba = jieba_rs::Jieba::new();
    let words: Vec<&str> = jieba.cut(text, false).collect();
    
    let mut freq: HashMap<String, usize> = HashMap::new();
    for word in words {
        if word.len() > 1 { // 跳过单字
            *freq.entry(word.to_string()).or_insert(0) += 1;
        }
    }
    
    freq
}

fn cosine_similarity(freq1: &HashMap<String, usize>, freq2: &HashMap<String, usize>) -> f32 {
    // 计算余弦相似度
    let mut dot_product = 0.0;
    let mut norm1 = 0.0;
    let mut norm2 = 0.0;
    
    let all_words: std::collections::HashSet<_> = freq1.keys().chain(freq2.keys()).collect();
    
    for word in all_words {
        let v1 = *freq1.get(*word).unwrap_or(&0) as f32;
        let v2 = *freq2.get(*word).unwrap_or(&0) as f32;
        
        dot_product += v1 * v2;
        norm1 += v1 * v1;
        norm2 += v2 * v2;
    }
    
    if norm1 == 0.0 || norm2 == 0.0 {
        return 0.0;
    }
    
    dot_product / (norm1.sqrt() * norm2.sqrt())
}

/// 分析对话标签使用习惯
fn analyze_dialogue_tags(text: &str) -> (f32, f32, f32) {
    let char_count = text.chars().count() as f32 / 1000.0;
    
    // 对话标签词
    let tag_words = ["道", "说", "问", "喊", "叫", "笑", "哭", "叹", "喝", "答", "回", "应"];
    
    // 统计对话标签数量
    let tag_count = count_keyword_occurrences(text, &tag_words);
    let tag_frequency = if char_count > 0.0 {
        tag_count as f32 / char_count
    } else {
        0.0
    };
    
    // 统计无标签对话（只有引号，没有"道/说"等）
    let total_dialogues = Regex::new(r"[「「""][^」」""]+[」」""]").unwrap()
        .find_iter(text)
        .count();
    
    let tagged_dialogues = Regex::new(r"[道说问喊叫笑道][：:]\s*[「「""]").unwrap()
        .find_iter(text)
        .count();
    
    let untagged_ratio = if total_dialogues > 0 {
        (total_dialogues - tagged_dialogues) as f32 / total_dialogues as f32
    } else {
        0.0
    };
    
    // 统计副词修饰（如"冷冷地说"）
    let adverb_pattern = Regex::new(r"[地][说问喊叫道笑]").unwrap();
    let adverb_count = adverb_pattern.find_iter(text).count();
    let adverb_ratio = if tag_count > 0 {
        adverb_count as f32 / tag_count as f32
    } else {
        0.0
    };
    
    (tag_frequency, untagged_ratio, adverb_ratio)
}

fn count_keyword_occurrences(text: &str, keywords: &[&str]) -> usize {
    keywords.iter()
        .map(|&kw| text.matches(kw).count())
        .sum()
}

pub fn extract_dialogue_features(text: &str) -> DialogueAnalysisResult {
    let dialogue_ratio = calculate_dialogue_ratio(text);
    let voice_distinction = analyze_character_voice_distinction(text);
    let (tag_freq, untagged_ratio, adverb_ratio) = analyze_dialogue_tags(text);
    
    // 计算平均对话长度
    let dialogue_pattern = Regex::new(r"[「「""]([^」」""]+)[」」""]").unwrap();
    let dialogues: Vec<&str> = dialogue_pattern
        .captures_iter(text)
        .filter_map(|cap| cap.get(1).map(|m| m.as_str()))
        .collect();
    
    let avg_dialogue_length = if dialogues.is_empty() {
        0.0
    } else {
        dialogues.iter().map(|d| d.chars().count()).sum::<usize>() as f32 / dialogues.len() as f32
    };
    
    DialogueAnalysisResult {
        dialogue_ratio,
        character_voice_distinction: voice_distinction,
        dialogue_tag_frequency: tag_freq,
        untagged_dialogue_ratio: untagged_ratio,
        adverb_modifier_ratio: adverb_ratio,
        avg_dialogue_length,
    }
}
```

#### 描写层分析

```rust
// services/style_analysis/description_analysis.rs

use jieba_rs::Jieba;
use regex::Regex;
use std::collections::HashMap;

pub struct DescriptionAnalysisResult {
    pub description_ratio: f32,           // 描写内容比例
    pub detail_granularity: f32,          // 描写详细程度（细节密度）
    pub modifier_density: f32,            // 修饰词密度（形容词/副词）
    pub action_description_ratio: f32,    // 动作描写比例
    pub environment_description_ratio: f32, // 环境描写比例
    pub psychological_description_ratio: f32, // 心理描写比例
    pub appearance_description_ratio: f32,    // 外貌描写比例
}

/// 统计描写内容比例（与对话相对）
fn calculate_description_ratio(text: &str) -> f32 {
    // 描写比例 ≈ 1 - 对话比例（简化）
    let dialogue_ratio = super::dialogue_analysis::calculate_dialogue_ratio(text);
    1.0 - dialogue_ratio
}

/// 分析描写详细程度
fn analyze_detail_granularity(text: &str) -> f32 {
    let jieba = Jieba::new();
    let words: Vec<&str> = jieba.cut(text, false).collect();
    
    // 统计细节词（具体名词、细节形容词）
    let detail_indicators = ["微微", "缓缓", "轻轻", "仔细", "认真", "专注",
                             "细致", "精细", "清晰", "分明", "层层", "片片"];
    
    let detail_count = count_keyword_occurrences(&words.iter().collect::<String>(), &detail_indicators);
    let total_words = words.len() as f32;
    
    if total_words > 0.0 {
        detail_count as f32 / total_words
    } else {
        0.0
    }
}

/// 分析修饰词密度
fn analyze_modifier_density(text: &str) -> f32 {
    let jieba = Jieba::new();
    
    // 词性标注
    let words_with_pos = jieba.tag(text).unwrap_or_default();
    
    // 统计形容词 (a) 和副词 (d)
    let modifier_count = words_with_pos
        .iter()
        .filter(|(_, pos)| *pos == "a" || *pos == "d" || pos.starts_with("adjective") || pos.starts_with("adverb"))
        .count();
    
    let total_words = words_with_pos.len() as f32;
    if total_words > 0.0 {
        modifier_count as f32 / total_words
    } else {
        0.0
    }
}

/// 分析描写偏好
fn analyze_description_preferences(text: &str) -> (f32, f32, f32, f32) {
    // 动作描写关键词
    let action_words = ["打", "斗", "战", "冲", "跑", "跳", "飞", "掠", "劈", "砍",
                        "闪", "躲", "避", "攻", "守", "跃", "翻", "转", "旋"];
    
    // 环境描写关键词
    let environment_words = ["天", "地", "山", "水", "风", "云", "日", "月", "星", "辰",
                             "花", "草", "树", "木", "江", "河", "湖", "海", "景", "色"];
    
    // 心理描写关键词
    let psychological_words = ["想", "思", "念", "忆", "悟", "忖", "揣", "摩", "觉", "感",
                               "心", "神", "魂", "魄", "意", "志", "情", "绪"];
    
    // 外貌描写关键词
    let appearance_words = ["眉", "眼", "鼻", "口", "耳", "发", "脸", "容", "貌", "身",
                            "形", "姿", "态", "衣", "袍", "衫", "裙", "履", "剑", "刀"];
    
    let action_count = count_keyword_occurrences(text, &action_words);
    let env_count = count_keyword_occurrences(text, &environment_words);
    let psycho_count = count_keyword_occurrences(text, &psychological_words);
    let appear_count = count_keyword_occurrences(text, &appearance_words);
    
    let total = (action_count + env_count + psycho_count + appear_count) as f32;
    
    if total > 0.0 {
        (
            action_count as f32 / total,
            env_count as f32 / total,
            psycho_count as f32 / total,
            appear_count as f32 / total,
        )
    } else {
        (0.25, 0.25, 0.25, 0.25) // 默认平均分布
    }
}

fn count_keyword_occurrences(text: &str, keywords: &[&str]) -> usize {
    keywords.iter()
        .map(|&kw| text.matches(kw).count())
        .sum()
}

pub fn extract_description_features(text: &str) -> DescriptionAnalysisResult {
    let description_ratio = calculate_description_ratio(text);
    let detail_granularity = analyze_detail_granularity(text);
    let modifier_density = analyze_modifier_density(text);
    let (action_ratio, env_ratio, psycho_ratio, appear_ratio) = analyze_description_preferences(text);
    
    DescriptionAnalysisResult {
        description_ratio,
        detail_granularity,
        modifier_density,
        action_description_ratio: action_ratio,
        environment_description_ratio: env_ratio,
        psychological_description_ratio: psycho_ratio,
        appearance_description_ratio: appear_ratio,
    }
}
```

### 数据库 Schema

```sql
-- 更新 style_analysis_tasks 表，添加对话和描写结果存储
ALTER TABLE style_analysis_tasks 
ADD COLUMN dialogue_json TEXT,
ADD COLUMN description_json TEXT;
```

### API 端点扩展

```
# 风格分析进度查询
GET    /api/style-analysis/:id               # 获取任务详情
GET    /api/style-analysis/:id/dialogue      # 获取对话层分析结果
GET    /api/style-analysis/:id/description   # 获取描写层分析结果
```

### 响应格式

```json
{
  "success": true,
  "data": {
    "task_id": "uuid",
    "status": "all_layers_completed",
    "progress": 0.75,
    "vocabulary": { ... },
    "sentence": { ... },
    "rhetoric": { ... },
    "narrative": { ... },
    "emotion": { ... },
    "pacing": { ... },
    "dialogue": {
      "dialogue_ratio": 0.35,
      "character_voice_distinction": 0.72,
      "dialogue_tag_frequency": 15.3,
      "untagged_dialogue_ratio": 0.25,
      "adverb_modifier_ratio": 0.18,
      "avg_dialogue_length": 45.6
    },
    "description": {
      "description_ratio": 0.65,
      "detail_granularity": 0.12,
      "modifier_density": 0.15,
      "action_description_ratio": 0.30,
      "environment_description_ratio": 0.25,
      "psychological_description_ratio": 0.20,
      "appearance_description_ratio": 0.25
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
│       ├── vocabulary_analysis.rs     # 词汇层分析（Story 1.3）
│       ├── sentence_analysis.rs       # 句式层分析（Story 1.3）
│       ├── rhetoric_analysis.rs       # 修辞层分析（Story 1.4）
│       ├── narrative_analysis.rs      # 叙事层分析（Story 1.4）
│       ├── emotion_analysis.rs        # 情感层分析（Story 1.5）
│       ├── pacing_analysis.rs         # 节奏层分析（Story 1.5）
│       ├── dialogue_analysis.rs       # 对话层分析（NEW）
│       ├── description_analysis.rs    # 描写层分析（NEW）
│       └── style_analyzer.rs          # 统一分析入口
├── models/
│   └── style_features.rs              # 特征数据结构（扩展对话/描写结构）
├── handlers/
│   └── style_analysis.rs              # 分析结果查询接口（扩展）
└── db/
    └── migrations/
        └── 006_add_dialogue_description_results.sql
```

---

## Testing Requirements

### 后端测试（Rust）

```rust
// tests/dialogue_analysis_test.rs

#[test]
fn test_dialogue_ratio_calculation() {
    let text = "他道：「你好。」她回答：「好久不见。」";
    let ratio = calculate_dialogue_ratio(text);
    assert!(ratio > 0.0 && ratio < 1.0);
}

#[test]
fn test_character_voice_distinction() {
    let text = "张三道：「咱们走吧。」李四道：「我再待会儿。」";
    let distinction = analyze_character_voice_distinction(text);
    assert!(distinction >= 0.0 && distinction <= 1.0);
}

#[test]
fn test_dialogue_tag_frequency() {
    let text = "他道：'...'。她说：'...'。他们问：'...'。";
    let (tag_freq, _, _) = analyze_dialogue_tags(text);
    assert!(tag_freq > 0.0);
}

// tests/description_analysis_test.rs

#[test]
fn test_description_ratio() {
    let text = "天空湛蓝，白云飘飘。他静静地站着。";
    let ratio = calculate_description_ratio(text);
    assert!(ratio > 0.0);
}

#[test]
fn test_modifier_density() {
    let text = "美丽的花朵在微风中轻轻地摇曳。";
    let density = analyze_modifier_density(text);
    assert!(density > 0.0);
}

#[test]
fn test_description_preferences() {
    let text = "他打斗起来，动作迅猛。周围环境优美，山水如画。";
    let (action, env, psycho, appear) = analyze_description_preferences(text);
    assert!(action > 0.0 || env > 0.0);
}
```

---

## Story Completion Status

- [ ] 后端：对话层分析函数实现
- [ ] 后端：描写层分析函数实现
- [ ] 后端：分析结果数据模型扩展
- [ ] 后端：数据库迁移（添加对话/描写结果存储）
- [ ] 后端：分析进度更新逻辑（追加到 Story 1.5 结果）
- [ ] 前端：分析步骤显示扩展（增加对话/描写步骤）
- [ ] 测试：对话分析单元测试
- [ ] 测试：描写分析单元测试
- [ ] 测试：集成测试（验证 75% 进度）

---

**Story created:** 2026-04-09  
**Last updated:** 2026-04-09  
**Status:** ready-for-dev
