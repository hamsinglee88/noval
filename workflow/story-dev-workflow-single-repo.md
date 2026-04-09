# Story 开发工作流（单仓库 / 无子模块）

> 这是 `story-dev-workflow.md` 的简化版本，用于快速 Story 开发。
>
> 适用于 **Novel Agent** 项目：单仓库结构，前后端代码在同一仓库根目录下。

---

## 快速启动

**触发**：用户说 `start story {epic}-{story}` 或 `开发 story {epic}-{story}`

### 1. 确认分支策略

```bash
git checkout main && git pull origin main
git checkout -b story/{epic}-{story}-{slug}
```

### 2. 创建/更新 Story 文件

- 如 Story 尚未创建：`bmad-create-story`
- 如 Story 已存在：读取并确认验收标准

### 3. 实现功能

- 后端：`backend/` → Rust + Axum + SQLite
- 前端：`frontend/` → Vue 3 + TypeScript + Naive UI
- 运行 `bmad-dev-story` 或手动实现

### 4. 验证

```bash
# 后端
cd backend && cargo check && cargo test

# 前端
cd frontend && npm run build && npm test
```

### 5. 提交 & 推送

```bash
git add .
git commit -m "feat({scope}): implement story {epic}-{story}

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>"
git push -u origin story/{epic}-{story}-{slug}
```

### 6. 创建 PR

```bash
gh pr create --title "feat({scope}): Story {epic}-{story}" --body "Summary of changes"
```

### 7. 更新 Sprint 状态

更新 `_bmad-output/implementation-artifacts/sprint-status.yaml`：

```yaml
{epic}-{story}: done
```

---

## 技术栈参考

### 后端 (Rust)

| 组件 | 技术 |
|------|------|
| 框架 | Axum |
| 数据库 | SQLite + SQLx |
| 密码加密 | bcrypt |
| 认证 | JWT (jsonwebtoken) |
| 测试 | cargo test |

### 前端 (Vue 3)

| 组件 | 技术 |
|------|------|
| 框架 | Vue 3.4+ Composition API |
| UI 组件 | Naive UI |
| 状态管理 | Pinia |
| HTTP | Axios |
| 构建 | Vite |
| 测试 | Vitest |

---

## 快速参考

| 命令 | 作用 |
|------|------|
| `cargo check` | 后端编译检查 |
| `cargo test` | 后端单元测试 |
| `npm run build` | 前端构建 |
| `npm test` | 前端测试 |
| `gh pr create` | 创建 PR |
| `git worktree add` | 创建隔离工作目录 |
