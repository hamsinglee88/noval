---
status: ready-for-dev
epic: 1
story: 5
story_key: 1-5-emotion-rhythm-analysis
last_updated: 2026-04-09
---

# Story 1.5: 情感层和节奏层特征提取

## Story Header

| 属性 | 值 |
|------|-----|
| **Epic** | Epic 1: 项目初始化与风格管理 |
| **Story ID** | 1.5 |
| **Story Key** | 1-5-emotion-rhythm-analysis |
| **优先级** | P0 |
| **估算复杂度** | 中高 |
| **状态** | ready-for-dev |
| **前置依赖** | Story 1.4 (修辞层和叙事层特征提取) |

---

## User Story Statement

**As a** 系统，  
**I want** 提取文本的情感层和节奏层特征，  
**So that** 理解用户的情感表达和节奏控制风格。

---

## Acceptance Criteria (BDD Format)

### AC1: 情感层分析 - 整体情感基调识别

**Given** 叙事分析完成  
**When** 系统执行情感层分析  
**Then** 识别整体情感基调（如：史诗感/压抑/轻松/悲伤/激昂）  
**And** 计算情感基调的置信度评分  
**And** 存储到情感层特征数据结构

### AC2: 情感波动幅度计算

**Given** 情感基调识别完成  
**When** 系统分析情感波动  
**Then** 计算文本各段落之间的情感变化幅度  
**And** 统计情感波动频率（单位章节内情感变化次数）  
**And** 存储情感波动指标

### AC3: 情感表达方式分析

**Given** 情感波动分析完成  
**When** 系统分析情感表达方式  
**Then** 统计直接情感表达词频率（如"悲伤""喜悦"）  
**And** 统计间接情感表达频率（通过动作/环境描写暗示）  
**And** 计算直接/间接表达比例

### AC4: 节奏层分析 - 章节平均长度

**Given** 情感分析完成  
**When** 系统执行节奏层分析  
**Then** 统计章节平均字数  
**And** 计算章节长度标准差（衡量章节长度变化）  
**And** 识别最长章节和最短章节

### AC5: 场景切换频率分析

**Given** 章节长度分析完成  
**When** 系统分析场景切换  
**Then** 识别场景转换标记（时间/地点变化）  
**And** 统计单位字数内场景切换次数  
**And** 计算场景切换频率（每千字切换次数）

### AC6: 悬念结尾使用频率

**Given** 场景切换分析完成  
**When** 系统分析章节结尾  
**Then** 识别悬念结尾模式（未解疑问/突然中断/危机时刻）  
**Then** 统计悬念结尾使用频率（悬念结尾数/总章节数）  
**And** 存储到节奏层特征数据结构

### AC7: 段落节奏分析

**Given** 悬念结尾分析完成  
**When** 系统分析段落节奏  
**Then** 统计段落平均长度（句数/字数）  
**And** 识别短段落密集使用区域（动作/紧张场景）  
**And** 识别长段落密集使用区域（描写/抒情场景）

### AC8: 分析结果存储

**Given** 情感层和节奏层分析完成  
**When** 系统存储分析结果  
**Then** 将特征数据追加到 `style_analysis_tasks` 的进度数据中  
**And** 更新任务状态为 `emotion_rhythm_completed`  
**And** 进度更新为 62.5%（七层中的五层完成）

### AC9: 分析失败处理

**Given** 分析过程出错  
**When** 错误发生  
**Then** 记录错误信息到 `style_analysis_tasks.error_message`  
**And** 更新状态为 `partial_failure`  
**And** 保留已完成的中间结果（词汇/句式/修辞/叙事）

---

## Story Requirements

### 功能需求

| ID | 需求 | 优先级 |
|----|------|--------|
| FR1.5.1 | 系统识别整体情感基调类型 | P0 |
| FR1.5.2 | 系统计算情感波动幅度和频率 | P0 |
| FR1.5.3 | 系统分析直接/间接情感表达比例 | P1 |
| FR1.5.4 | 系统统计章节平均长度和变化 | P0 |
| FR1.5.5 | 系统识别场景切换频率 | P0 |
| FR1.5.6 | 系统统计悬念结尾使用频率 | P1 |
| FR1.5.7 | 系统分析段落节奏特征 | P1 |
| FR1.5.8 | 系统存储分析结果到数据库 | P0 |
| FR1.5.9 | 系统更新分析任务进度到 62.5% | P0 |

### 非功能需求

| ID | 需求 | 优先级 |
|----|------|--------|
| NFR1.5.1 | 情感基调识别准确率 > 75% | P0 |
| NFR1.5.2 | 10 万字文本分析时间 < 50 秒 | P0 |
| NFR1.5.3 | 支持多情感基调混合的文本 | P0 |
| NFR1.5.4 | 内存占用 < 600MB（10 万字） | P1 |

---

## Developer Context Section

### Story Scope Guardrails

**本 Story 必须完成：**
- 情感层特征提取（情感基调/波动幅度/表达方式）
- 节奏层特征提取（章节长度/场景切换/悬念结尾/段落节奏）
- 分析结果追加到 Story 1.4 的结果之后
- 进度更新到 62.5%

**本 Story 明确不做：**
- 对话层、描写层分析
- 128 维风格向量生成
- 风格报告展示 UI

### 技术栈要求

**后端（Rust）：**
- **框架：** Axum
- **中文分词：** jieba-rs（情感词识别）
- **文本处理：** regex, unicode-segmentation
- **数据存储：** SQLite + SQLx
- **情感分析：** 可考虑引入情感词典或简单规则引擎

**前端（Vue 3）：**
- 本 Story 主要是后端分析逻辑
- 前端复用 Story 1.2/1.3/1.4 的进度显示组件

### 架构合规要求

1. **模块化设计** - 情感分析和节奏分析应该是独立的可测试函数
2. **流式处理** - 大文本应该分块处理，避免一次性加载到内存
3. **进度可追踪** - 每一步都应该更新任务进度
4. **结果可累加** - 情感和节奏分析结果应该追加到前四层结果之后
5. **与 Story 1.3/1.4 共享数据结构** - 使用统一的 `StyleAnalysisProgress` 结构

### 核心算法实现

#### 情感层分析

```rust
// services/style_analysis/emotion_analysis.rs

use std::collections::HashMap;

pub struct EmotionAnalysisResult {
    pub overall_tone: String,           // 整体情感基调
    pub tone_confidence: f32,           // 基调置信度
    pub emotional_amplitude: f32,       // 情感波动幅度
    pub emotional_frequency: f32,       // 情感波动频率（每千字）
    pub direct_expression_ratio: f32,   // 直接情感表达比例
    pub indirect_expression_ratio: f32, // 间接情感表达比例
}

/// 情感基调分类
fn classify_emotional_tone(text: &str) -> (String, f32) {
    // 情感词词典（简化版）
    let epic_words = ["史诗", "宏伟", "壮丽", "浩瀚", "惊天动地", "气吞山河"];
    let depressed_words = ["压抑", "沉重", "悲伤", "绝望", "痛苦", "凄凉"];
    let relaxed_words = ["轻松", "愉快", "欢乐", "惬意", "悠闲", "舒畅"];
    let intense_words = ["激昂", "紧张", "激烈", "震撼", "紧迫", "危机"];
    let sad_words = ["悲伤", "哀伤", "悲痛", "伤心", "落泪", "哭泣"];
    
    let mut emotion_scores: HashMap<&str, usize> = HashMap::new();
    emotion_scores.insert("epic", count_keyword_occurrences(text, &epic_words));
    emotion_scores.insert("depressed", count_keyword_occurrences(text, &depressed_words));
    emotion_scores.insert("relaxed", count_keyword_occurrences(text, &relaxed_words));
    emotion_scores.insert("intense", count_keyword_occurrences(text, &intense_words));
    emotion_scores.insert("sad", count_keyword_occurrences(text, &sad_words));
    
    // 找出得分最高的情感
    let max_emotion = emotion_scores.iter()
        .max_by_key(|&(_, v)| v)
        .unwrap_or(&&("neutral", &0));
    
    let total_emotions: usize = emotion_scores.values().sum();
    let confidence = if total_emotions > 0 {
        *max_emotion.1 as f32 / total_emotions as f32
    } else {
        0.2 // 默认低置信度
    };
    
    let tone_name = match max_emotion.0 {
        &"epic" => "史诗感",
        &"depressed" => "压抑",
        &"relaxed" => "轻松",
        &"intense" => "激昂",
        &"sad" => "悲伤",
        _ => "中性",
    };
    
    (tone_name.to_string(), confidence)
}

/// 分析情感波动幅度
fn analyze_emotional_amplitude(text: &str) -> f32 {
    // 将文本分段（每 1000 字一段）
    let paragraphs = split_into_paragraphs(text, 1000);
    
    // 计算每段的情感得分
    let paragraph_scores: Vec<f32> = paragraphs
        .iter()
        .map(|p| calculate_paragraph_emotion_score(p))
        .collect();
    
    // 计算情感得分的标准差作为波动幅度
    if paragraph_scores.is_empty() {
        return 0.0;
    }
    
    let mean = paragraph_scores.iter().sum::<f32>() / paragraph_scores.len() as f32;
    let variance = paragraph_scores.iter()
        .map(|&s| (s - mean).powi(2))
        .sum::<f32>() / paragraph_scores.len() as f32;
    
    variance.sqrt()
}

fn calculate_paragraph_emotion_score(paragraph: &str) -> f32 {
    // 简化：统计情感词数量作为情感强度
    let emotion_words = ["喜", "怒", "哀", "乐", "悲", "惊", "恐", "惧", 
                         "爱", "恨", "痛", "痒", "冷", "热", "温", "凉"];
    count_keyword_occurrences(paragraph, &emotion_words) as f32
}

/// 分析直接/间接情感表达比例
fn analyze_expression_types(text: &str) -> (f32, f32) {
    // 直接情感词
    let direct_emotion_words = ["悲伤", "喜悦", "愤怒", "恐惧", "惊讶", "厌恶",
                                "高兴", "难过", "激动", "平静"];
    
    // 间接情感表达标记（动作/环境）
    let indirect_markers = ["嘴角", "眼神", "眉头", "拳头", "颤抖", "紧握",
                           "寒风", "烈日", "细雨", "狂风"];
    
    let direct_count = count_keyword_occurrences(text, &direct_emotion_words);
    let indirect_count = count_keyword_occurrences(text, &indirect_markers);
    
    let total = (direct_count + indirect_count) as f32;
    if total > 0.0 {
        (direct_count as f32 / total, indirect_count as f32 / total)
    } else {
        (0.5, 0.5) // 默认中间值
    }
}

pub fn extract_emotion_features(text: &str) -> EmotionAnalysisResult {
    let (overall_tone, tone_confidence) = classify_emotional_tone(text);
    let emotional_amplitude = analyze_emotional_amplitude(text);
    let emotional_frequency = emotional_amplitude * 0.5; // 简化：与波动幅度相关
    let (direct_ratio, indirect_ratio) = analyze_expression_types(text);
    
    EmotionAnalysisResult {
        overall_tone,
        tone_confidence,
        emotional_amplitude,
        emotional_frequency,
        direct_expression_ratio: direct_ratio,
        indirect_expression_ratio: indirect_ratio,
    }
}

fn split_into_paragraphs(text: &str, target_size: usize) -> Vec<String> {
    // 按段落分割，每段约 target_size 字
    let mut paragraphs = Vec::new();
    let mut current = String::new();
    
    for line in text.lines() {
        if current.chars().count() + line.chars().count() > target_size {
            if !current.is_empty() {
                paragraphs.push(current.clone());
                current.clear();
            }
        }
        current.push_str(line);
        current.push('\n');
    }
    
    if !current.is_empty() {
        paragraphs.push(current);
    }
    
    paragraphs
}

fn count_keyword_occurrences(text: &str, keywords: &[&str]) -> usize {
    keywords.iter()
        .map(|&kw| text.matches(kw).count())
        .sum()
}
```

#### 节奏层分析

```rust
// services/style_analysis/pacing_analysis.rs

use regex::Regex;

pub struct PacingAnalysisResult {
    pub avg_chapter_length: f32,       // 章节平均字数
    pub chapter_length_variance: f32,  // 章节长度标准差
    pub scene_transition_frequency: f32, // 场景切换频率（每千字）
    pub cliffhanger_ratio: f32,        // 悬念结尾比例
    pub avg_paragraph_length: f32,     // 段落平均长度（字）
    pub short_paragraph_density: f32,  // 短段落密集度
    pub long_paragraph_density: f32,   // 长段落密集度
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
        r"(次日|第二天|翌日|数日后|数月后|数年后)",  // 时间切换
        r"(与此同时|另一边|此时在|镜头转向)",      // 空间切换
        r"(话说|且说|却说|花开两朵)",            // 叙事切换
        r"(chapter|章 | 节|回)",                  // 章节标记
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
        r"(突然 | 忽然|猛地|骤然)",          // 突发事件
        r"(却不知 | 殊不知|然而 | 但是)",   // 转折
        r"(难道 | 莫非|究竟 | 到底)",        // 疑问
        r"(等着瞧 | 未完待续 | 待续)",     // 明示继续
        r"(危机 | 危险|致命 | 绝境)",        // 危机时刻
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
    let short_density = short_count as f32 / paragraph_lengths.len() as f32;
    
    // 长段落（>300 字）密度
    let long_count = paragraph_lengths.iter().filter(|&&l| l > 300).count();
    let long_density = long_count as f32 / paragraph_lengths.len() as f32;
    
    (avg_length, short_density, long_density)
}

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
```

### 数据库 Schema

```sql
-- 更新 style_analysis_tasks 表，添加情感和节奏结果存储
ALTER TABLE style_analysis_tasks 
ADD COLUMN emotion_json TEXT,
ADD COLUMN pacing_json TEXT;
```

### API 端点扩展

```
# 风格分析进度查询
GET    /api/style-analysis/:id           # 获取任务详情
GET    /api/style-analysis/:id/emotion   # 获取情感层分析结果
GET    /api/style-analysis/:id/pacing    # 获取节奏层分析结果
```

### 响应格式

```json
{
  "success": true,
  "data": {
    "task_id": "uuid",
    "status": "emotion_rhythm_completed",
    "progress": 0.625,
    "vocabulary": { ... },
    "sentence": { ... },
    "rhetoric": { ... },
    "narrative": { ... },
    "emotion": {
      "overall_tone": "史诗感",
      "tone_confidence": 0.78,
      "emotional_amplitude": 0.45,
      "emotional_frequency": 0.23,
      "direct_expression_ratio": 0.35,
      "indirect_expression_ratio": 0.65
    },
    "pacing": {
      "avg_chapter_length": 4500.0,
      "chapter_length_variance": 800.5,
      "scene_transition_frequency": 3.2,
      "cliffhanger_ratio": 0.65,
      "avg_paragraph_length": 120.5,
      "short_paragraph_density": 0.25,
      "long_paragraph_density": 0.15
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
│       ├── emotion_analysis.rs        # 情感层分析（NEW）
│       ├── pacing_analysis.rs         # 节奏层分析（NEW）
│       └── style_analyzer.rs          # 统一分析入口
├── models/
│   └── style_features.rs              # 特征数据结构（扩展情感/节奏结构）
├── handlers/
│   └── style_analysis.rs              # 分析结果查询接口（扩展）
└── db/
    └── migrations/
        └── 005_add_emotion_pacing_results.sql
```

---

## Testing Requirements

### 后端测试（Rust）

```rust
// tests/emotion_analysis_test.rs

#[test]
fn test_classify_emotional_tone_epic() {
    let text = "史诗般的战斗场面，气吞山河的壮丽景象...";
    let (tone, confidence) = classify_emotional_tone(text);
    assert_eq!(tone, "史诗感");
    assert!(confidence > 0.5);
}

#[test]
fn test_emotional_amplitude_calculation() {
    let text = "他很高兴。然后非常悲伤。接着又兴奋起来。";
    let amplitude = analyze_emotional_amplitude(text);
    assert!(amplitude > 0.0);
}

// tests/pacing_analysis_test.rs

#[test]
fn test_avg_chapter_length() {
    let chapters = ["第一章：开头".repeat(100), "第二章：发展".repeat(150)];
    let (avg, variance) = analyze_chapter_lengths(&chapters);
    assert!(avg > 0.0);
}

#[test]
fn test_scene_transition_detection() {
    let text = "第二天，他来到了京城。与此同时，另一边...";
    let transitions = detect_scene_transitions(text);
    assert!(transitions > 0);
}

#[test]
fn test_cliffhanger_detection() {
    let chapter_endings = "突然，一个黑影闪过。他面临着前所未有的危机！";
    let chapters = [chapter_endings];
    let count = detect_cliffhanger_endings(&chapters);
    assert!(count > 0);
}
```

---

## Story Completion Status

- [ ] 后端：情感层分析函数实现
- [ ] 后端：节奏层分析函数实现
- [ ] 后端：分析结果数据模型扩展
- [ ] 后端：数据库迁移（添加情感/节奏结果存储）
- [ ] 后端：分析进度更新逻辑（追加到 Story 1.4 结果）
- [ ] 前端：分析步骤显示扩展（增加情感/节奏步骤）
- [ ] 测试：情感分析单元测试
- [ ] 测试：节奏分析单元测试
- [ ] 测试：集成测试（验证 62.5% 进度）

---

**Story created:** 2026-04-09  
**Last updated:** 2026-04-09  
**Status:** ready-for-dev
