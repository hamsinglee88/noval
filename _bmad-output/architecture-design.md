# 📐 长篇小说 Agent 系统架构设计（完整版）

**项目名称：** Novel Agent - AI 长篇小说创作系统  
**版本：** v1.0  
**更新日期：** 2026-04-08  
**技术栈：** Rust 后端 + Vue 3 前端

---

## 目录

1. [系统架构总览](#一系统架构总览)
2. [核心模块设计](#二核心模块设计)
3. [风格模仿 Agent](#三风格模仿 Agent)
4. [数据库设计](#四数据库设计)
5. [API 设计](#五 api 设计)
6. [前端架构](#六前端架构)
7. [失败防御策略](#七失败防御策略)
8. [优先级与路线图](#八优先级与路线图)

---

## 一、系统架构总览

```
┌─────────────────────────────────────────────────────────────┐
│                      前端层 (Vue 3 + TypeScript)             │
│  ┌─────────────┬─────────────┬─────────────┬─────────────┐ │
│  │  创作编辑器  │  大纲规划器  │  角色管理   │  世界观构建  │ │
│  │  (Tiptap)   │  (卡片墙)    │  (关系图)   │  (设定库)   │ │
│  └─────────────┴─────────────┴─────────────┴─────────────┘ │
│  ┌─────────────┬─────────────┬─────────────┬─────────────┐ │
│  │  AI 辅助面板  │  伏笔追踪器  │  风格模仿   │  项目管理   │ │
│  └─────────────┴─────────────┴─────────────┴─────────────┘ │
└─────────────────────────────────────────────────────────────┘
                            ↓ REST API / WebSocket
┌─────────────────────────────────────────────────────────────┐
│                    后端层 (Rust + Axum)                      │
│  ┌─────────────────────────────────────────────────────┐   │
│  │                 API Gateway / 中间件                 │   │
│  │  认证 | 限流 | 日志 | CORS | 错误处理                │   │
│  └─────────────────────────────────────────────────────┘   │
│  ┌─────────────┬─────────────┬─────────────┬─────────────┐ │
│  │  创作引擎   │  角色引擎   │  情节引擎   │  伏笔引擎   │ │
│  └─────────────┴─────────────┴─────────────┴─────────────┘ │
│  ┌─────────────┬─────────────┬─────────────┬─────────────┐ │
│  │  风格引擎   │  LLM 路由器  │  上下文管理  │  一致性检查  │ │
│  └─────────────┴─────────────┴─────────────┴─────────────┘ │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│                      数据层                                  │
│  ┌─────────────┬─────────────┬─────────────┬─────────────┐ │
│  │   SQLite    │  文件系统    │  向量索引    │   缓存层    │ │
│  │  (结构化)   │  (原始文本)  │  (HNSW)     │  (Redis)   │ │
│  └─────────────┴─────────────┴─────────────┴─────────────┘ │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│                    外部服务层                                │
│  ┌─────────────┬─────────────┬─────────────┐               │
│  │  本地 LLM    │  云端 LLM    │  文件存储    │               │
│  │  (Ollama)   │  (Claude)   │  (本地)      │               │
│  └─────────────┴─────────────┴─────────────┘               │
└─────────────────────────────────────────────────────────────┘
```

---

## 二、核心模块设计

### 2.1 创作引擎 (Writing Engine)

**职责：** 管理文本生成、编辑、保存

```rust
pub struct WritingEngine {
    db: SqlitePool,
    llm_router: Arc<LlmRouter>,
    style_profile: Arc<StyleProfile>,
    consistency_checker: Arc<ConsistencyChecker>,
}

pub struct Chapter {
    id: ChapterId,
    novel_id: NovelId,
    chapter_number: u32,
    title: String,
    file_path: PathBuf,
    word_count: u32,
    status: ChapterStatus,
    created_at: DateTime,
    updated_at: DateTime,
}
```

**核心功能：**

| 功能 | 描述 | 优先级 |
|------|------|--------|
| 实时保存 | 每 30 秒自动保存 | P0 |
| 版本历史 | 保留所有历史版本 | P0 |
| 风格约束 | 生成时遵循用户风格 | P0 |
| 差异对比 | 比较不同版本 | P1 |
| 分支剧情 | 实验不同走向 | P2 |

---

### 2.2 角色引擎 (Character Engine)

**职责：** 角色档案管理、一致性检查、弧光追踪

```rust
pub struct CharacterEngine {
    db: SqlitePool,
    behavior_index: Arc<BehaviorIndex>,
}

pub struct Character {
    id: CharacterId,
    novel_id: NovelId,
    name: String,
    level: CharacterLevel,  // Main/Supporting/Minor
    core_narrative: String, // 叙事式存储
    key_traits: Vec<Trait>,
    behavior_log: Vec<BehaviorRecord>,
    arc_state: ArcState,
}
```

**核心功能：**

| 功能 | 描述 | 优先级 |
|------|------|--------|
| 叙事式存储 | 自然语言角色档案 | P0 |
| 规则检查 | Never/Always 规则验证 | P0 |
| 行为追溯 | 与历史行为相似度对比 | P0 |
| 弧光追踪 | 里程碑 + 快照 | P1 |
| 分级管理 | 主/次/配角不同追踪级别 | P1 |
| 动态关系 | 需要时计算角色关系 | P2 |

---

### 2.3 情节引擎 (Plot Engine)

**职责：** 大纲管理、情节线追踪、任务式伏笔管理

```rust
pub struct PlotEngine {
    db: SqlitePool,
}

pub struct QuestLine {
    id: QuestId,
    name: String,
    status: QuestStatus,
    prerequisites: Vec<QuestId>,
    triggers: Vec<Trigger>,
    resolution: Option<Resolution>,
}
```

**核心功能：**

| 功能 | 描述 | 优先级 |
|------|------|--------|
| 情节卡片墙 | 可拖拽的场景卡片 | P0 |
| 三幕结构模板 | 经典剧作结构 | P1 |
| 时间轴视图 | 故事时间线性展示 | P1 |
| 多线叙事管理 | A/B/C 故事线并行 | P1 |
| 节奏曲线 | 情感/冲突强度可视化 | P2 |

---

### 2.4 伏笔引擎 (Foreshadow Engine)

**职责：** 伏笔识别、追踪、回收提醒

```rust
pub struct ForeshadowTracker {
    db: SqlitePool,
    ai_classifier: Arc<ForeshadowClassifier>,
}

pub struct Foreshadow {
    id: ForeshadowId,
    content: String,
    chapter_id: ChapterId,
    foreshadow_type: ForeshadowType,
    status: ForeshadowStatus,
    expected_resolution_chapter: Option<u32>,
    resolution_chapter: Option<ChapterId>,
    confidence_score: f32,
    urgency: Urgency,
}
```

**核心功能：**

| 功能 | 描述 | 优先级 |
|------|------|--------|
| AI 自动识别 | NLP 检测伏笔语言模式 | P0 |
| 状态追踪 | Active/Resolved/Abandoned | P0 |
| 回收倒计时 | 预期回收章节设置 | P0 |
| 逾期警告 | 超过预期未回收则警告 | P0 |
| 伏笔地图 | 可视化展示所有伏笔 | P1 |
| 结局检查 | 小说结束时检查未回收伏笔 | P1 |

---

### 2.5 LLM 路由器 (LLM Router)

**职责：** 混合模型管理、任务路由、成本优化

```rust
pub struct LlmRouter {
    config: Arc<LlmConfig>,
    local_client: Arc<LocalLlmClient>,
    cloud_client: Arc<CloudLlmClient>,
}

pub enum TaskType {
    Drafting,           // 章节草稿 → 云端
    Outlining,          // 大纲生成 → 云端
    ForeshadowDetection,// 伏笔识别 → 本地
    ConsistencyCheck,   // 连贯性检查 → 本地
    StyleTransfer,      // 风格润色 → 云端
    Summarization,      // 摘要压缩 → 本地
}
```

**核心功能：**

| 功能 | 描述 | 优先级 |
|------|------|--------|
| 混合模式 | 本地 + 云端可配置 | P0 |
| 任务路由 | 根据任务类型选择模型 | P0 |
| 成本优化 | 简单任务用便宜模型 | P1 |
| 模型回退 | 主模型失败切换备用 | P1 |
| A/B 测试 | 多模型输出对比 | P2 |

---

### 2.6 上下文管理引擎 (Context Engine)

**职责：** 无损压缩、分层摘要、向量检索

```rust
pub struct ContextEngine {
    db: SqlitePool,
    vector_index: Arc<HnswIndex>,
}

pub struct ContextSnapshot {
    id: SnapshotId,
    novel_id: NovelId,
    checkpoint_chapter: ChapterId,
    foreshadow_layer: ForeshadowSummary,
    character_layer: CharacterSummary,
    plot_layer: PlotSummary,
    world_layer: WorldSummary,
    emotional_layer: EmotionalSummary,
    vector_index_path: PathBuf,
}
```

**核心功能：**

| 功能 | 描述 | 优先级 |
|------|------|--------|
| 分层压缩 | 伏笔/角色/情节/世界观分层 | P0 |
| 无损存储 | 原文存储在文件系统 | P0 |
| 向量检索 | 语义搜索精确检索 | P0 |
| 主动注入 | 生成前检索相关上下文 | P0 |
| 冲突检测 | 新内容与历史对比 | P0 |

---

## 三、风格模仿 Agent

### 3.1 概述

**目标：** 分析参考小说的写作风格，提取风格特征，然后在生成时模仿这种风格

**核心价值：** 让 AI 生成内容"像人写的"而非"像 AI 写的"

**关键挑战：**
- 风格是抽象的，如何量化？
- 如何区分"模仿"和"抄袭"？
- 如何处理多本书的混合风格？

---

### 3.2 七层风格特征框架

```rust
pub struct StyleProfile {
    // 词汇层
    pub vocabulary: VocabularyStyle,
    // 句式层
    pub sentence: SentenceStyle,
    // 修辞层
    pub rhetoric: RhetoricStyle,
    // 叙事层
    pub narrative: NarrativeStyle,
    // 情感层
    pub emotional: EmotionalStyle,
    // 节奏层
    pub pacing: PacingStyle,
    // 对话层
    pub dialogue: DialogueStyle,
    // 描写层
    pub description: DescriptionStyle,
}
```

**词汇层 (VocabularyStyle):**
```rust
pub struct VocabularyStyle {
    pub avg_word_length: f32,              // 平均词长
    pub vocabulary_richness: f32,          // 词汇丰富度 (unique/total)
    pub common_adjectives: Vec<String>,    // 常用形容词
    pub common_verbs: Vec<String>,         // 常用动词
    pub common_adverbs: Vec<String>,       // 常用副词
    pub rare_words: Vec<String>,           // 罕见词使用习惯
    pub forbidden_words: Vec<String>,      // 几乎不用的词
    pub transition_words: Vec<String>,     // 连接词偏好
}
```

**句式层 (SentenceStyle):**
```rust
pub struct SentenceStyle {
    pub avg_sentence_length: f32,          // 平均句长
    pub sentence_length_variance: f32,     // 句长变化
    pub short_sentence_ratio: f32,         // 短句比例 (<10 字)
    pub medium_sentence_ratio: f32,        // 中句比例 (10-30 字)
    pub long_sentence_ratio: f32,          // 长句比例 (>30 字)
    pub complex_sentence_ratio: f32,       // 复合句比例
    pub question_ratio: f32,               // 问句比例
    pub exclamation_ratio: f32,            // 感叹句比例
    pub paragraph_avg_sentences: f32,      // 段落平均句数
}
```

**各层特征详情：**

| 层次 | 关键特征 | 示例字段 |
|------|---------|---------|
| 词汇层 | 用词偏好、丰富度 | `common_adjectives`, `vocabulary_richness` |
| 句式层 | 句长分布、句式类型 | `avg_sentence_length`, `complex_sentence_ratio` |
| 修辞层 | 修辞手法、感官细节 | `metaphor_frequency`, `sensory_details` |
| 叙事层 | 视角、时间结构 | `pov_type`, `show_vs_tell_ratio` |
| 情感层 | 情感基调、波动 | `overall_tone`, `emotional_range` |
| 节奏层 | 场景节奏、章节结构 | `action_scene_pacing`, `cliffhanger_frequency` |
| 对话层 | 对话比例、角色声音 | `dialogue_ratio`, `character_voice_distinction` |
| 描写层 | 描写偏好、详细程度 | `description_ratio`, `detail_granularity` |

---

### 3.3 风格分析工作流

```
┌─────────────────────────────────────────────────────────────┐
│                   风格分析 Agent                             │
│                                                              │
│  输入：参考小说 (TXT/EPUB 格式)                              │
│  输出：StyleProfile (JSON)                                   │
│                                                              │
│  Step 1: 文本预处理 → 分章/分段/分句、词性标注               │
│  Step 2: 统计特征提取 → 词汇/句式/修辞统计                   │
│  Step 3: LLM 深度分析 → 叙事/情感/节奏识别                    │
│  Step 4: 风格向量化 → 128 维嵌入向量                          │
│  Step 5: 生成风格报告 → 可视化 + 示例段落                     │
└─────────────────────────────────────────────────────────────┘
```

---

### 3.4 核心组件

```rust
// 风格特征提取器
pub struct StyleFeatureExtractor {
    jieba: Jieba,
    stemmer: Stemmer,
}

impl StyleFeatureExtractor {
    /// 提取词汇层特征
    pub fn extract_vocabulary_features(&self, text: &str) -> VocabularyStyle {
        // 分词
        let words: Vec<&str> = self.jieba.cut(text, false).collect();
        // 词性标注
        let pos_tagged = self.pos_tag(&words);
        // 统计词频
        let word_freq = self.count_frequency(&words);
        // 词汇丰富度 (Type-Token Ratio)
        let unique_words: HashSet<_> = words.iter().collect();
        let ttr = unique_words.len() as f32 / words.len() as f32;
        // ... 更多特征提取
    }
    
    /// 提取句式层特征
    pub fn extract_sentence_features(&self, text: &str) -> SentenceStyle {
        // 分句（基于中文标点）
        let sentences: Vec<&str> = text.split(&['.', '!', '?', ';', '…'][..]).collect();
        // 句长统计
        let sentence_lengths: Vec<usize> = sentences.iter()
            .filter(|s| !s.is_empty())
            .map(|s| s.chars().count())
            .collect();
        // ... 更多特征提取
    }
}

// 风格向量化器
pub struct StyleVectorizer {
    weights: StyleWeights,  // 各维度权重
}

pub struct StyleWeights {
    vocabulary_weight: f32,      // 0.15
    sentence_weight: f32,        // 0.15
    rhetoric_weight: f32,        // 0.10
    narrative_weight: f32,       // 0.20
    emotional_weight: f32,       // 0.15
    pacing_weight: f32,          // 0.10
    dialogue_weight: f32,        // 0.10
    description_weight: f32,     // 0.05
}

impl StyleVectorizer {
    /// 将 StyleProfile 转换为 128 维向量
    pub fn vectorize(&self, profile: &StyleProfile) -> Array1<f32> {
        let mut vector = Array1::zeros(128);
        // 词汇层 (16 维)
        // 句式层 (16 维)
        // 修辞层 (16 维)
        // 叙事层 (24 维)
        // 情感层 (16 维)
        // 节奏层 (16 维)
        // 对话层 (12 维)
        // 描写层 (12 维)
        vector
    }
    
    /// 计算两个风格的相似度（余弦相似度）
    pub fn style_similarity(&self, v1: &Array1<f32>, v2: &Array1<f32>) -> f32 {
        let dot = v1.dot(v2);
        let norm1 = v1.norm();
        let norm2 = v2.norm();
        if norm1 == 0.0 || norm2 == 0.0 { return 0.0; }
        dot / (norm1 * norm2)
    }
}

// 风格迁移生成器
pub struct StyleTransferGenerator {
    style_analyzer: Arc<StyleAnalyzer>,
    llm_client: Arc<dyn LlmClient>,
}

impl StyleTransferGenerator {
    /// 基于目标风格生成内容
    pub async fn generate_with_style(
        &self,
        prompt: String,
        target_style: &StyleProfile,
        context: String,
        autonomy_level: AutonomyLevel,
    ) -> Result<GenerationResult> {
        // Step 1: 构建风格化 Prompt
        let styled_prompt = self.build_styled_prompt(&prompt, target_style, &context).await?;
        
        // Step 2: LLM 生成
        let raw_generation = self.llm_client.generate(&styled_prompt).await?;
        
        // Step 3: 风格一致性检查
        let style_match = self.check_style_consistency(&raw_generation, target_style).await?;
        
        // Step 4: 如果风格不匹配，进行润色
        let final_output = if style_match.overall_score < 0.7 {
            self.refine_to_match_style(&raw_generation, target_style).await?
        } else {
            raw_generation
        };
        
        // Step 5: 根据自主性级别决定输出
        Ok(GenerationResult {
            content: final_output,
            style_match_score: style_match.overall_score,
            requires_approval: autonomy_level == AutonomyLevel::ConfirmBefore,
        })
    }
}

// 风格混合引擎
pub struct StyleMixer;

impl StyleMixer {
    /// 混合多个风格档案
    pub fn mix_styles(
        &self,
        profiles: Vec<StyleProfile>,
        weights: Vec<f32>,  // 如 [0.5, 0.5] 表示 50% + 50%
    ) -> Result<StyleProfile> {
        // 1. 向量化所有风格
        // 2. 加权平均向量
        // 3. 从混合向量重建 StyleProfile
    }
}
```

---

### 3.5 风格模仿流程

```
1. 用户上传参考小说 (TXT/EPUB)
         ↓
2. StyleAnalyzer 分析风格（统计 + LLM）
         ↓
3. 生成 StyleProfile 并保存（128 维向量）
         ↓
4. 用户在创作时选择风格
         ↓
5. AI 生成时应用风格约束（风格化 Prompt）
         ↓
6. 风格一致性检查（相似度 < 0.7 则润色）
         ↓
7. 输出风格匹配的内容
```

---

### 3.6 风格化 Prompt 示例

```
你是一位专业的小说作家，需要模仿以下写作风格进行创作。

## 目标风格特征

### 词汇特征
- 常用形容词：苍凉、雄浑、飘逸、...
- 常用动词：纵身、提气、凝目、...
- 避免使用的词：搞定、靠谱、...
- 词汇丰富度：0.65

### 句式特征
- 平均句长：23.5 字
- 短句比例：35%  中句比例：45%  长句比例：20%
- 复合句比例：25%

### 修辞特征
- 隐喻频率：0.15  明喻频率：0.08  排比频率：0.05
- 感官偏好：[视觉，听觉]

### 叙事特征
- 视角：第三人称限知
- 展示/讲述比例：60/40
- 信息密度：0.72

### 情感基调
- 整体基调：史诗感/压抑/轻松
- 情感波动：0.45
- 幽默频率：0.08

### 节奏特征
- 章节平均长度：4500 字
- 悬念结尾频率：0.65

## 示例段落（体现目标风格）

"萧峰纵身一跃，已落在擂台中央，提气喝道：'今日便要领教各位的高招！'
风卷起他的衣袂，猎猎作响..."

## 创作要求

请基于以下上下文，按照上述风格创作内容：

**上下文：** [当前章节的前文]
**创作提示：** [用户的写作要求]

## 输出要求
- 严格遵循目标风格的词汇、句式、修辞特征
- 保持叙事视角一致
- 情感基调与目标风格匹配
- 输出纯文本，不要解释
```

---

### 3.7 创新功能

| 功能 | 描述 | 优先级 |
|------|------|--------|
| 风格混合 | 50% 金庸 + 50% 古龙 = 新风格 | P1 |
| 风格迁移 | 将用户草稿转换为目标风格 | P1 |
| 风格对比 | 对比两本书的风格差异 | P2 |
| 风格进化 | 分析作者不同时期风格变化 | P2 |
| 风格推荐 | "喜欢这本书？试试这些风格" | P2 |
| 风格市场 | 用户分享/下载风格档案 | P3 |
| 风格检测 | "这段文字像谁写的？" | P2 |

---

### 3.8 风格管理 UI 设计

```
┌─────────────────────────────────────────────────────────────┐
│  风格模仿 Agent                                             │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌────────────────────────────────────────────────────┐     │
│  │  📚 上传参考小说                                    │     │
│  │  [拖拽文件到此处] 或 [点击上传]                      │     │
│  │  支持格式：TXT, EPUB, PDF                          │     │
│  └────────────────────────────────────────────────────┘     │
│                                                              │
│  已分析的风格档案：                                          │
│  ┌──────────────────┐  ┌──────────────────┐                │
│  │ 📖 金庸武侠风格   │  │ 📖 刘慈欣科幻风格 │                │
│  │ 来源：天龙八部    │  │ 来源：三体        │                │
│  │ 分析完成 ✓        │  │ 分析完成 ✓        │                │
│  │ [查看报告] [使用] │  │ [查看报告] [使用] │                │
│  └──────────────────┘  └──────────────────┘                │
│                                                              │
│  风格雷达图：                                                │
│  [词汇丰富度、句式变化、修辞频率、情感表达、节奏控制]        │
│                                                              │
│  风格特征详情：                                              │
│  - 词汇特征：常用形容词、动词列表                           │
│  - 句式特征：平均句长、句长分布                             │
│  - 修辞特征：隐喻/排比频率、感官偏好                        │
│  - 叙事特征：视角、展示/讲述比例                            │
│                                                              │
│  示例段落：                                                  │
│  "萧峰纵身一跃，已落在擂台中央..."                          │
│  [标注] 动词选择、短句节奏、动作描写风格                    │
│                                                              │
│  [保存风格] [导出] [删除]                                    │
└─────────────────────────────────────────────────────────────┘
```

---

## 四、数据库设计

```sql
-- 小说项目
CREATE TABLE novels (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT NOT NULL,
    description TEXT,
    style_profile_json TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 章节
CREATE TABLE chapters (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    novel_id UUID REFERENCES novels(id) ON DELETE CASCADE,
    chapter_number INTEGER NOT NULL,
    title TEXT,
    file_path TEXT NOT NULL,
    word_count INTEGER DEFAULT 0,
    status TEXT DEFAULT 'draft',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(novel_id, chapter_number)
);

-- 角色
CREATE TABLE characters (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    novel_id UUID REFERENCES novels(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    level TEXT NOT NULL,  -- main/supporting/minor
    core_narrative TEXT NOT NULL,
    profile_json TEXT,
    arc_state_json TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 角色规则
CREATE TABLE character_rules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    character_id UUID REFERENCES characters(id) ON DELETE CASCADE,
    rule_type TEXT NOT NULL,  -- never/always/sometimes/contextual
    description TEXT NOT NULL,
    priority TEXT DEFAULT 'medium',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 角色行为日志
CREATE TABLE behavior_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    character_id UUID REFERENCES characters(id) ON DELETE CASCADE,
    chapter_id UUID REFERENCES chapters(id),
    action TEXT NOT NULL,
    context TEXT,
    reasoning TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 伏笔
CREATE TABLE foreshadows (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    novel_id UUID REFERENCES novels(id) ON DELETE CASCADE,
    chapter_id UUID REFERENCES chapters(id),
    content TEXT NOT NULL,
    foreshadow_type TEXT NOT NULL,
    status TEXT DEFAULT 'active',
    expected_resolution_chapter INTEGER,
    resolution_chapter_id UUID REFERENCES chapters(id),
    confidence_score REAL DEFAULT 1.0,
    urgency TEXT DEFAULT 'low',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 情节线
CREATE TABLE plot_lines (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    novel_id UUID REFERENCES novels(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    plot_type TEXT NOT NULL,
    status TEXT DEFAULT 'active',
    chapters_involved UUID[],
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 上下文快照
CREATE TABLE context_snapshots (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    novel_id UUID REFERENCES novels(id) ON DELETE CASCADE,
    checkpoint_chapter UUID REFERENCES chapters(id),
    summary_json TEXT NOT NULL,
    vector_index_path TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 风格档案（新增）
CREATE TABLE style_profiles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    source_novels TEXT[],
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    -- 各层特征 JSON
    vocabulary_json TEXT,
    sentence_json TEXT,
    rhetoric_json TEXT,
    narrative_json TEXT,
    emotional_json TEXT,
    pacing_json TEXT,
    dialogue_json TEXT,
    description_json TEXT,
    
    -- 风格向量（用于相似度检索）
    style_vector TEXT,
    
    -- 示例段落
    example_passages TEXT
);

-- 小说 - 风格关联（新增）
CREATE TABLE novel_style_bindings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    novel_id UUID REFERENCES novels(id) ON DELETE CASCADE,
    style_profile_id UUID REFERENCES style_profiles(id),
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 风格分析任务（新增）
CREATE TABLE style_analysis_tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    novel_id UUID REFERENCES novels(id),
    source_file_path TEXT NOT NULL,
    status TEXT DEFAULT 'pending',
    progress REAL DEFAULT 0.0,
    result_profile_id UUID REFERENCES style_profiles(id),
    error_message TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 写作统计
CREATE TABLE writing_stats (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    novel_id UUID REFERENCES novels(id) ON DELETE CASCADE,
    date DATE NOT NULL,
    words_written INTEGER DEFAULT 0,
    time_spent_minutes INTEGER DEFAULT 0,
    UNIQUE(novel_id, date)
);

-- 索引
CREATE INDEX idx_chapters_novel ON chapters(novel_id);
CREATE INDEX idx_characters_novel ON characters(novel_id);
CREATE INDEX idx_foreshadows_novel ON foreshadows(novel_id);
CREATE INDEX idx_foreshadows_status ON foreshadows(status);
CREATE INDEX idx_behavior_logs_character ON behavior_logs(character_id);
CREATE INDEX idx_style_profiles_name ON style_profiles(name);
CREATE INDEX idx_novel_style_bindings_novel ON novel_style_bindings(novel_id);
```

---

## 五、API 设计

### 5.1 RESTful Endpoints

```
# 小说管理
GET    /api/novels                 # 获取小说列表
POST   /api/novels                 # 创建小说
GET    /api/novels/:id             # 获取小说详情
PUT    /api/novels/:id             # 更新小说
DELETE /api/novels/:id             # 删除小说

# 章节管理
GET    /api/novels/:id/chapters    # 获取章节列表
POST   /api/novels/:id/chapters    # 创建章节
GET    /api/chapters/:id           # 获取章节内容
PUT    /api/chapters/:id           # 更新章节
DELETE /api/chapters/:id           # 删除章节

# 角色管理
GET    /api/novels/:id/characters  # 获取角色列表
POST   /api/novels/:id/characters  # 创建角色
PUT    /api/characters/:id         # 更新角色
DELETE /api/characters/:id         # 删除角色

# 伏笔管理
GET    /api/novels/:id/foreshadows # 获取伏笔列表
POST   /api/novels/:id/foreshadows # 创建伏笔
PUT    /api/foreshadows/:id        # 更新伏笔
DELETE /api/foreshadows/:id        # 删除伏笔

# 风格管理（新增）
GET    /api/styles                 # 获取所有风格
POST   /api/styles/analyze         # 分析新风格
POST   /api/styles/mix             # 混合风格
GET    /api/styles/:id             # 获取风格详情
DELETE /api/styles/:id             # 删除风格

# AI 功能
POST   /api/novels/:id/generate    # AI 生成内容
POST   /api/novels/:id/analyze     # AI 分析
POST   /api/novels/:id/check       # 一致性检查
```

### 5.2 WebSocket 事件

```json
// 服务端 → 客户端
{ "type": "generation_progress", "data": { "progress": 0.6, "current_text": "..." } }
{ "type": "consistency_alert", "data": { "violation": "...", "suggestion": "..." } }
{ "type": "foreshadow_reminder", "data": { "foreshadow_id": "...", "urgency": "high" } }
{ "type": "style_analysis_progress", "data": { "step": 2, "total": 5 } }

// 客户端 → 服务端
{ "type": "start_generation", "data": { "chapter_id": "...", "prompt": "..." } }
{ "type": "analyze_style", "data": { "file_path": "..." } }
{ "type": "accept_suggestion", "data": { "suggestion_id": "..." } }
```

---

## 六、前端架构

### 6.1 技术栈

```json
{
  "framework": "Vue 3.4+ (Composition API)",
  "language": "TypeScript 5.x",
  "build": "Vite 5.x",
  "state": {
    "client": "Pinia",
    "server": "TanStack Query (Vue Query)"
  },
  "ui": {
    "components": "Naive UI",
    "editor": "Tiptap (基于 ProseMirror)"
  },
  "charts": {
    "relationship": "Cytoscape.js",
    "timeline": "ECharts",
    "style_radar": "Chart.js"
  },
  "http": "Axios",
  "realtime": "WebSocket"
}
```

---

### 6.2 核心组件

```
src/
├── components/
│   ├── editor/
│   │   ├── NovelEditor.vue      # 主编辑器
│   │   ├── EditorToolbar.vue    # 工具栏
│   │   └── StyleIndicator.vue   # 风格指示器
│   ├── outline/
│   │   ├── PlotCardWall.vue     # 情节卡片墙
│   │   ├── TimelineView.vue     # 时间轴视图
│   │   └── StructureTemplate.vue # 结构模板
│   ├── character/
│   │   ├── CharacterCard.vue    # 角色卡片
│   │   ├── RelationshipGraph.vue # 关系图
│   │   └── ArcTimeline.vue      # 弧光时间线
│   ├── foreshadow/
│   │   ├── ForeshadowList.vue   # 伏笔列表
│   │   ├── ForeshadowMap.vue    # 伏笔地图
│   │   └── UrgencyIndicator.vue # 紧急度指示器
│   ├── style/                   # 风格模仿模块（新增）
│   │   ├── StyleUploader.vue    # 风格文件上传
│   │   ├── StyleAnalyzer.vue    # 风格分析界面
│   │   ├── StyleProfileCard.vue # 风格档案卡片
│   │   ├── StyleRadarChart.vue  # 风格雷达图
│   │   ├── StyleMixer.vue       # 风格混合器
│   │   └── StyleLibrary.vue     # 风格库
│   └── ai/
│       ├── AiAssistantPanel.vue # AI 助手面板
│       ├── GenerationPreview.vue # 生成预览
│       └── AutonomySlider.vue   # 自主性滑块
├── stores/
│   ├── novel.ts                 # 小说状态
│   ├── character.ts             # 角色状态
│   ├── plot.ts                  # 情节状态
│   ├── style.ts                 # 风格状态（新增）
│   └── ai.ts                    # AI 配置状态
└── services/
    ├── api.ts                   # API 客户端
    ├── websocket.ts             # WebSocket 连接
    └── style.ts                 # 风格分析 API（新增）
```

---

## 七、失败防御策略

| 失败场景 | 失败表现 | 防御策略 | 实现模块 |
|---------|---------|---------|---------|
| 内容机械 | 对话像 AI、描写千篇一律 | 风格学习 + 禁忌词表 + 人工审核 | WritingEngine + StyleEngine |
| 角色不一致 | 前后判若两人、行为矛盾 | 实时检查 + 规则验证 + 行为追溯 | CharacterEngine |
| 伏笔未回收 | 埋坑不填、线索消失 | 状态追踪 + 逾期警告 + 结局检查 | ForeshadowTracker |
| 情节老套 | 全是套路、没有新意 | 套路检测 + 反套路建议 + 多分支 | PlotEngine |
| 失去控制感 | AI 自作主张、不知道要写什么 | 自主性配置 + 变更日志 + 撤销 | AutonomyConfig |
| 上下文丢失 | 忘记前文设定、前后矛盾 | 分层压缩 + 向量检索 + 主动注入 | ContextEngine |
| 性能问题 | 加载慢、搜索慢 | 懒加载 + 查询优化 + 缓存 | 全系统 |
| 学习成本高 | 不知道从哪里开始 | 新手引导 + 模板系统 + AI 解释 | Frontend |

---

## 八、优先级与路线图

### Phase 1: MVP (4-6 周)

**目标：** 最基础的小说创作功能

- [ ] 基础编辑器 + 实时保存
- [ ] 章节管理 + 文件系统存储
- [ ] SQLite + SQLx 集成
- [ ] 基础 LLM 集成（云端）
- [ ] 简单角色档案

**交付物：** 可用的基础写作工具

---

### Phase 2: 核心功能 (6-8 周)

**目标：** AI 辅助创作核心能力

- [ ] 角色一致性检查
- [ ] 伏笔追踪系统
- [ ] 混合 LLM 路由
- [ ] 上下文压缩
- [ ] 风格分析基础功能

**交付物：** 具有 AI 辅助能力的创作平台

---

### Phase 3: 高级功能 (8-12 周)

**目标：** 差异化竞争优势

- [ ] 风格迁移生成
- [ ] 风格混合引擎
- [ ] 情节卡片墙
- [ ] 角色关系图
- [ ] 自主性配置系统
- [ ] 伏笔地图可视化

**交付物：** 完整的长篇小说创作系统

---

### Phase 4: 扩展功能 (12+ 周)

**目标：** 生态与扩展

- [ ] 风格市场
- [ ] 连载发布集成
- [ ] 协作创作模式
- [ ] IP 衍生工具
- [ ] 学习成长系统

**交付物：** 创作者生态系统

---

## 九、技术风险与缓解

| 风险 | 影响 | 可能性 | 缓解策略 |
|------|------|--------|---------|
| LLM API 成本高 | 中 | 中 | 混合模式 + 成本优化路由 |
| 上下文窗口限制 | 高 | 高 | 分层压缩 + 向量检索 |
| 角色一致性难保证 | 高 | 中 | 多层检查 + 用户确认 |
| 风格分析准确性 | 中 | 中 | LLM + 统计混合方法 |
| 性能瓶颈 | 中 | 中 | 懒加载 + 索引优化 |
| 用户接受度 | 中 | 低 | 新手引导 + 示例项目 |

---

## 附录：头脑风暴成果统计

本次头脑风暴共产生 **265+ 个想法**，涵盖：

| 类别 | 想法数量 | 关键产出 |
|------|---------|---------|
| 系统架构 | 25 | Rust+Vue 技术栈、SQLite+ 文件混合存储 |
| 前端功能 | 60 | 编辑器/大纲/角色/伏笔/UI 组件 |
| 角色管理 | 30 | 叙事式存储、分级管理、弧光追踪 |
| 逆向思考（失败防御）| 30 | 8 大失败场景 + 防御策略 |
| AI 模型协作 | 15 | 混合 LLM 路由、任务分配 |
| 连载发布 | 15 | 一键发布、读者反馈分析 |
| 协作创作 | 15 | 多人协作、权限管理 |
| IP 衍生 | 15 | 剧本改编、角色立绘 |
| 学习成长 | 15 | 写作课程、弱点诊断 |
| 情感健康 | 15 | 倦怠预警、心流辅助 |
| 数据洞察 | 15 | 写作统计、习惯分析 |
| 高级 AI 功能 | 20 | 自动校对、事实核查 |
| **风格模仿 Agent** | **15** | **七层特征框架、风格迁移** |

---

## 总结

这是一份完整的**长篇小说 AI 创作系统**架构设计，核心创新点包括：

1. **游戏任务系统用于情节管理** - 伏笔 = 待触发任务
2. **角色自主决策引擎** - 基于性格而非情节需要
3. **无损上下文压缩** - 分层摘要 + 向量索引
4. **AI 自动伏笔识别** - NLP 检测 + 置信度评分
5. **混合 LLM 路由** - 任务类型自动选择模型
6. **七层风格模仿** - 词汇/句式/修辞/叙事/情感/节奏/对话

---

*文档版本：v1.0*  
*创建日期：2026-04-08*  
*头脑风暴会话完成*
