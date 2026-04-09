# CLAUDE.md — Novel Agent / noval

## 项目定位

这是一个面向**专业网络小说作家**的 AI 辅助创作项目，核心目标不是“写得更快”，而是**写得更好**：

- 减少 AI 味道，尽量贴近作者真实文风
- 支持长篇小说创作中的一致性维护
- 强化角色、情节、伏笔等长期状态管理
- 让作家把精力集中在创意，而不是重复劳动

当前仓库是一个 **greenfield 单仓库项目**，现阶段以 **BMAD 工作流、PRD、架构设计、UX 产物、Sprint 跟踪** 为主，**业务代码尚未正式进入仓库**。

根据现有产物，项目目标技术栈为：

- **后端**：Rust
- **前端**：Vue 3 + TypeScript
- **架构**：前后端分离 Web 应用
- **数据层**：SQLite + 文件系统 + 向量索引
- **模型接入**：本地 LLM（如 Ollama）+ 云端 LLM（如 Claude）

---

## 仓库结构

这是一个**单仓库、无 Git Submodule** 的结构：

```text
noval/
├── _bmad/                           ← BMAD 框架、模块、技能与工作流模板
├── _bmad-output/                    ← 当前项目已生成的规划/实现产物
│   ├── planning-artifacts/          ← PRD、Epic、UX 设计规格等
│   ├── implementation-artifacts/    ← Sprint 状态、后续 Story 产物
│   └── architecture-design.md       ← 系统架构设计
├── design-artifacts/                ← 产品简报、触发地图、UX 场景、设计系统、开发资料
├── workflow/                        ← 仓库级工作流说明
├── docs/                            ← 预留文档目录（当前基本为空）
└── CLAUDE.md                        ← 当前项目协作约定
```

### 当前最重要的文档

- `_bmad-output/planning-artifacts/prd.md`：产品需求文档，定义项目目标、范围、用户与成功标准
- `_bmad-output/architecture-design.md`：系统架构设计，定义模块拆分与目标技术方案
- `_bmad-output/planning-artifacts/ux-design-specification.md`：UX 设计规格
- `_bmad-output/implementation-artifacts/sprint-status.yaml`：Epic / Story 当前状态

---

## 关键规则

### 1. 仓库类型判断

- **本项目默认按单仓库处理**
- 当前仓库**没有** `backend/` 子模块，也不是父仓库 + 子模块结构
- 任何涉及 Story 开发、分支、工作目录的操作，默认都以**当前仓库根目录**为准

### 2. 技术栈约束

- 项目描述、实现建议、目录规划应与 **Rust 后端 + Vue 3 前端** 保持一致
- 不要再沿用 Java / Kotlin / Gradle / `union-service-property` / H2 这类旧项目设定
- 若后续代码入库，命令与脚本应基于实际目录和实际工具链更新，例如：
  - Rust：`cargo fmt` / `cargo test`
  - 前端：`pnpm dev` / `pnpm test` / `pnpm build`

### 3. 当前阶段的工作重心

- 当前阶段以**需求澄清、架构收敛、UX 设计、Story 拆分、开发流程约束**为主
- 在没有真实代码目录前，不要假设已有后端模块、前端应用、数据库迁移或 CI 配置
- 若用户要求“开发 Story”或“开始实现”，先根据仓库现状判断：
  - 若只有文档产物，则先补齐 Story、实现方案、目录规划或脚手架
  - 若未来已加入代码，再按真实代码结构执行开发

### 4. Story / Sprint 跟踪

- Sprint 跟踪文件：`_bmad-output/implementation-artifacts/sprint-status.yaml`
- 当前 Story 状态定义：
  - `backlog`
  - `ready-for-dev`
  - `in-progress`
  - `review`
  - `done`
- 该文件是当前项目开发进度的权威来源

### 5. 文档优先级

当文档之间存在差异时，优先级建议如下：

1. 用户当前明确要求
2. `_bmad-output/architecture-design.md`
3. `_bmad-output/planning-artifacts/prd.md`
4. `_bmad-output/planning-artifacts/ux-design-specification.md`
5. 其他设计资料与工作流说明

---

## 工作流命令

工作流文件位于 `workflow/` 目录，供 AI Agent 按步骤执行。

### dev-story — 开发 Story

**触发**：用户说 `start story` 或 `开发 story`，后跟 Story 编号（如 `2-4`）

执行步骤：

- **默认读取** `workflow/story-dev-workflow-single-repo.md`
- 只有当仓库未来真的演变为“父仓库 + `backend` 子模块”时，才改读 `workflow/story-dev-workflow.md`

补充说明：

- 当前单仓库工作流是**正确入口**
- 其中若出现与当前项目不符的示例命令，应以本项目真实技术栈为准进行替换
- 在业务代码尚未入库前，Story 开发更多是创建 Story、细化方案、准备脚手架，而不是直接进入完整编码阶段

### submit-questions — 登记问题

**触发**：用户说 `登记问题` 或 `submit questions`

执行步骤：读取 `workflow/question-submission-workflow.md`，按工作流步骤将对话中讨论的问题登记到问题清单。

---

## 对 AI Agent 的补充约束

- 先识别当前仓库是否已有真实代码，再决定是“文档推进”还是“代码实现”
- 所有项目描述应围绕 **Novel Agent / 长篇小说 AI 创作系统**
- 讨论功能时，优先围绕以下核心能力展开：
  - 风格分析与模仿
  - 创作编辑器
  - AI 辅助生成
  - 伏笔追踪
  - 角色一致性
  - 世界观一致性
  - LLM 路由与统计
- 若要新增目录、脚手架或模块命名，应优先贴合 Rust + Vue 3 项目习惯，而不是旧的企业 Java 项目结构
