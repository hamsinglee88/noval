---
status: ready-for-dev
epic: 1
story: 2
story_key: 1-2-novel-upload-analysis
last_updated: 2026-04-09
---

# Story 1.2: 上传小说进行风格分析

## Story Header

| 属性 | 值 |
|------|-----|
| **Epic** | Epic 1: 项目初始化与风格管理 |
| **Story ID** | 1.2 |
| **Story Key** | 1-2-novel-upload-analysis |
| **优先级** | P0 |
| **估算复杂度** | 中 |
| **状态** | ready-for-dev |
| **前置依赖** | Story 1.1 提供认证上下文；本 Story 不重复实现认证 |

---

## User Story Statement

**As a** 作家，  
**I want** 上传我的代表作用于风格分析，  
**So that** 系统可以学习我的写作风格。

---

## Acceptance Criteria (BDD Format)

### AC1: 上传参考小说并自动启动分析

**Given** 用户进入风格管理页面  
**When** 用户点击"上传参考小说"并选择 TXT/EPUB 文件  
**Then** 系统显示上传进度条和预估时间  
**And** 上传完成后自动创建风格分析任务  
**And** 进入分析中状态，不要求用户再次点击"开始分析"

### AC2: 大文件处理与取消上传

**Given** 用户上传文件  
**When** 文件大小超过 10MB  
**Then** 系统提示将采用分段/流式预处理或建议精简文件  
**And** 用户可以取消当前上传  
**And** 取消后不保留半完成的前端上传状态

### AC3: 文件格式与错误反馈

**Given** 用户选择文件  
**When** 文件格式不属于 TXT/EPUB 或文本解析失败  
**Then** 系统阻止分析任务创建  
**And** 明确提示原因与支持格式  
**And** 提供重试入口

### AC4: 分析任务持久化

**Given** 上传通过校验  
**When** 后端接收文件并开始预处理  
**Then** 系统在本地存储中保存源文件路径  
**And** 在 `style_analysis_tasks` 中创建任务记录  
**And** 记录至少包含 `source_file_path`、`status`、`progress`、`created_at`、`updated_at`

### AC5: 与后续 Story 的边界清晰

**Given** 上传和任务创建完成  
**When** 开发实现本 Story  
**Then** 不在本 Story 内完成七层风格特征提取  
**And** 不在本 Story 内完成 128 维向量生成  
**And** 不在本 Story 内完成最终风格报告保存  
**And** 仅为 Story 1.3-1.9 提供稳定输入和任务入口

---

## Story Requirements

### 功能需求

| ID | 需求 | 优先级 |
|----|------|--------|
| FR1.2.1 | 用户可以从风格管理页选择本地 TXT/EPUB 文件 | P0 |
| FR1.2.2 | 前端显示上传进度、预估时间、当前状态 | P0 |
| FR1.2.3 | 上传完成后系统自动创建分析任务并开始预处理 | P0 |
| FR1.2.4 | 系统校验文件类型并对解析失败给出可操作错误 | P0 |
| FR1.2.5 | 大文件上传支持分段/流式处理提示 | P0 |
| FR1.2.6 | 用户可以取消正在进行的上传请求 | P0 |
| FR1.2.7 | 系统将上传源文件路径和任务状态持久化到本地存储 | P0 |
| FR1.2.8 | 任务创建后的结果可被后续风格分析 Story 复用 | P1 |

### 非功能需求

| ID | 需求 | 优先级 |
|----|------|--------|
| NFR1.2.1 | 首屏上传入口和页面加载符合首屏 < 3 秒目标 | P0 |
| NFR1.2.2 | 上传大文件时不得一次性把整文件读入内存 | P0 |
| NFR1.2.3 | 本地存储路径和任务数据需要可恢复，避免中途中断后完全丢失上下文 | P0 |
| NFR1.2.4 | 接口和状态命名应与后续 WebSocket/分析流程保持一致 | P1 |

---

## Developer Context Section

### Story Scope Guardrails

本 Story 只负责建立"上传小说 -> 创建分析任务 -> 进入分析流程"这条通路。

**本 Story 必须完成：**
- 风格管理页的上传入口、格式说明、空状态 CTA
- TXT/EPUB 文件选择与基础校验
- 上传进度展示与取消上传
- 后端接收文件并持久化源文件
- 创建 `style_analysis_tasks` 任务记录
- 自动触发预处理/分析起点，供后续 Story 接续

**本 Story 明确不做：**
- 词汇层、句式层、修辞层、叙事层、情感层、节奏层、对话层、描写层的完整特征提取
- 128 维风格向量生成
- 风格雷达图和最终分析报告展示
- StyleProfile 的确认保存与风格库 CRUD
- 重复实现 Story 1.1 的认证、用户表、Session 逻辑

这条边界必须严格执行，否则会把 Story 1.3-1.9 的工作提前塞进 1.2，导致验收口径失真。

### 技术栈要求

**后端（Rust）：**
- **框架：** Axum
- **数据库：** SQLite + SQLx
- **存储策略：** SQLite + 本地文件系统混合存储
- **任务模型：** `style_analysis_tasks`

**前端（Vue 3）：**
- **框架：** Vue 3.4+ Composition API
- **语言：** TypeScript 5.x
- **状态管理：** Pinia
- **UI 组件：** Naive UI（深色主题）
- **HTTP 客户端：** Axios
- **实时进度扩展点：** WebSocket 事件 `style_analysis_progress`

### 架构合规要求

根据架构和 UX 文档，本 Story 需要遵循：

1. **本地优先**  
   上传源文件保存在本地文件系统，任务元数据保存在本地 SQLite。不要依赖云端对象存储。  

2. **接口命名统一**  
   上传/分析入口沿用架构中的 `POST /api/styles/analyze`。后续进度事件沿用 `style_analysis_progress`。不要临时发明另一套命名。  

3. **大文件流式处理**  
   对 `>10MB` 文件不要整文件读入内存。应采用流式写盘、分段读取或后台预处理任务。  

4. **后续 Story 可扩展**  
   本 Story 写入的任务记录必须能被 Story 1.3-1.7 接着消费，不能把上传逻辑和分析逻辑硬编码耦合死。  

5. **错误反馈可操作**  
   UX 已明确要求显示格式错误、文件过大、解析失败等原因，并给出重试入口。  

6. **取消语义限定在上传阶段**  
   本 Story 的"取消上传"以终止当前前端上传请求为主，可使用 `AbortController`。不要擅自扩展为完整的服务端分析取消系统。  

### 当前仓库现实情况

- 当前仓库还没有 Rust 后端或 Vue 前端工程骨架。
- 现有内容主要是 `_bmad-output` 规划文档和已创建的 Story 文件。
- 因此下面的文件结构属于**目标落点**，不是现存路径。

开发时如果先落地了 bootstrap/scaffold，请把本 Story 模块映射进新的真实工程结构，不要再平行造一套目录。

### API Contract

#### 推荐接口

```http
POST /api/styles/analyze
Content-Type: multipart/form-data
```

#### 请求字段

```text
file: TXT | EPUB 文件（二进制）
source_name: 可选，前端显示用原始文件名
```

#### 成功响应

```json
{
  "success": true,
  "data": {
    "task_id": "uuid",
    "status": "processing",
    "progress": 0.0,
    "source_file_path": "data/style-sources/<task-id>/source.epub"
  }
}
```

#### 失败响应

```json
{
  "success": false,
  "error": {
    "code": "UNSUPPORTED_FILE_TYPE",
    "message": "Only TXT and EPUB files are supported."
  }
}
```

**约束说明：**
- 返回 `202 Accepted` 或 `200 OK` 均可，但整个项目应统一。
- 不要在这个接口里返回最终 `StyleProfile`；那是后续 Story 的责任。
- 如果 EPUB 文本抽取需要异步处理，接口应先返回任务，再由后续流程推进。

### 数据库 Schema

本 Story 最少需要落地 `style_analysis_tasks`。  
由于架构定义里 `style_analysis_tasks.result_profile_id` 依赖 `style_profiles`，若迁移采用外键约束，则建议一并创建 `style_profiles` 基础表结构，但**不要在本 Story 内实现完整档案管理逻辑**。

```sql
CREATE TABLE style_profiles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    source_novels TEXT[],
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    vocabulary_json TEXT,
    sentence_json TEXT,
    rhetoric_json TEXT,
    narrative_json TEXT,
    emotional_json TEXT,
    pacing_json TEXT,
    dialogue_json TEXT,
    description_json TEXT,
    style_vector TEXT,
    example_passages TEXT
);

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
```

**实现建议：**
- `status` 至少支持 `pending`、`processing`、`failed`，后续可扩展 `completed`
- `progress` 在本 Story 内可先覆盖上传完成和预处理开始两个阶段，不必伪装成完整七层分析进度
- 原始文件内容建议写入本地专用目录，例如 `data/style-sources/<task-id>/`

### File Structure Requirements

### 后端目标结构

```text
backend/
└── src/
    ├── handlers/
    │   └── styles.rs                # /api/styles/analyze
    ├── models/
    │   ├── style_profile.rs         # 预留给后续 Story
    │   └── style_analysis_task.rs   # 任务模型
    ├── services/
    │   ├── style_upload_service.rs  # 文件接收、校验、写盘
    │   └── text_extract_service.rs  # TXT/EPUB 预处理入口
    ├── db/
    │   └── migrations/
    │       └── 002_create_style_analysis_tables.sql
    └── main.rs
```

### 前端目标结构

```text
frontend/
└── src/
    ├── components/
    │   └── style/
    │       ├── StyleUploader.vue    # 上传入口、进度、取消
    │       └── StyleAnalyzer.vue    # 分析中的状态容器
    ├── views/
    │   └── StyleLibraryView.vue     # 风格管理页面/空状态入口
    ├── stores/
    │   └── style.ts                 # 上传与任务状态
    └── services/
        └── style.ts                 # 调用 /api/styles/analyze
```

### Project Structure Notes

- 如果后续 bootstrap 使用单仓结构而不是 `frontend/` + `backend/`，保留相同模块分层即可，不必死守顶层目录名。
- 不要把风格上传逻辑塞进认证模块，也不要把 TXT/EPUB 解析直接写进路由处理器。
- 上传状态、任务状态、最终风格档案状态需要分开建模，避免把所有状态塞进一个 Pinia 字段。

### UX Design Requirements

### 页面与交互

- 风格管理页必须提供显式 CTA："创建风格档案" / "上传参考小说分析风格"
- 上传中状态显示：进度条 + 预估时间 + 当前状态文案
- 支持格式说明：TXT、EPUB
- 空状态文案需要引导用户开始第一次风格分析
- 错误状态需要显示"重试"而不是只显示失败

### 视觉约束

```css
:root {
  --color-bg-primary: #1E1E1E;
  --color-bg-secondary: #252526;
  --color-text-primary: #D4D4D4;
  --color-text-secondary: #858585;
  --color-accent: #4EC9B0;
  --color-warning: #D19A66;
  --color-error: #F48771;
}
```

### 上传态文案建议

- `正在上传参考小说...`
- `上传完成，正在准备分析...`
- `文件较大，将采用分段处理`
- `仅支持 TXT / EPUB 文件`

### Previous Story Intelligence

从 Story 1.1 可以提炼出以下约束：

- 用户上下文和本地 SQLite 是已定方向，本 Story 应复用这个总方向，而不是另起存储方案。
- Story 1.1 还只是 `ready-for-dev`，尚无已实现代码可复用，因此不要假定认证接口已经存在。
- 为避免重复建设，风格上传模块应通过抽象的当前用户/当前会话接口接入，而不是再次定义用户表或 Session 结构。

### Git / Repo Intelligence

- 当前目录不是 Git 仓库，无法从提交历史提炼实现模式。
- 当前也没有现成的前后端工程文件，因此不能假设已有模块命名或测试目录。

### Testing Requirements

### 后端测试（Rust）

```rust
#[tokio::test]
async fn test_upload_txt_creates_analysis_task() {
    // 上传 TXT 后创建 style_analysis_tasks 记录
}

#[tokio::test]
async fn test_upload_epub_creates_analysis_task() {
    // 上传 EPUB 后创建 style_analysis_tasks 记录
}

#[tokio::test]
async fn test_unsupported_file_type_is_rejected() {
    // PDF/Docx 等不支持格式被拒绝
}

#[tokio::test]
async fn test_large_file_uses_streaming_path() {
    // 大文件不应走整文件读入内存的实现路径
}
```

### 前端测试（Vitest）

```typescript
describe('StyleUploader', () => {
  it('shows upload progress while request is in flight', () => {
    // 上传中显示进度条
  });

  it('allows canceling the current upload request', async () => {
    // 点击取消后中断请求并清理状态
  });

  it('shows actionable error for unsupported file types', async () => {
    // 展示支持格式和重试入口
  });

  it('shows large-file segmented-processing hint', async () => {
    // >10MB 时提示分段处理
  });
});
```

### 集成测试建议

- 上传成功后应看到任务进入 `processing`
- 上传失败时不应创建脏任务记录
- 取消上传时前端状态应恢复到 idle
- 后端落盘文件路径和数据库记录必须一致

### References

- `_bmad-output/planning-artifacts/epics.md` - `Story 1.2: 上传小说进行风格分析`
- `_bmad-output/planning-artifacts/epics.md` - `Story 1.3 ~ 1.9`（用于划定边界）
- `_bmad-output/planning-artifacts/prd.md` - `Journey 1：第一次使用 - 风格分析`
- `_bmad-output/planning-artifacts/prd.md` - `FR1-FR14`, `NFR-P3`, `NFR-I3`
- `_bmad-output/planning-artifacts/ux-design-specification.md` - `Journey 1: 风格分析`
- `_bmad-output/architecture-design.md` - `3.3 风格分析工作流`
- `_bmad-output/architecture-design.md` - `5.1 RESTful Endpoints`
- `_bmad-output/architecture-design.md` - `5.2 WebSocket 事件`
- `_bmad-output/architecture-design.md` - `6.1 技术栈`
- `_bmad-output/architecture-design.md` - `6.2 核心组件`
- `_bmad-output/architecture-design.md` - `style_profiles`, `style_analysis_tasks` schema

---

## Story Completion Status

- [x] 前端：风格管理页空状态与上传入口
- [x] 前端：TXT/EPUB 选择与格式校验
- [x] 前端：上传进度、预估时间、取消上传
- [x] 后端：`/api/styles/analyze` 路由
- [x] 后端：源文件写盘与路径规范
- [x] 后端：`style_analysis_tasks` 任务记录
- [x] 后端：TXT/EPUB 预处理入口
- [x] 测试：后端上传/校验/任务创建测试
- [x] 测试：前端上传组件状态测试
- [x] 集成：上传成功、失败、取消三条主路径验证

## Change Log

- 2026-04-10：完成 Story 1-2 实现并标记为 done
  - 后端：风格分析 API、文件上传服务、数据库迁移
  - 前端：风格管理页面、上传组件、状态管理
  - 测试：cargo test 通过，npm build 通过
  - PR: https://github.com/hamsinglee88/noval/pull/1

---

**Story created:** 2026-04-09  
**Last updated:** 2026-04-10  
**Status:** done
