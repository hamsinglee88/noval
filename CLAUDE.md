# CLAUDE.md — Novel Agent / noval

## 项目定位

这是一个面向**专业网络小说作家**的 AI 辅助创作项目，核心目标不是"写得更快"，而是**写得更好**：

- 减少 AI 味道，尽量贴近作者真实文风
- 支持长篇小说创作中的一致性维护
- 强化角色、情节、伏笔等长期状态管理
- 让作家把精力集中在创意，而不是重复劳动

---

## 技术栈

**后端**：Rust + Axum + SQLite + SQLx
**前端**：Vue 3.4+ + TypeScript + Naive UI + Pinia
**架构**：前后端分离 Web 应用
**数据层**：SQLite + 文件系统 + 向量索引
**模型接入**：本地 LLM（如 Ollama）+ 云端 LLM（如 Claude）
**认证**：JWT (jsonwebtoken) + bcrypt 密码加密

---

## 仓库结构

```text
noval/
├── _bmad/                           ← BMAD 框架、模块、技能
├── _bmad-output/                    ← 规划/实现产物
│   ├── planning-artifacts/          ← PRD、Epic、UX 设计规格
│   ├── implementation-artifacts/    ← Sprint 状态、Story 文件
│   └── architecture-design.md       ← 系统架构设计
├── backend/                         ← Rust 后端
│   ├── src/
│   │   ├── handlers/                ← HTTP 处理器
│   │   ├── models/                  ← 数据模型
│   │   ├── services/                ← 业务逻辑
│   │   └── db.rs / errors.rs / validation.rs
│   ├── migrations/                  ← 数据库迁移
│   └── Cargo.toml
├── frontend/                        ← Vue 3 前端
│   ├── src/
│   │   ├── components/              ← 组件
│   │   ├── views/                   ← 页面
│   │   ├── stores/                  ← 状态管理 (Pinia)
│   │   ├── services/                ← API 客户端
│   │   └── router/                  ← 路由
│   └── package.json
├── design-artifacts/                ← 设计产物
├── workflow/                        ← 工作流说明
└── CLAUDE.md
```

---

## 重要文档

| 文件 | 说明 |
|------|------|
| `_bmad-output/planning-artifacts/prd.md` | 产品需求文档（54 个 FR + 14 个 NFR） |
| `_bmad-output/architecture-design.md` | 系统架构设计 |
| `_bmad-output/planning-artifacts/ux-design-specification.md` | UX 设计规格 |
| `_bmad-output/implementation-artifacts/sprint-status.yaml` | **Sprint 状态（权威）** |
| `workflow/story-dev-workflow.md` | Story 开发完整工作流 |

---

## 当前状态

**Sprint 状态**：

| Epic | 状态 | Story 进度 |
|------|------|-----------|
| Epic 1: 项目初始化与风格管理 | in-progress | 1-1 done, 1-2~1-12 ready-for-dev |
| Epic 2: 创作编辑器核心功能 | backlog | - |
| Epic 3: AI 辅助生成 | backlog | - |
| Epic 4: 连贯性管理 | backlog | - |
| Epic 5: 系统管理与统计 | backlog | - |

**已完成 Story**：
- **Story 1-1** (用户注册/登录系统) - done
  - 后端：JWT 认证、bcrypt 密码、SQLite 用户存储
  - 前端：登录/注册页面、Pinia 状态管理、AES 加密存储

---

## 关键规则

### 1. 单仓库结构

- **本项目为单仓库**，前后端代码在同一仓库根目录下
- 任何涉及 Story 开发、分支、工作目录的操作，默认都以**当前仓库根目录**为准
- 后端目录：`backend/`
- 前端目录：`frontend/`

### 2. 技术栈命令

| 任务 | 后端 | 前端 |
|------|------|------|
| 编译检查 | `cargo check` | `npm run build` |
| 单元测试 | `cargo test` | `npm test` |
| 开发模式 | - | `npm run dev` |
| 格式化 | `cargo fmt` | - |

### 3. Story / Sprint 跟踪

- Sprint 跟踪文件：`_bmad-output/implementation-artifacts/sprint-status.yaml`
- Story 状态：`backlog` → `ready-for-dev` → `in-progress` → `review` → `done`
- 该文件是项目开发进度的权威来源

### 4. 文档优先级

当文档之间存在差异时，优先级如下：

1. 用户当前明确要求
2. `_bmad-output/architecture-design.md`
3. `_bmad-output/planning-artifacts/prd.md`
4. `_bmad-output/planning-artifacts/ux-design-specification.md`
5. 其他设计资料

---

## 工作流命令

### dev-story — 开发 Story

**触发**：用户说 `start story {编号}` 或 `开发 story {编号}`（如 `start story 1-2`）

**执行**：读取 `workflow/story-dev-workflow.md`，按步骤执行：

1. 同步代码 & 创建分支
2. 创建 Story 文件（如尚未创建）
3. 实现功能
4. 可选：编译验证、单元测试、代码审查
5. 提交代码 & 推送
6. 创建 PR
7. 更新 Sprint 状态

### submit-questions — 登记问题

**触发**：用户说 `登记问题` 或 `submit questions`

**执行**：读取 `workflow/question-submission-workflow.md`，将问题登记到 `design-artifacts/questions.md`

---

## Git 约定

| 项目 | 规则 |
|------|------|
| 分支命名 | `story/{epic}-{story}-{slug}` |
| Commit 格式 | `<type>(<scope>): <subject>` |
| 作者 | `Hamsing <boil@vip.qq.com>` |
| Co-Authored | `Claude Opus 4.6 <noreply@anthropic.com>` |
| PR 目标 | `main` 或依赖分支 |

---

## 核心功能模块

| 模块 | 说明 | 对应 Epic |
|------|------|----------|
| 风格管理 | 上传文本分析风格、向量化、档案存储 | Epic 1 |
| 创作编辑器 | VS Code 风格三栏布局、章节 CRUD、自动保存 | Epic 2 |
| AI 辅助生成 | 续写、润色、扩写、风格约束 | Epic 3 |
| 连贯性管理 | 伏笔追踪、角色一致性、世界观检查 | Epic 4 |
| 系统管理 | LLM 路由、统计、用户偏好 | Epic 5 |

---

## 对 AI Agent 的补充约束

1. **先读 sprint-status.yaml** 了解当前进度，再决定下一步
2. **代码改动后更新文档**：Story 状态、Change Log 同步更新
3. **安全优先**：涉及认证、密码、Session 的代码必须遵循安全最佳实践
4. **Rust + Vue 3 项目习惯**：不要引入 Java/Gradle 等企业级旧项目约定
