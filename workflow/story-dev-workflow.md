# Story 开发工作流

> 单仓库、无 Git Submodule 时改用 [story-dev-workflow-single-repo.md](./story-dev-workflow-single-repo.md)。

本文件描述从 Story 创建到合并的完整开发流程（**父仓库 + `backend` 子模块**）。AI Agent 按此流程执行 Story 开发。

---

## 启动确认

工作流启动时，依次弹出两个确认框，用户回复后立即开始执行。

### 第一步：可选步骤（多选）

使用 `AskUserQuestion` 工具，**multiSelect: true**，询问：

```
question: "请选择需要执行的可选步骤？"
header: "可选步骤"
multiSelect: true
options:
  - label: "编译验证"
    description: "执行 gradlew compileKotlin + compileJava，开发完成后验证编译通过"
  - label: "单元测试"
    description: "执行 gradlew test，使用 H2 内存数据库，确保所有测试通过"
  - label: "代码审查"
    description: "执行 bmad-code-review，审查代码质量，根据结果修复关键问题"
  - label: "Worktree 模式"
    description: "在 backend 子模块中用 git worktree 创建隔离工作目录，支持多 Story 并行开发互不干扰"
  - label: "以上均不需要"
    description: "跳过所有可选步骤，直接按默认流程执行"
```

用户选择"以上均不需要"或仅选该项 → 四项可选步骤均跳过。其余选项按选中状态生效。

### 第二步：确认模式（单选）

使用 `AskUserQuestion` 工具，**multiSelect: false**，询问：

```
question: "请选择执行过程中的确认模式？"
header: "确认模式"
multiSelect: false
options:
  - label: "AI 判断"
    description: "由 AI 根据步骤风险决定是否暂停确认（推荐）"
  - label: "全部确认"
    description: "每个步骤完成后暂停，展示结果并等待确认后继续"
  - label: "全部跳过"
    description: "所有步骤自动执行，不暂停"
```

### AI 判断规则

**必须确认**：步骤 2（Story 规格）、步骤 3（实现结果）、步骤 6（审查结果，若启用）、步骤 8（PR）。

**可跳过确认**：步骤 1、4、5、7、9；步骤 10 由用户决定。

---

## 跨阶段上下文（权威）

后端操作**只认一个目录**：`$BACKEND_ROOT`（勿在 `backend/` 主克隆与 `worktrees/...` 间混用）。

### 会话变量（步骤 1 完成后固化并全程携带）

| 变量 | 含义 |

|------|------|

| `BACKEND_ROOT` | 后端仓库根（含 `.git`）。普通：`backend/`；Worktree：父仓库根下 `worktrees/<BRANCH>`（例 `worktrees/story/4-5-foo`）。 |

| `BACKEND_ROOT_ABS` | 上列目录的绝对路径；**跨会话接力必填**。 |

| `BRANCH` | 如 `story/4-5-move-in-inspection-pc`。 |

| `WORKTREE_MODE` | `true` = worktree；`false` = 普通。 |

**执行范围**：`gradlew`、`git`、`gh`、代码审查均在 `$BACKEND_ROOT`；审查与 dev 同路径。

**接力输出**：须含 `BACKEND_ROOT_ABS`、`BRANCH`、`WORKTREE_MODE`（可选再写相对路径 `BACKEND_ROOT`）。

### 跨会话启动词（仅两处）

触发与 `CLAUDE.md`「dev-story」一致：`start story` / `开发 story` + Story 编号；Agent 读本文件并按步执行。下列为接力块（粘贴到下一会话首条或附在触发句后）。

**变量**：`{epic}-{story}`、`{BRANCH}`、`{BACKEND_ROOT}`、`{BACKEND_ROOT_ABS}`、`{WORKTREE_MODE}` 由会话变量填实。

---

**位置一** — 步骤 2 完成且本会话终止（不进入步骤 3）

```
【跨会话接力 · 继续实现 Story】
开发 story {epic}-{story}（或 start story {epic}-{story}）
同条消息须含：已执行步骤 1～2；下一步从步骤 3（bmad-dev-story）起，勿重复 bmad-create-story；
BRANCH={BRANCH}；WORKTREE_MODE={WORKTREE_MODE}；BACKEND_ROOT={BACKEND_ROOT}；BACKEND_ROOT_ABS={BACKEND_ROOT_ABS}
示例：开发 story {epic}-{story}。创建 Story 已做完；从步骤 3 继续。BACKEND_ROOT_ABS=... BRANCH=... WORKTREE_MODE=...
```

---

**位置二** — 步骤 3 完成且本会话终止（未跑步骤 6，或用户结束）

```
【跨会话接力 · 仅代码审查】
开发 story {epic}-{story}（或 start story {epic}-{story}）
同条消息须含：已执行步骤 1～3；下一步仅在 BACKEND_ROOT_ABS 执行步骤 6（bmad-code-review）；
BRANCH=...；WORKTREE_MODE=...；BACKEND_ROOT_ABS=...
示例：开发 story {epic}-{story}。实现已完成；仅审查。BACKEND_ROOT_ABS=... BRANCH=... WORKTREE_MODE=...
```

**不输出位置二**：本会话已执行步骤 6 且未要求接力。

---

## 前置条件

- `_bmad-output/implementation-artifacts/sprint-status.yaml` 存在；Story 为 `backlog` 或 `ready-for-dev`

- 已读 `_bmad-output/project-context.md`、`docs/BACKEND_CODE_STYLE.md`

---

## 工作流步骤

### 1. 同步代码 & 创建分支

> 先于步骤 2，保证 `bmad-create-story` 扫代码时在目标分支上。

- 拉取（在 `backend/`）：`cd backend && git checkout main && git pull origin main`

- 分支名：`story/{epic}-{story}-{slug}`

- **基点**：查 sprint 依赖 → `gh pr list` 判断依赖 PR；已合并则基 `main`，未合并则基依赖分支（PR 目标同）

#### 普通模式

```bash
git checkout -b story/1-1-xxx [基点分支]
```

`BACKEND_ROOT`= 父仓库下 `backend/`；`WORKTREE_MODE=false`。`BACKEND_ROOT_ABS`：`cd backend && pwd`（PowerShell：`(Get-Location).Path`）。
#### Worktree 模式（启动勾选时）
```bash

cd backend

BRANCH="story/1-1-data-dictionary-management"

WORKTREE_DIR="../worktrees/$BRANCH"

git fetch origin

git branch $BRANCH [基点分支]

git worktree add "$WORKTREE_DIR" "$BRANCH"

```
父仓库根相对路径恒为 **`worktrees/<BRANCH>`**（与 `WORKTREE_DIR` 一致）。`BACKEND_ROOT` = 该目录；`WORKTREE_MODE=true`；`BACKEND_ROOT_ABS`：`cd "$WORKTREE_DIR" && pwd`。后续勿回主克隆改同一分支。
推送：`cd "$BACKEND_ROOT" && git push -u origin "$BRANCH"`
清理（在 **`backend/`** 子模块根）：
```bash

cd backend

git worktree remove "../worktrees/story/1-1-xxx"

git branch -d story/1-1-xxx

```
### 2. 创建 Story 文件
`backlog` 时跑 `bmad-create-story`（多在**父仓库根**写 `_bmad-output/`）；代码侧以 **`$BACKEND_ROOT` 的 `BRANCH`** 为准（见上文「跨阶段上下文」）。

### 3. 实现 Story

在 **`$BACKEND_ROOT`** 执行 `bmad-dev-story`；遵守 `project-context.md`。

### 4. 编译验证（可选）

启动勾选时执行，否则跳过。

```bash
cd "$BACKEND_ROOT" && ./gradlew :union-service-property:compileKotlin :union-service-property:compileJava
```

### 5. 单元测试（可选）

启动勾选时执行，否则跳过。

```bash
cd "$BACKEND_ROOT" && ./gradlew :union-service-property:test
```

`application-test.yaml` + H2；测通为止。
### 6. 代码审查（可选）
启动勾选时执行，否则跳过。在 **`$BACKEND_ROOT`** 跑 `bmad-code-review`，按结果修必修复项。
### 7. 提交代码
在 **`$BACKEND_ROOT`** 提交子模块变更；父仓库若有 sprint/文档另库提交则拆分。作者 `Sue <boil@vip.qq.com>`；无 Co-Authored-By；`<type>(<scope>): <subject>`（type: feat/fix/docs/style/refactor/test/chore；scope：模块名）。
### 8. 推送 & 创建 PR
在 **`$BACKEND_ROOT`** 下 `git push`、`gh`（remote = 后端仓库，常为 union-platform）。
**Issue（可选）**：`gh issue list --search "{epic}-{story} in:title" --state open --limit 20`，无则试 `in:title,body`；多条则 Web 定 `#N`。PR 正文含 **`Closes #N`**（或 `Fixes`/`Resolves`）；与 PR **同仓库** 时合并进默认分支后 GitHub 通常自动关 Issue；无 Issue 不写；跨仓库（父仓 Issue + 子模块 PR）不自动关，需手动或改 Issue 到代码仓。
**PR**：`gh pr create`；body：Summary、Test plan；有 `#N` 则 `Closes #N`。目标：`main` 或依赖分支。
### 9. 更新 Sprint 状态
`_bmad-output/implementation-artifacts/sprint-status.yaml` 中该 Story → `done`。

### 10. 合并 & 清理

用户决定合并时机；合并后删分支；Worktree 清理见步骤 1。

---

## 关键规则（速查）

| 项目 | 规则 |

|------|------|

| Git 作者 | `Sue <boil@vip.qq.com>`，无 Co-Authored-By |

| 分支 | `story/{epic}-{story}-{slug}` |

| Commit | Conventional Commits |

| 编译 / 测 / 审查 | 可选，启动勾选；编译 `:union-service-property:compileKotlin`+`compileJava`；测 `test` profile + H2 |

| Worktree | 可选；路径 `worktrees/<BRANCH>` |

| PR | 目标 `main` 或依赖分支 |

| Issue | `$BACKEND_ROOT` 下 `gh`；`Closes #N`；见步骤 8 |

