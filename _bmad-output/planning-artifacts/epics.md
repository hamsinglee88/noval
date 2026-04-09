---
stepsCompleted: ["step-01-validate-prerequisites", "step-02-design-epics", "step-03-create-stories", "step-04-final-validation"]
inputDocuments: ["_bmad-output/planning-artifacts/prd.md", "_bmad-output/architecture-design.md", "_bmad-output/planning-artifacts/ux-design-specification.md"]
---

# Novel Agent - Epic Breakdown

## Overview

本文档提供 Novel Agent 的完整 Epic 和 Story 分解，将 PRD、UX Design 和 Architecture 需求分解为可实现的 Story。

## Requirements Inventory

### Functional Requirements (FRs)

**风格管理 (FR1-FR14):**
FR1: 作家可以上传小说文本进行风格分析
FR2: 系统可以从上传文本中提取词汇层特征
FR3: 系统可以从上传文本中提取句式层特征
FR4: 系统可以从上传文本中提取修辞层特征
FR5: 系统可以从上传文本中提取叙事层特征
FR6: 系统可以从上传文本中提取情感层特征
FR7: 系统可以从上传文本中提取节奏层特征
FR8: 系统可以从上传文本中提取对话层特征
FR9: 系统可以从上传文本中提取描写层特征
FR10: 系统可以生成风格向量化表示（128 维）
FR11: 系统可以保存风格档案到本地存储
FR12: 系统可以加载已保存的风格档案
FR13: 系统可以混合多个风格档案生成新风格
FR14: 系统可以计算两段文本的风格相似度

**创作编辑 (FR15-FR23):**
FR15: 作家可以创建新的小说项目
FR16: 作家可以为项目选择风格档案
FR17: 作家可以创建新章节
FR18: 作家可以编辑章节内容
FR19: 作家可以删除章节
FR20: 系统可以自动保存章节草稿
FR21: 作家可以查看章节历史版本
FR22: 作家可以回滚到历史版本
FR23: 作家可以导出章节为 TXT/EPUB 格式

**AI 生成 (FR24-FR30):**
FR24: 作家可以使用 AI 续写功能生成草稿
FR25: 作家可以使用 AI 润色功能改进文本
FR26: 作家可以使用 AI 扩写功能将大纲扩展为场景
FR27: 系统可以在生成时应用风格约束
FR28: 系统可以检测生成内容的风格匹配度
FR29: 作家可以配置 AI 自主性级别
FR30: 系统可以在需要确认时提示作家

**连贯性管理 (FR31-FR40):**
FR31: 系统可以自动识别文本中的伏笔
FR32: 系统可以标记伏笔的状态（活跃/已回收/废弃）
FR33: 系统可以检测伏笔逾期（超过预期章节未回收）
FR34: 系统可以通知作家逾期未回收的伏笔
FR35: 系统可以检测角色行为与角色档案的一致性
FR36: 系统可以检测世界观设定的前后矛盾
FR37: 系统可以扫描全部章节进行连贯性检查
FR38: 系统可以生成连贯性检查报告
FR39: 系统可以提供修复建议
FR40: 作家可以接受或忽略修复建议

**AI 编辑 (FR41-FR47):**
FR41: 系统可以在章节完成后自动触发 AI 编辑
FR42: AI 编辑可以检查角色行为一致性
FR43: AI 编辑可以识别新伏笔并标记
FR44: AI 编辑可以检查风格一致性
FR45: AI 编辑可以验证世界观规则
FR46: AI 编辑可以生成审阅报告
FR47: 系统可以通知作家审阅发现的问题

**用户管理 (FR48-FR50):**
FR48: 用户可以注册/登录系统
FR49: 用户可以管理个人偏好设置
FR50: 用户可以查看写作统计（字数、时长、章节数）

**系统管理 (FR51-FR54):**
FR51: 管理员可以配置 LLM 路由策略
FR52: 管理员可以查看 LLM 使用统计
FR53: 系统可以监控 AI 生成进度并实时推送
FR54: 系统可以管理本地存储的数据

### Non-Functional Requirements (NFRs)

**性能 (NFR-P1-P5):**
NFR-P1: 编辑器键盘输入响应时间 < 50ms
NFR-P2: 应用页面切换时间 < 100ms
NFR-P3: 首屏加载时间 < 3 秒
NFR-P4: AI 草稿生成时间 < 10 秒（2000 字）
NFR-P5: 风格分析时间 < 60 秒（10 万字）

**安全 (NFR-S1-S3):**
NFR-S1: 用户作品数据本地加密存储
NFR-S2: 云端传输使用 HTTPS 加密
NFR-S3: 自动保存功能每 30 秒触发

**可扩展性 (NFR-SC1-SC2):**
NFR-SC1: 支持单用户 100 万字 + 的小说管理
NFR-SC2: 支持 1000+ 并发用户（如部署为 SaaS）

**集成 (NFR-I1-I3):**
NFR-I1: 支持 Claude/OpenAI/Ollama 多种 LLM
NFR-I2: LLM API 失败时自动切换到备用
NFR-I3: 支持 TXT/EPUB 格式导入导出

**可靠性 (NFR-R1-R2):**
NFR-R1: 数据零丢失（实时保存 + 版本历史）
NFR-R2: 支持版本回滚到任意历史状态

### Additional Requirements (Architecture)

- **技术栈**: Rust 后端 (Axum) + Vue 3 前端 (TypeScript + Composition API)
- **数据库**: SQLite + 文件系统混合存储
- **向量索引**: HNSW 用于语义检索
- **LLM 路由**: 混合模式（本地 Ollama + 云端 Claude/OpenAI）
- **实时通信**: WebSocket 用于进度推送和通知
- **上下文管理**: 分层摘要 + 向量检索，支持 100 万字+连贯性
- **Starter Template**: 无特定模板要求，从零开始构建

### UX Design Requirements

UX-DR1: 实现 VS Code 风格三栏布局（侧边栏 240px + 编辑器 + 功能面板 300px，均可收起）
UX-DR2: 支持一键心流模式（隐藏所有面板，仅保留编辑器）
UX-DR3: 实现 Cursor 式行内 AI 操作菜单（选中文本→浮动菜单→续写/润色/扩写）
UX-DR4: 自定义深色主题设计令牌（背景#1E1E1E、文字#D4D4D4、强调色#4EC9B0）
UX-DR5: 实现风格雷达图可视化（Chart.js/ECharts，七层风格特征）
UX-DR6: 实现伏笔追踪器组件（状态标签+倒计时指示器，状态：活跃/已回收/废弃/逾期）
UX-DR7: 实现 AI 操作菜单组件（浮动菜单，支持键盘导航↑↓Enter/Esc）
UX-DR8: 实现状态栏组件（字数、风格匹配度、保存状态、WebSocket 连接状态）
UX-DR9: 基于 Naive UI 的深色主题定制（Button、Input、Modal 等基础组件）
UX-DR10: 实现骨架屏 + Spinner 加载状态（分步骤文案，如"正在分析词汇层...（2/7）"）
UX-DR11: 实现 Toast 通知系统（顶部居中，成功 3 秒/错误 5 秒自动消失）
UX-DR12: 实现空状态组件（无章节/无风格档案/无伏笔，带行动号召按钮）

### FR Coverage Map

| Epic | FRs Covered | NFRs Covered | UX-DRs Covered |
|------|-------------|--------------|----------------|
| Epic 1: 项目初始化与风格管理 | FR1-FR14, FR48-FR50 | NFR-P3, NFR-S1, NFR-I3 | UX-DR4, UX-DR5, UX-DR9, UX-DR10, UX-DR11, UX-DR12 |
| Epic 2: 创作编辑器核心功能 | FR15-FR23, FR54 | NFR-P1, NFR-P2, NFR-R1, NFR-R2 | UX-DR1, UX-DR2, UX-DR4, UX-DR8, UX-DR9 |
| Epic 3: AI 辅助生成 | FR24-FR30, FR53 | NFR-P4, NFR-I1, NFR-I2 | UX-DR3, UX-DR7, UX-DR8, UX-DR10 |
| Epic 4: 连贯性管理 | FR31-FR47 | NFR-SC1, NFR-I1 | UX-DR6, UX-DR8, UX-DR11 |
| Epic 5: 系统管理与统计 | FR51-FR52, FR50 | NFR-S2, NFR-SC2 | UX-DR8, UX-DR9 |

## Epic List

1. **Epic 1: 项目初始化与风格管理** - 风格分析、风格档案、项目创建
2. **Epic 2: 创作编辑器核心功能** - 编辑器、章节管理、自动保存、版本历史
3. **Epic 3: AI 辅助生成** - AI 续写/润色/扩写、风格约束生成、进度推送
4. **Epic 4: 连贯性管理** - 伏笔追踪、角色一致性检查、AI 编辑自动审阅
5. **Epic 5: 系统管理与统计** - LLM 路由配置、写作统计、系统设置

---

## Epic 1: 项目初始化与风格管理

**Goal:** 实现用户注册/登录、风格档案创建与分析、项目创建功能，为后续创作提供基础。

### Story 1.1: 用户注册/登录系统

As a 新用户，
I want 注册和登录系统，
So that 我可以管理个人项目和风格档案。

**Acceptance Criteria:**

**Given** 用户访问应用
**When** 用户点击注册并填写用户名/密码
**Then** 系统创建用户账户并自动登录
**And** 用户数据本地加密存储

**Given** 已注册用户
**When** 用户输入正确凭据
**Then** 系统验证通过并登录
**And** 加载用户的项目和风格档案列表

---

### Story 1.2: 上传小说进行风格分析

As a 作家，
I want 上传我的代表作用于风格分析，
So that 系统可以学习我的写作风格。

**Acceptance Criteria:**

**Given** 用户进入风格管理页面
**When** 用户点击"上传参考小说"并选择 TXT/EPUB 文件
**Then** 系统显示上传进度条
**And** 文件上传完成后自动开始分析

**Given** 用户上传文件
**When** 文件大小超过 10MB
**Then** 系统提示分段处理或建议精简文件
**And** 支持取消上传

---

### Story 1.3: 词汇层和句式层特征提取

As a 系统，
I want 提取文本的词汇层和句式层特征，
So that 量化用户的写作风格。

**Acceptance Criteria:**

**Given** 文本上传完成
**When** 系统执行词汇层分析
**Then** 提取常用形容词/动词/副词列表
**And** 计算词汇丰富度（Type-Token Ratio）

**Given** 词汇分析完成
**When** 系统执行句式层分析
**Then** 统计平均句长、句长分布
**And** 计算短句/中句/长句比例

---

### Story 1.4: 修辞层和叙事层特征提取

As a 系统，
I want 提取文本的修辞层和叙事层特征，
So that 捕捉用户的修辞和叙事风格。

**Acceptance Criteria:**

**Given** 句式分析完成
**When** 系统执行修辞层分析
**Then** 识别隐喻/明喻/排比等修辞手法频率
**And** 检测感官细节偏好（视觉/听觉/触觉等）

**Given** 修辞分析完成
**When** 系统执行叙事层分析
**Then** 识别叙事视角（第一/第三人称）
**And** 计算展示/讲述比例（Show vs Tell）

---

### Story 1.5: 情感层和节奏层特征提取

As a 系统，
I want 提取文本的情感层和节奏层特征，
So that 理解用户的情感表达和节奏控制风格。

**Acceptance Criteria:**

**Given** 叙事分析完成
**When** 系统执行情感层分析
**Then** 识别整体情感基调（史诗感/压抑/轻松等）
**And** 计算情感波动幅度

**Given** 情感分析完成
**When** 系统执行节奏层分析
**Then** 统计章节平均长度、场景切换频率
**And** 检测悬念结尾使用频率

---

### Story 1.6: 对话层和描写层特征提取

As a 系统，
I want 提取文本的对话层和描写层特征，
So that 完成七层风格特征分析。

**Acceptance Criteria:**

**Given** 节奏分析完成
**When** 系统执行对话层分析
**Then** 统计对话比例、角色声音区分度
**And** 检测对话标签使用习惯

**Given** 对话分析完成
**When** 系统执行描写层分析
**Then** 统计描写比例、详细程度
**And** 识别描写偏好（动作/环境/心理）

---

### Story 1.7: 生成风格向量化表示

As a 系统，
I want 将七层风格特征转换为 128 维向量，
So that 支持风格相似度计算和混合。

**Acceptance Criteria:**

**Given** 七层分析全部完成
**When** 系统执行向量化
**Then** 生成 128 维风格向量
**And** 各层权重符合预设（词汇 0.15/句式 0.15/叙事 0.20 等）

---

### Story 1.8: 显示风格分析报告

As a 作家，
I want 查看我的风格分析报告，
So that 我可以确认系统准确捕捉了我的风格。

**Acceptance Criteria:**

**Given** 风格向量化完成
**When** 系统生成报告
**Then** 显示风格雷达图（七层特征）
**And** 展示示例段落及风格标注

**Given** 用户查看报告
**When** 用户认为风格准确
**Then** 可以点击"保存风格档案"
**And** 档案保存到本地数据库

---

### Story 1.9: 保存和管理风格档案

As a 作家，
I want 保存和管理我的风格档案，
So that 我可以在创作时使用这些风格。

**Acceptance Criteria:**

**Given** 用户确认风格报告
**When** 用户点击"保存风格"
**Then** 系统将 StyleProfile 保存到 SQLite
**And** 风格向量保存到 style_profiles 表

**Given** 已有风格档案
**When** 用户进入风格库
**Then** 显示所有已保存的风格档案列表
**And** 支持查看、使用、删除操作

---

### Story 1.10: 创建小说项目并选择风格

As a 作家，
I want 创建新的小说项目并选择风格档案，
So that 我可以开始创作。

**Acceptance Criteria:**

**Given** 用户有至少一个风格档案
**When** 用户点击"创建新项目"
**Then** 输入项目名称和描述
**And** 选择一个风格档案关联

**Given** 项目创建完成
**When** 用户进入项目
**Then** 显示空状态（无章节）
**And** 提供"创建第一章"按钮

---

### Story 1.11: 混合多个风格档案

As a 作家，
I want 混合多个风格档案生成新风格，
So that 我可以创造独特的混合风格。

**Acceptance Criteria:**

**Given** 用户有至少两个风格档案
**When** 用户进入风格混合器
**Then** 选择多个风格并设置权重（如 50% 金庸 + 50% 古龙）
**And** 预览混合风格的雷达图

**Given** 混合风格预览完成
**When** 用户确认混合效果
**Then** 保存为新风格档案
**And** 可在创作时使用

---

### Story 1.12: 计算两段文本的风格相似度

As a 系统，
I want 计算两段文本的风格相似度，
So that 可以评估生成内容与目标风格的匹配度。

**Acceptance Criteria:**

**Given** 两个风格向量
**When** 系统计算相似度
**Then** 使用余弦相似度算法
**And** 返回 0-1 之间的相似度分数

---

---

## Epic 2: 创作编辑器核心功能

**Goal:** 实现沉浸式创作编辑器、章节管理、自动保存和版本历史功能。

### Story 2.1: 实现 VS Code 风格三栏布局

As a 作家，
I want 一个可收起的三栏布局界面，
So that 我可以根据需要调整工作区。

**Acceptance Criteria:**

**Given** 用户进入项目
**When** 界面加载完成
**Then** 显示左侧边栏（240px，章节树/角色列表）+ 主编辑区 + 右侧功能面板（300px，AI 助手/伏笔追踪）
**And** 所有面板支持 Cmd+B 快捷键收起/展开

**Given** 侧边栏收起状态
**When** 用户 hover 到收起的侧边栏区域
**Then** 显示图标和 tooltip 提示
**And** 点击可快速展开

---

### Story 2.2: 集成 Tiptap 富文本编辑器

As a 作家，
I want 一个专业的创作编辑器，
So that 我可以专注写作。

**Acceptance Criteria:**

**Given** 用户进入章节
**When** 编辑器加载完成
**Then** 显示无边框、沉浸式编辑区域
**And** 使用等宽字体（JetBrains Mono/Fira Code 可选）

**Given** 编辑器加载完成
**When** 用户在编辑器中输入
**Then** 键盘响应时间 < 50ms
**And** 支持基本 Markdown 语法（标题/粗体/列表）

---

### Story 2.3: 创建和编辑章节

As a 作家，
I want 创建和编辑章节，
So that 我可以组织我的小说内容。

**Acceptance Criteria:**

**Given** 用户在项目中
**When** 用户点击"创建章节"
**Then** 输入章节标题和可选大纲
**And** 编辑器打开，状态为"草稿"

**Given** 章节已创建
**When** 用户在侧边栏点击章节
**Then** 编辑器加载章节内容
**And** 显示章节状态和字数统计

---

### Story 2.4: 实时自动保存

As a 作家，
I want 系统自动保存我的写作进度，
So that 我不需要担心数据丢失。

**Acceptance Criteria:**

**Given** 用户在编辑章节
**When** 用户输入内容
**Then** 系统每 30 秒自动保存
**And** 状态栏显示"已保存"时间和下次保存倒计时

**Given** 自动保存触发
**When** 保存完成
**Then** 章节内容写入文件系统
**And** 数据库更新 updated_at 时间戳

---

### Story 2.5: 章节历史版本查看

As a 作家，
I want 查看章节的历史版本，
So that 我可以找回之前的内容。

**Acceptance Criteria:**

**Given** 章节有多个保存版本
**When** 用户点击"查看历史"
**Then** 显示版本列表（按时间倒序）
**And** 每个版本显示保存时间和字数

**Given** 用户选择一个历史版本
**When** 用户点击"预览"
**Then** 显示该版本的内容快照
**And** 与当前版本并排对比（差异高亮）

---

### Story 2.6: 回滚到历史版本

As a 作家，
I want 回滚到历史版本，
So that 我可以撤销不满意的修改。

**Acceptance Criteria:**

**Given** 用户预览历史版本
**When** 用户点击"恢复此版本"
**Then** 显示确认对话框（此操作将覆盖当前内容）
**And** 用户确认后，当前内容保存为新版本，旧版本内容恢复

**Given** 恢复操作完成
**When** 恢复成功
**Then** 显示 Toast 通知"已恢复到版本 X"
**And** 编辑器显示恢复后的内容

---

### Story 2.7: 导出章节为 TXT/EPUB

As a 作家，
I want 导出章节为 TXT 或 EPUB 格式，
So that 我可以分享或备份我的作品。

**Acceptance Criteria:**

**Given** 用户选择章节
**When** 用户点击"导出"
**Then** 选择导出格式（TXT/EPUB）
**And** 文件下载到本地

**Given** EPUB 导出
**When** 导出完成
**Then** EPUB 包含章节标题、内容、基本元数据
**And** 使用标准 EPUB 3 格式

---

### Story 2.8: 自定义深色主题

As a 作家，
I want 一个专业的深色主题，
So that 我可以长时间舒适地写作。

**Acceptance Criteria:**

**Given** 应用加载
**When** 主题初始化
**Then** 使用深色设计令牌（背景#1E1E1E、文字#D4D4D4、边框#3C3C3C）
**And** 功能色（强调#4EC9B0、成功#6A9955、警告#D19A66、错误#F48771）

**Given** 深色主题应用
**When** 对比度检查
**Then** 正文文字对比度 ≥ 4.5:1（AA 标准）
**And** 大标题对比度 ≥ 3:1

---

### Story 2.9: 一键心流模式

As a 作家，
I want 一键隐藏所有面板，
So that 我可以进入专注创作状态。

**Acceptance Criteria:**

**Given** 用户在编辑模式
**When** 用户点击"心流模式"按钮或按 F11
**Then** 隐藏侧边栏和右侧功能面板
**And** 仅保留编辑器和状态栏

**Given** 心流模式激活
**When** 用户再次点击"退出心流模式"或按 F11
**Then** 恢复所有面板
**And** 保持之前的展开/收起状态

---

### Story 2.10: 状态栏组件

As a 作家，
I want 底部状态栏显示全局状态，
So that 我可以随时了解应用状态。

**Acceptance Criteria:**

**Given** 应用加载完成
**When** 状态栏渲染
**Then** 左侧显示保存状态（已保存/保存中）
**And** 中间显示字数统计和风格匹配度（如适用）
**And** 右侧显示 WebSocket 连接状态和通知角标

**Given** 状态变更
**When** 状态栏更新
**Then** 使用 ARIA live region 通知屏幕阅读器
**And** 视觉更新平滑不突兀

---

---

## Epic 3: AI 辅助生成

**Goal:** 实现 AI 续写/润色/扩写功能，风格约束生成，实时进度推送。

### Story 3.1: 选中文本唤起 AI 操作菜单

As a 作家，
I want 选中文本后快速唤起 AI 操作菜单，
So that 我可以使用 AI 辅助功能。

**Acceptance Criteria:**

**Given** 用户在编辑器中选中文本
**When** 用户按下 Cmd+K 或右键点击
**Then** 显示浮动 AI 操作菜单（续写/润色/扩写）
**And** 菜单支持键盘导航（↑↓选择，Enter 确认，Esc 关闭）

**Given** AI 菜单显示
**When** 用户点击菜单外部或按 Esc
**Then** 菜单关闭
**And** 选区保持

---

### Story 3.2: AI 续写功能

As a 作家，
I want 使用 AI 续写我的文本，
So that 我可以克服写作瓶颈。

**Acceptance Criteria:**

**Given** 用户选择续写操作
**When** 用户确认续写
**Then** 发送续写请求到后端（包含上下文、风格约束）
**And** 状态栏显示生成进度（不打断编辑区）

**Given** AI 生成完成
**When** 生成内容返回
**Then** 新内容以浅色背景高亮显示
**And** 侧边显示风格匹配度评分

**Given** 用户查看生成内容
**When** 用户点击"接受"
**Then** 内容融入正文
**And** 自动保存

---

### Story 3.3: AI 润色功能

As a 作家，
I want 使用 AI 润色我的文本，
So that 我可以改进已有内容。

**Acceptance Criteria:**

**Given** 用户选中一段文本并选择润色
**When** 用户确认润色
**Then** 发送润色请求到后端（包含原文、风格约束）
**And** 显示生成进度

**Given** 润色完成
**When** 润色内容返回
**Then** 并排显示原文和润色后内容（差异高亮）
**And** 用户可选择接受/拒绝/部分接受

---

### Story 3.4: AI 扩写功能

As a 作家，
I want 使用 AI 将大纲扩写为完整场景，
So that 我可以快速将想法转换为内容。

**Acceptance Criteria:**

**Given** 用户选中一段大纲文字并选择扩写
**When** 用户确认扩写
**Then** 发送扩写请求（包含大纲、风格约束、上下文）
**And** 显示生成进度

**Given** 扩写完成
**When** 扩写内容返回
**Then** 高亮显示新生成的内容
**And** 显示风格匹配度评分

---

### Story 3.5: 风格约束生成

As a 系统，
I want 在 AI 生成时应用风格约束，
So that 生成内容像作者自己写的。

**Acceptance Criteria:**

**Given** 项目有关联的风格档案
**When** 构建 AI 生成请求
**Then** 从风格档案提取特征（词汇/句式/修辞等）
**And** 构建风格化 Prompt

**Given** 风格化 Prompt 构建完成
**When** 发送到 LLM
**Then** LLM 按照风格约束生成内容
**And** 生成内容体现目标风格特征

---

### Story 3.6: 检测生成内容的风格匹配度

As a 系统，
I want 检测生成内容与目标风格的匹配度，
So that 用户可以判断是否需要调整。

**Acceptance Criteria:**

**Given** AI 生成内容返回
**When** 系统分析匹配度
**Then** 提取生成内容的风格特征
**And** 与目标风格向量计算余弦相似度

**Given** 匹配度计算完成
**When** 匹配度 < 0.7
**Then** 主动提示"风格匹配度较低，建议润色"
**And** 提供一键润色选项

---

### Story 3.7: 配置 AI 自主性级别

As a 作家，
I want 配置 AI 的自主性级别，
So that 我可以控制 AI 的介入程度。

**Acceptance Criteria:**

**Given** 用户进入 AI 设置
**When** 用户调整自主性滑块
**Then** 选择自主性级别（完全手动/确认后接受/自动接受）
**And** 设置保存到项目偏好

**Given** 自主性级别设置
**When** AI 生成完成
**Then** 根据自主性级别决定是否需要确认
**And** 自动接受模式下内容直接融入正文

---

### Story 3.8: WebSocket 实时进度推送

As a 用户，
I want 实时看到 AI 生成进度，
So that 我知道系统正在工作。

**Acceptance Criteria:**

**Given** AI 生成请求发送
**When** 后端开始处理
**Then** 通过 WebSocket 推送进度消息
**And** 状态栏显示进度条和百分比

**Given** 生成进行中
**When** 用户刷新页面或网络中断
**Then** 重新连接后恢复进度显示
**And** 生成完成后通知用户

---

### Story 3.9: LLM 路由和自动切换

As a 系统，
I want 根据任务类型选择合适的 LLM，
So that 平衡成本和质量。

**Acceptance Criteria:**

**Given** AI 生成请求
**When** 路由策略检查
**Then** 根据任务类型选择模型（续写→云端/润色→云端/检测→本地）
**And** 如主模型失败，自动切换到备用模型

**Given** 备用模型也失败
**When** 所有 LLM 都不可用
**Then** 显示错误通知并提供重试选项
**And** 保存草稿状态防止数据丢失

---

---

## Epic 4: 连贯性管理

**Goal:** 实现伏笔追踪、角色一致性检查、AI 编辑自动审阅功能。

### Story 4.1: AI 自动识别伏笔

As a 系统，
I want 自动识别文本中的伏笔，
So that 可以追踪和管理伏笔。

**Acceptance Criteria:**

**Given** 章节保存完成
**When** AI 伏笔识别触发
**Then** 使用 NLP 模型检测伏笔语言模式
**And** 为每个识别的伏笔分配置信度评分

**Given** 伏笔识别完成
**When** 置信度 > 阈值
**Then** 创建伏笔记录（状态=活跃）
**And** 在伏笔追踪器中显示

---

### Story 4.2: 伏笔状态追踪

As a 系统，
I want 追踪伏笔的状态，
So that 用户可以管理伏笔生命周期。

**Acceptance Criteria:**

**Given** 伏笔已识别
**When** 伏笔状态变更
**Then** 支持状态：活跃（黄色）/已回收（绿色）/废弃（灰色）/逾期（红色）
**And** 状态变更保存到数据库

**Given** 用户手动标记伏笔
**When** 用户选择状态
**Then** 更新伏笔状态
**And** 刷新伏笔列表显示

---

### Story 4.3: 伏笔逾期检测和警告

As a 系统，
I want 检测逾期未回收的伏笔，
So that 提醒用户及时回收。

**Acceptance Criteria:**

**Given** 伏笔有预期回收章节
**When** 当前章节 > 预期回收章节
**Then** 标记伏笔为"逾期"状态
**And** 在状态栏显示警告角标

**Given** 有逾期伏笔
**When** 用户查看伏笔追踪器
**Then** 逾期伏笔用红色高亮
**And** 显示"已超过 X 章未回收"提示

---

### Story 4.4: 角色行为一致性检查

As a 系统，
I want 检查角色行为与角色档案的一致性，
So that 避免角色崩坏（OOC）。

**Acceptance Criteria:**

**Given** 章节保存完成
**When** AI 编辑触发角色检查
**Then** 提取章节中角色的行为和对话
**And** 与角色档案中的规则（Never/Always）对比

**Given** 发现不一致
**When** 不一致严重度 > 阈值
**Then** 生成审阅报告条目
**And** 通知用户查看

---

### Story 4.5: 世界观设定一致性检查

As a 系统，
I want 检查世界观设定的前后矛盾，
So that 避免世界观漏洞。

**Acceptance Criteria:**

**Given** 章节保存完成
**When** AI 编辑触发世界观检查
**Then** 扫描章节中的世界观描述（魔法体系/科技设定/地理等）
**And** 与已有世界观设定对比

**Given** 发现矛盾
**When** 矛盾确认
**Then** 生成修复建议
**And** 用户可选择接受/忽略

---

### Story 4.6: 全书连贯性扫描

As a 系统，
I want 扫描全部章节进行连贯性检查，
So that 发现长篇创作中的潜在问题。

**Acceptance Criteria:**

**Given** 用户手动触发或系统自动触发（每 10 章）
**When** 连贯性扫描开始
**Then** 后台扫描所有章节（角色行为/伏笔状态/世界观/情节线）
**And** 通过 WebSocket 推送进度

**Given** 扫描完成
**When** 生成检查报告
**Then** 按严重程度排序问题列表
**And** 提供修复建议和跳转链接

---

### Story 4.7: AI 编辑自动触发

As a 系统，
I want 在章节完成后自动进行 AI 编辑，
So that 用户可以及时发现潜在问题。

**Acceptance Criteria:**

**Given** 用户保存章节（完成状态）
**When** AI 编辑自动触发
**Then** 检查角色行为一致性/伏笔识别/风格一致性/世界观规则
**And** 状态栏显示小图标旋转（审阅中）

**Given** AI 编辑完成
**When** 发现问题
**Then** 显示通知角标和审阅报告面板
**And** 用户可以查看并选择性接受建议

**Given** AI 编辑完成
**When** 未发现问题
**Then** 静默完成，不通知用户
**And** 记录审阅日志

---

### Story 4.8: 伏笔地图可视化

As a 作家，
I want 可视化查看所有伏笔的分布，
So that 我可以全局把握伏笔布局。

**Acceptance Criteria:**

**Given** 用户进入伏笔地图
**When** 地图加载
**Then** 显示所有章节节点
**And** 用颜色标记伏笔状态（黄/绿/灰/红）

**Given** 用户点击伏笔节点
**When** 节点选中
**Then** 显示伏笔详情（内容/状态/预期回收章节）
**And** 支持拖拽调整预期回收章节

---

### Story 4.9: 生成连贯性检查报告

As a 用户，
I want 查看连贯性检查报告，
So that 我可以了解并修复潜在问题。

**Acceptance Criteria:**

**Given** 连贯性扫描完成
**When** 报告生成
**Then** 按类别分组问题（角色/伏笔/世界观/情节）
**And** 每个问题显示严重度和修复建议

**Given** 用户查看报告
**When** 用户点击问题
**Then** 跳转到对应章节位置
**And** 高亮问题文本

---

---

## Epic 5: 系统管理与统计

**Goal:** 实现 LLM 路由配置、写作统计、系统设置功能。

### Story 5.1: 配置 LLM 路由策略

As a 管理员，
I want 配置 LLM 路由策略，
So that 我可以平衡成本和质量。

**Acceptance Criteria:**

**Given** 管理员进入 LLM 配置页面
**When** 配置页面加载
**Then** 显示当前路由策略和各任务类型映射
**And** 显示各 LLM 的使用统计和成本

**Given** 管理员调整路由策略
**When** 保存配置
**Then** 配置立即生效
**And** 后续 AI 请求使用新策略

---

### Story 5.2: 查看 LLM 使用统计

As a 管理员，
I want 查看 LLM 使用统计，
So that 我可以优化成本和性能。

**Acceptance Criteria:**

**Given** 管理员进入统计页面
**When** 统计页面加载
**Then** 显示各 LLM 的调用次数、Token 消耗、成本估算
**And** 支持按时间范围过滤（日/周/月）

**Given** 统计数据
**When** 数据量较大
**Then** 使用图表可视化（折线图/柱状图）
**And** 支持导出 CSV 格式

---

### Story 5.3: 写作统计

As a 作家，
I want 查看我的写作统计，
So that 我可以了解自己的创作习惯。

**Acceptance Criteria:**

**Given** 用户进入写作统计页面
**When** 统计页面加载
**Then** 显示字数统计（总字数/今日字数/平均每日字数）
**And** 显示时长统计（总时长/平均每次时长）

**Given** 用户查看历史
**When** 选择时间范围
**Then** 显示字数增长曲线（按日/按周）
**And** 显示章节完成里程碑

---

### Story 5.4: 管理个人偏好设置

As a 用户，
I want 管理个人偏好设置，
So that 我可以自定义使用体验。

**Acceptance Criteria:**

**Given** 用户进入设置页面
**When** 设置页面加载
**Then** 显示编辑器设置（字体/字号/行高）
**And** 显示 AI 设置（自主性级别/默认操作）

**Given** 用户修改设置
**When** 保存设置
**Then** 设置保存到本地数据库
**And** 立即应用到界面

---

### Story 5.5: 管理本地存储数据

As a 系统，
I want 管理本地存储的数据，
So that 确保数据安全和性能。

**Acceptance Criteria:**

**Given** 系统启动
**When** 数据层初始化
**Then** 连接 SQLite 数据库
**And** 检查数据库完整性

**Given** 用户数据
**When** 数据存储
**Then** 使用加密算法加密敏感数据
**And** 文件系统存储原始文本内容

**Given** 数据库大小超过阈值
**When** 性能检查触发
**Then** 提示用户进行数据库优化
**And** 支持清理旧版本释放空间

---

**Confirm the Requirements are complete and correct to [C] continue:**

- 所有功能需求 (FRs) 已提取并分解为 Story
- 所有非功能需求 (NFRs) 已映射到对应 Epic
- 架构技术要求已整合到相关 Story 中
- UX 设计需求已分解为具体的 Story

请确认需求是否完整准确，是否有任何补充或修正？
