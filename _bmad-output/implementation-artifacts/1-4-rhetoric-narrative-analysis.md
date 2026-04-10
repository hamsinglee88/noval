---
status: done
epic: 1
story: 4
story_key: 1-4-rhetoric-narrative-analysis
last_updated: 2026-04-10
---

# Story 1.4: 修辞层和叙事层特征提取

## Story Header

| 属性 | 值 |
|------|-----|
| **Epic** | Epic 1: 项目初始化与风格管理 |
| **Story ID** | 1.4 |
| **Story Key** | 1-4-rhetoric-narrative-analysis |
| **优先级** | P0 |
| **估算复杂度** | 中高 |
| **状态** | ready-for-dev |
| **前置依赖** | Story 1.3 (词汇层和句式层特征提取) |

---

## User Story Statement

**As a** 系统，  
**I want** 提取文本的修辞层和叙事层特征，  
**So that** 捕捉用户的修辞和叙事风格。

---

## Acceptance Criteria (BDD Format)

### AC1: 修辞层分析 - 隐喻和明喻识别

**Given** 句式分析完成  
**When** 系统执行修辞层分析  
**Then** 识别隐喻（暗喻）使用频率（每千字出现次数）  
**And** 识别明喻（使用"像""如同"等比喻词）使用频率  
**And** 存储到修辞层特征数据结构

### AC2: 排比和对偶识别

**Given** 修辞分析进行中  
**When** 系统检测排比句式  
**Then** 识别连续三个或以上结构相似的句子/短语  
**And** 统计排比使用频率  
**And** 识别对偶句（结构对称、字数相近）

### AC3: 感官细节偏好分析

**Given** 修辞特征提取中  
**When** 系统分析感官细节  
**Then** 统计视觉描写频率（颜色、光影、形态相关词）  
**And** 统计听觉描写频率（声音相关词）  
**And** 统计触觉/嗅觉/味觉描写频率  
**And** 计算感官偏好排序（如：视觉>听觉>触觉）

### AC4: 叙事视角识别

**Given** 修辞分析完成  
**When** 系统执行叙事层分析  
**Then** 识别叙事视角类型（第一人称/第三人称限知/第三人称全知）  
**And** 统计视角一致性（视角切换频率）  
**And** 存储到叙事层特征数据结构

### AC5: 展示/讲述比例分析

**Given** 叙事分析进行中  
**When** 系统分析展示与讲述  
**Then** 识别展示性文字（Show：动作、对话、细节描写）  
**And** 识别讲述性文字（Tell：概述、说明、心理分析）  
**And** 计算 Show vs Tell 比例

### AC6: 信息密度分析

**Given** 叙事分析进行中  
**When** 系统计算信息密度  
**Then** 统计实体密度（人名、地名、组织名数量）  
**And** 统计情节信息量（动作/事件密度）  
**And** 统计描写密度（形容词/副词密度）

### AC7: 分析结果存储

**Given** 修辞层和叙事层分析完成  
**When** 系统存储分析结果  
**Then** 将特征数据保存到 `style_analysis_tasks.progress_data`  
**And** 更新任务状态为 `rhetoric_narrative_completed`  
**And** 进度更新为 50%（七层中的四层完成）

### AC8: 分析失败处理

**Given** 分析过程出错  
**When** 错误发生  
**Then** 记录错误信息到 `style_analysis_tasks.error_message`  
**And** 更新状态为 `partial_failure`（如词汇句式已完成）  
**And** 保留已完成的中间结果

---

## Story Requirements

### 功能需求

| ID | 需求 | 优先级 |
|----|------|--------|
| FR1.4.1 | 系统识别隐喻/明喻频率 | P0 |
| FR1.4.2 | 系统识别排比/对偶句式 | P1 |
| FR1.4.3 | 系统统计感官细节偏好 | P0 |
| FR1.4.4 | 系统识别叙事视角类型 | P0 |
| FR1.4.5 | 系统计算 Show vs Tell 比例 | P0 |
| FR1.4.6 | 系统计算信息密度指标 | P1 |
| FR1.4.7 | 系统存储分析结果到数据库 | P0 |
| FR1.4.8 | 系统更新分析任务进度 | P0 |

### 非功能需求

| ID | 需求 | 优先级 |
|----|------|--------|
| NFR1.4.1 | 修辞识别准确率 > 80% | P0 |
| NFR1.4.2 | 10 万字文本分析时间 < 45 秒 | P0 |
| NFR1.4.3 | 支持叙事视角混合的文本 | P0 |
| NFR1.4.4 | 内存占用 < 600MB（10 万字） | P1 |

---

## Developer Context Section

### Story Scope Guardrails

**本 Story 必须完成：**
- 修辞层特征提取（隐喻/明喻/排比/感官细节）
- 叙事层特征提取（视角/Show vs Tell/信息密度）
- 分析结果持久化到数据库
- 进度更新机制（累加 Story 1.3 的结果）

**本 Story 明确不做：**
- 情感层、节奏层、对话层、描写层分析
- 128 维风格向量生成
- 风格报告展示 UI

### 技术栈要求

**后端（Rust）：**
- **框架：** Axum
- **中文分词：** jieba-rs（词性标注辅助修辞识别）
- **文本处理：** regex, unicode-segmentation
- **数据存储：** SQLite + SQLx
- **NLP 辅助：** 可考虑引入简单的规则引擎

**前端（Vue 3）：**
- 本 Story 主要是后端分析逻辑
- 前端只需要显示分析进度（复用 Story 1.2/1.3 的进度组件）

### 架构合规要求

1. **模块化设计** - 修辞分析和叙事分析应该是独立的可测试函数
2. **流式处理** - 大文本应该分块处理，避免一次性加载到内存
3. **进度可追踪** - 每一步都应该更新任务进度
4. **错误可恢复** - 分析失败应该保留已完成的中间结果
5. **结果可累加** - 修辞和叙事分析结果应该追加到词汇/句式结果之后

### 核心算法实现

#### 修辞层分析

```rust
// services/style_analysis/rhetoric_analysis.rs

use jieba_rs::Jieba;
use regex::Regex;
use std::collections::HashMap;

pub struct RhetoricAnalysisResult {
    pub metaphor_frequency: f32,        // 隐喻频率（每千字）
    pub simile_frequency: f32,          // 明喻频率（每千字）
    pub parallelism_frequency: f32,     // 排比频率
    pub sensory_preferences: HashMap<String, f32>, // 感官偏好
    pub total_rhetoric_count: usize,
}

/// 识别明喻（使用比喻词）
fn detect_similes(text: &str) -> Vec<&str> {
    // 中文常见比喻词
    let simile_pattern = Regex::new(
        r"(像 | 如同 | 仿佛 | 好似 | 犹如|宛如|像...一样 | 如同...一般)"
    ).unwrap();
    
    simile_pattern
        .find_iter(text)
        .map(|m| m.as_str())
        .collect()
}

/// 识别隐喻（暗喻，使用"是""成为"等）
fn detect_metaphors(text: &str) -> Vec<&str> {
    // 暗喻常见标记词
    let metaphor_pattern = Regex::new(
        r"(是 | 成为 | 变成 | 化作|成了)"
    ).unwrap();
    
    // 简单检测：A 是 B 结构（需要上下文理解，这里用简化版）
    metaphor_pattern
        .find_iter(text)
        .map(|m| m.as_str())
        .collect()
}

/// 识别排比句式
fn detect_parallelism(text: &str) -> usize {
    // 分句
    let sentences: Vec<&str> = text
        .split(&['.', '!', '?', ';', '…'][..])
        .filter(|s| !s.trim().is_empty())
        .collect();
    
    let mut parallelism_count = 0;
    let mut i = 0;
    
    while i + 2 < sentences.len() {
        // 检测连续三句是否结构相似（简化：检查句首词是否相似）
        let s1 = sentences[i].trim();
        let s2 = sentences[i + 1].trim();
        let s3 = sentences[i + 2].trim();
        
        // 简单启发式：检查是否有相同的前缀
        if s1.starts_with(&s2[..s2.len().min(3).max(1)]) 
            && s2.starts_with(&s3[..s3.len().min(3).max(1)]) {
            parallelism_count += 1;
            i += 3;
        } else {
            i += 1;
        }
    }
    
    parallelism_count
}

/// 分析感官细节偏好
fn analyze_sensory_details(text: &str) -> HashMap<String, f32> {
    let mut sensory_counts: HashMap<String, usize> = HashMap::new();
    
    // 视觉相关词
    let visual_words = ["红", "蓝", "绿", "黄", "白", "黑", "亮", "暗", "光", "影", 
                        "色", "彩", "明", "昏", "耀", "闪"];
    // 听觉相关词
    let auditory_words = ["声", "响", "听", "闻", "音", "鸣", "吼", "啸", "唱", "语",
                          "喧", "寂", "静", "噪"];
    // 触觉相关词
    let tactile_words = ["触", "摸", "感", "温", "冷", "热", "暖", "凉", "硬", "软",
                         "滑", "粗", "细", "痛", "痒"];
    // 嗅觉相关词
    let olfactory_words = ["嗅", "闻", "香", "臭", "味", "气", "息", "芳", "馨"];
    // 味觉相关词
    let gustatory_words = ["尝", "品", "味", "甜", "酸", "苦", "辣", "咸", "涩"];
    
    sensory_counts.insert("visual".to_string(), count_keyword_occurrences(text, &visual_words));
    sensory_counts.insert("auditory".to_string(), count_keyword_occurrences(text, &auditory_words));
    sensory_counts.insert("tactile".to_string(), count_keyword_occurrences(text, &tactile_words));
    sensory_counts.insert("olfactory".to_string(), count_keyword_occurrences(text, &olfactory_words));
    sensory_counts.insert("gustatory".to_string(), count_keyword_occurrences(text, &gustatory_words));
    
    // 计算频率（每千字）
    let char_count = text.chars().count() as f32 / 1000.0;
    if char_count > 0.0 {
        for (_, count) in sensory_counts.iter_mut() {
            *count = (*count as f32 / char_count) as usize;
        }
    }
    
    // 转换为 f32 频率
    sensory_counts.iter()
        .map(|(k, v)| (k.clone(), *v as f32))
        .collect()
}

fn count_keyword_occurrences(text: &str, keywords: &[&str]) -> usize {
    keywords.iter()
        .map(|&kw| text.matches(kw).count())
        .sum()
}

pub fn extract_rhetoric_features(text: &str) -> RhetoricAnalysisResult {
    let similes = detect_similes(text);
    let metaphors = detect_metaphors(text);
    let parallelism_count = detect_parallelism(text);
    let sensory_prefs = analyze_sensory_details(text);
    
    let char_count = text.chars().count() as f32 / 1000.0;
    
    RhetoricAnalysisResult {
        metaphor_frequency: metaphors.len() as f32 / char_count.max(1.0),
        simile_frequency: similes.len() as f32 / char_count.max(1.0),
        parallelism_frequency: parallelism_count as f32 / char_count.max(1.0),
        sensory_preferences: sensory_prefs,
        total_rhetoric_count: similes.len() + metaphors.len() + parallelism_count,
    }
}
```

#### 叙事层分析

```rust
// services/style_analysis/narrative_analysis.rs

use regex::Regex;

pub struct NarrativeAnalysisResult {
    pub pov_type: String,                 // 第一人称/第三人称限知/第三人称全知
    pub pov_consistency: f32,             // 视角一致性（视角切换频率的倒数）
    pub show_vs_tell_ratio: f32,          // Show vs Tell 比例 (0-1, 1 表示纯 Show)
    pub entity_density: f32,              // 实体密度（人名/地名/组织名每千字）
    pub action_density: f32,              // 动作密度（动词密度）
    pub description_density: f32,         // 描写密度（形容词/副词密度）
}

/// 识别叙事视角
fn identify_pov(text: &str) -> (String, f32) {
    // 第一人称标记
    let first_person_pattern = Regex::new(r"[我 咱们]").unwrap();
    // 第三人称标记
    let third_person_pattern = Regex::new(r"[他 她 它](?:们)?").unwrap();
    
    let first_person_count = first_person_pattern.find_iter(text).count();
    let third_person_count = third_person_pattern.find_iter(text).count();
    
    // 简单判断：第一人称代词多→第一人称；第三人称代词多→第三人称
    let (pov_type, consistency) = if first_person_count > third_person_count * 2 {
        ("第一人称".to_string(), 1.0) // 假设视角一致
    } else if third_person_count > first_person_count * 2 {
        // 进一步判断是限知还是全知（简化：检测是否有心理活动描写）
        let omniscient_pattern = Regex::new(r"(心想 | 暗道 | 思忖 | 揣测 | 料想)").unwrap();
        if omniscient_pattern.find_iter(text).count() > 5 {
            ("第三人称全知".to_string(), 0.9)
        } else {
            ("第三人称限知".to_string(), 0.95)
        }
    } else {
        ("混合视角".to_string(), 0.7) // 视角切换较频繁
    };
    
    (pov_type, consistency)
}

/// 分析 Show vs Tell 比例
fn analyze_show_vs_tell(text: &str) -> f32 {
    // 分句
    let sentences: Vec<&str> = text
        .split(&['.', '!', '?', ';', '…'][..])
        .filter(|s| !s.trim().is_empty())
        .collect();
    
    let mut show_count = 0;
    let mut tell_count = 0;
    
    // Show 标记：动作、对话、细节描写
    let show_pattern = Regex::new(r"(道 | 说 | 问 | 喊 | 叫 | 笑 | 哭 | 走 | 跑 | 跳 | 看 | 听 | 摸)").unwrap();
    // Tell 标记：概述、说明、评价
    let tell_pattern = Regex::new(r"(是 | 有 | 在 | 很 | 非常 | 十分 | 极其 | 确实 | 显然)").unwrap();
    
    for sentence in &sentences {
        let show_matches = show_pattern.find_iter(sentence).count();
        let tell_matches = tell_pattern.find_iter(sentence).count();
        
        if show_matches > tell_matches {
            show_count += 1;
        } else if tell_matches > show_matches {
            tell_count += 1;
        }
    }
    
    let total = (show_count + tell_count) as f32;
    if total > 0.0 {
        show_count as f32 / total
    } else {
        0.5 // 默认中间值
    }
}

/// 计算信息密度
fn calculate_information_density(text: &str) -> (f32, f32, f32) {
    let char_count = text.chars().count() as f32 / 1000.0;
    
    // 实体密度（简化：统计常见人名/地名标记）
    let entity_pattern = Regex::new(r"[A-Z][a-z]{2,}|[人名地名模式]").unwrap();
    let entity_count = entity_pattern.find_iter(text).count();
    let entity_density = entity_count as f32 / char_count.max(1.0);
    
    // 动作密度（动词密度）
    let action_pattern = Regex::new(r"[动词模式]").unwrap(); // 需要词性标注辅助
    let action_count = action_pattern.find_iter(text).count();
    let action_density = action_count as f32 / char_count.max(1.0);
    
    // 描写密度（形容词/副词密度）
    let adj_adv_pattern = Regex::new(r"[形容词/副词模式]").unwrap();
    let adj_adv_count = adj_adv_pattern.find_iter(text).count();
    let description_density = adj_adv_count as f32 / char_count.max(1.0);
    
    (entity_density, action_density, description_density)
}

pub fn extract_narrative_features(text: &str) -> NarrativeAnalysisResult {
    let (pov_type, pov_consistency) = identify_pov(text);
    let show_vs_tell = analyze_show_vs_tell(text);
    let (entity_density, action_density, description_density) = calculate_information_density(text);
    
    NarrativeAnalysisResult {
        pov_type,
        pov_consistency,
        show_vs_tell_ratio: show_vs_tell,
        entity_density,
        action_density,
        description_density,
    }
}
```

### 数据库 Schema

```sql
-- 更新 style_analysis_tasks 表，添加修辞和叙事结果存储
ALTER TABLE style_analysis_tasks 
ADD COLUMN rhetoric_json TEXT,
ADD COLUMN narrative_json TEXT;

-- 或者使用中间结果表（如果已创建）
-- analysis_intermediate_results 表已存在，直接插入即可
```

### API 端点扩展

```
# 风格分析进度查询
GET    /api/style-analysis/:id           # 获取任务详情（包含各层进度）
GET    /api/style-analysis/:id/rhetoric  # 获取修辞层分析结果
GET    /api/style-analysis/:id/narrative # 获取叙事层分析结果
```

### 响应格式

```json
{
  "success": true,
  "data": {
    "task_id": "uuid",
    "status": "rhetoric_narrative_completed",
    "progress": 0.50,
    "vocabulary": { ... },  // Story 1.3 的结果
    "sentence": { ... },    // Story 1.3 的结果
    "rhetoric": {
      "metaphor_frequency": 12.5,
      "simile_frequency": 8.3,
      "parallelism_frequency": 2.1,
      "sensory_preferences": {
        "visual": 45.2,
        "auditory": 18.5,
        "tactile": 12.3,
        "olfactory": 3.2,
        "gustatory": 1.8
      },
      "total_rhetoric_count": 156
    },
    "narrative": {
      "pov_type": "第三人称限知",
      "pov_consistency": 0.95,
      "show_vs_tell_ratio": 0.65,
      "entity_density": 8.5,
      "action_density": 25.3,
      "description_density": 18.7
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
│       ├── rhetoric_analysis.rs       # 修辞层分析（NEW）
│       ├── narrative_analysis.rs      # 叙事层分析（NEW）
│       └── style_analyzer.rs          # 统一分析入口
├── models/
│   └── style_features.rs              # 特征数据结构（扩展修辞/叙事结构）
├── handlers/
│   └── style_analysis.rs              # 分析结果查询接口（扩展）
└── db/
    └── migrations/
        └── 004_add_rhetoric_narrative_results.sql
```

### 前端扩展

```
src/
└── components/
    └── style/
        └── AnalysisProgress.vue     # 复用，增加修辞/叙事步骤显示
```

---

## Testing Requirements

### 后端测试（Rust）

```rust
// tests/rhetoric_analysis_test.rs

#[test]
fn test_detect_similes() {
    let text = "她的眼睛像星星一样闪亮。他如同一头猛虎。";
    let similes = detect_similes(text);
    assert_eq!(similes.len(), 2);
}

#[test]
fn test_metaphor_frequency() {
    let text = "他是狮子。他成为了王者。他化作一道光。";
    let result = extract_rhetoric_features(text);
    assert!(result.metaphor_frequency > 0.0);
}

#[test]
fn test_sensory_preferences() {
    let text = "红色的花朵在阳光下闪耀，散发出芳香。";
    let result = extract_rhetoric_features(text);
    assert!(result.sensory_preferences["visual"] > 0.0);
    assert!(result.sensory_preferences["olfactory"] > 0.0);
}

// tests/narrative_analysis_test.rs

#[test]
fn test_first_person_pov() {
    let text = "我走在路上，心里想着今天的事情。我觉得很奇怪。";
    let (pov_type, consistency) = identify_pov(text);
    assert_eq!(pov_type, "第一人称");
}

#[test]
fn test_third_person_pov() {
    let text = "他走向门口，打开门。她看着他，笑了。";
    let (pov_type, _) = identify_pov(text);
    assert!(pov_type.contains("第三人称"));
}

#[test]
fn test_show_vs_tell_ratio() {
    let text = "他喊道：'快跑！'然后冲了出去。";
    let ratio = analyze_show_vs_tell(text);
    assert!(ratio > 0.5); // 应该是 Show 为主
}

#[test]
fn test_large_text_narrative_analysis() {
    let text = std::fs::read_to_string("test_data/large_sample.txt").unwrap();
    let result = extract_narrative_features(&text);
    assert!(result.pov_consistency >= 0.0 && result.pov_consistency <= 1.0);
}
```

### 前端测试（Vitest）

```typescript
// tests/analysis-progress.test.ts

describe('AnalysisProgress - Rhetoric & Narrative', () => {
  it('should display rhetoric analysis step', () => {
    // 测试修辞层步骤显示
  });

  it('should display narrative analysis step', () => {
    // 测试叙事层步骤显示
  });

  it('should update progress to 50% when rhetoric and narrative complete', async () => {
    // 测试进度更新到 50%
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
  
  ✓ 修辞层分析完成
     - 隐喻频率：12.5 次/千字
     - 感官偏好：视觉 > 听觉 > 触觉
  
  ✓ 叙事层分析完成
     - 叙事视角：第三人称限知
     - Show vs Tell: 65% Show
  
  → 情感层分析中... (10%)
  
  ○ 节奏层分析
  ○ 对话层分析
  ○ 描写层分析
```

### 进度状态映射

| 状态 | 进度 | 显示 |
|------|------|------|
| vocabulary_sentence_completed | 25% | 词汇/句式完成 ✓ |
| rhetoric_completed | 37.5% | 修辞层完成 ✓ |
| narrative_completed | 50% | 叙事层完成 ✓ |
| rhetoric_narrative_completed | 50% | 四层都完成 |

---

## Implementation Notes

### 关键实现细节

1. **修辞识别挑战：**
   - 隐喻/明喻识别需要语义理解，规则方法准确率有限
   - 排比识别需要句法分析，Rust 中文 NLP 工具链相对简单
   - 建议：先实现规则版，后续可引入 LLM 辅助识别

2. **叙事视角识别：**
   - 第一/第三人称相对容易（代词统计）
   - 限知/全知区分较难，需要检测心理活动描写
   - 混合视角需要检测视角切换点

3. **Show vs Tell 分析：**
   - 这是文学理论概念，没有明确边界
   - 简化：动作/对话=Show，概述/评价=Tell
   - 准确分析需要深度语义理解

4. **性能优化：**
   - 修辞模式匹配使用正则预编译
   - 大文本分块处理（每 10 万字一块）
   - 与 Story 1.3 的分析可以并行执行

5. **边界情况：**
   - 无明显修辞手法的文本
   - 对话体小说（可能修辞少）
   - 视角频繁切换的实验性文本

### 依赖库推荐

```toml
[dependencies]
# 中文分词（词性标注辅助）
jieba-rs = "0.6"

# 正则表达式
regex = "1.10"

# Unicode 处理
unicode-segmentation = "1.10"

# 序列化
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### 与 Story 1.3 的集成

```rust
// services/style_analysis/style_analyzer.rs

pub struct StyleAnalysisProgress {
    pub vocabulary: Option<VocabularyAnalysisResult>,
    pub sentence: Option<SentenceAnalysisResult>,
    pub rhetoric: Option<RhetoricAnalysisResult>,
    pub narrative: Option<NarrativeAnalysisResult>,
    // ... 后续层
}

pub async fn analyze_style_layer_by_layer(
    text: &str,
    task_id: Uuid,
) -> Result<StyleAnalysisProgress> {
    let mut progress = StyleAnalysisProgress::default();
    
    // Story 1.3: 词汇层和句式层
    progress.vocabulary = Some(extract_vocabulary_features(text));
    update_task_progress(task_id, 0.125, "vocabulary_completed").await?;
    
    progress.sentence = Some(extract_sentence_features(text));
    update_task_progress(task_id, 0.25, "sentence_completed").await?;
    
    // Story 1.4: 修辞层和叙事层
    progress.rhetoric = Some(extract_rhetoric_features(text));
    update_task_progress(task_id, 0.375, "rhetoric_completed").await?;
    
    progress.narrative = Some(extract_narrative_features(text));
    update_task_progress(task_id, 0.50, "rhetoric_narrative_completed").await?;
    
    // ... 后续层（Story 1.5, 1.6）
    
    Ok(progress)
}
```

---

## Story Completion Status

- [x] 后端：修辞层分析函数实现
- [x] 后端：叙事层分析函数实现
- [x] 后端：分析结果数据模型扩展
- [x] 后端：数据库迁移（添加修辞/叙事结果存储）
- [x] 后端：分析进度更新逻辑（追加到 Story 1.3 结果）
- [x] 后端：查询接口扩展
- [x] 测试：修辞分析单元测试
- [x] 测试：叙事分析单元测试
- [x] 测试：集成测试（上传→分析→查询，验证 50% 进度）

## Change Log

- 2026-04-10：完成 Story 1-4 实现并标记为 done
  - 后端：修辞层分析服务（隐喻/明喻/排比/感官细节识别）
  - 后端：叙事层分析服务（视角识别/Show vs Tell/信息密度）
  - 后端：分析结果存储与查询接口
  - 后端：数据库迁移 004_add_rhetoric_narrative_results.sql
  - 测试：cargo test 通过（10 个测试全部通过）

---

**Story created:** 2026-04-09  
**Last updated:** 2026-04-10  
**Status:** done
