---
status: review
epic: 1
story: 1
story_key: 1-1-user-registration
last_updated: 2026-04-09
---

# Story 1.1: 用户注册/登录系统

## Story Header

| 属性 | 值 |
|------|-----|
| **Epic** | Epic 1: 项目初始化与风格管理 |
| **Story ID** | 1.1 |
| **Story Key** | 1-1-user-registration |
| **优先级** | P0 |
| **估算复杂度** | 中 |
| **状态** | ready-for-dev |

---

## User Story Statement

**As a** 新用户，  
**I want** 注册和登录系统，  
**So that** 我可以管理个人项目和风格档案。

---

## Acceptance Criteria (BDD Format)

### AC1: 用户注册流程

**Given** 用户首次访问应用  
**When** 用户点击"注册"并填写用户名和密码  
**Then** 系统创建用户账户并自动登录  
**And** 用户数据本地加密存储  
**And** 跳转到风格档案创建引导页面

### AC2: 用户名验证

**Given** 用户在注册表单中输入用户名  
**When** 用户失去焦点或点击提交  
**Then** 系统验证用户名格式（3-20 字符，仅允许字母数字下划线）  
**And** 显示验证结果（成功/错误原因）

### AC3: 密码验证

**Given** 用户在注册表单中输入密码  
**When** 用户失去焦点或点击提交  
**Then** 系统验证密码强度（最少 8 字符，包含字母和数字）  
**And** 显示密码强度指示器

### AC4: 用户登录流程

**Given** 已注册用户  
**When** 用户输入正确的用户名和密码  
**Then** 系统验证通过并登录  
**And** 加载用户的项目和风格档案列表  
**And** 跳转到项目列表页面

### AC5: 登录失败处理

**Given** 用户输入错误的凭据  
**When** 用户点击登录  
**Then** 系统显示错误提示（"用户名或密码错误"）  
**And** 不清除已输入的用户名  
**And** 提供"忘记密码"选项（预留）

### AC6: 自动登录（Session 保持）

**Given** 用户已登录并关闭应用  
**When** 用户再次打开应用  
**Then** 系统自动恢复登录状态  
**And** 直接跳转到项目列表页面

### AC7: 登出功能

**Given** 已登录用户  
**When** 用户点击"登出"  
**Then** 系统清除本地 Session  
**And** 跳转到登录页面  
**And** 显示"已成功登出"提示

### AC8: 数据本地加密存储

**Given** 用户注册或登录  
**When** 系统存储用户数据  
**Then** 使用加密算法加密敏感数据（密码 hash）  
**And** 存储到本地 SQLite 数据库

---

## Story Requirements

### 功能需求

| ID | 需求 | 优先级 |
|----|------|--------|
| FR1.1.1 | 用户可以注册新账户 | P0 |
| FR1.1.2 | 用户可以登录已有账户 | P0 |
| FR1.1.3 | 用户可以登出当前账户 | P0 |
| FR1.1.4 | 系统验证用户名格式 | P0 |
| FR1.1.5 | 系统验证密码强度 | P0 |
| FR1.1.6 | 系统加密存储用户凭据 | P0 |
| FR1.1.7 | 系统保持登录状态（Session） | P1 |

### 非功能需求

| ID | 需求 | 优先级 |
|----|------|--------|
| NFR1.1.1 | 登录响应时间 < 1 秒 | P0 |
| NFR1.1.2 | 密码使用 bcrypt 加密存储 | P0 |
| NFR1.1.3 | Session 有效期 7 天 | P1 |
| NFR1.1.4 | 支持断网环境下本地登录 | P1 |

---

## Developer Context Section

### 技术栈要求

**后端（Rust）：**
- **框架：** Axum
- **数据库：** SQLite + SQLx
- **密码加密：** bcrypt
- **Session 管理：** 本地存储 + 时间戳验证

**前端（Vue 3）：**
- **框架：** Vue 3.4+ Composition API
- **UI 组件：** Naive UI（深色主题）
- **状态管理：** Pinia
- **HTTP 客户端：** Axios

### 架构合规要求

根据架构设计文档，本 Story 需要遵循：

1. **本地优先存储** - 用户数据存储在本地 SQLite 数据库，不依赖云端
2. **加密存储** - 密码使用 bcrypt hash 存储
3. **前后端分离** - 前端通过 REST API 与后端交互
4. **深色主题** - 使用统一的设计令牌（背景#1E1E1E、文字#D4D4D4）

### 数据库 Schema

```sql
-- 用户表
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_login_at TIMESTAMP,
    session_token TEXT,
    session_expires_at TIMESTAMP
);

-- 索引
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_session ON users(session_token);
```

### API 端点

```
# 用户认证
POST   /api/auth/register          # 用户注册
POST   /api/auth/login             # 用户登录
POST   /api/auth/logout            # 用户登出
GET    /api/auth/me                # 获取当前用户信息
```

### 请求/响应格式

**注册请求：**
```json
POST /api/auth/register
{
  "username": "writer123",
  "password": "SecurePass123"
}
```

**注册响应：**
```json
{
  "success": true,
  "data": {
    "user": {
      "id": "uuid",
      "username": "writer123",
      "created_at": "2026-04-09T10:00:00Z"
    },
    "session": {
      "token": "session_token",
      "expires_at": "2026-04-16T10:00:00Z"
    }
  }
}
```

**登录请求：**
```json
POST /api/auth/login
{
  "username": "writer123",
  "password": "SecurePass123"
}
```

**登录响应：**
```json
{
  "success": true,
  "data": {
    "user": {
      "id": "uuid",
      "username": "writer123",
      "last_login_at": "2026-04-09T10:00:00Z"
    },
    "session": {
      "token": "session_token",
      "expires_at": "2026-04-16T10:00:00Z"
    }
  }
}
```

---

## File Structure Requirements

### 后端文件结构

```
src/
├── handlers/
│   └── auth.rs              # 认证处理器（register, login, logout）
├── models/
│   └── user.rs              # 用户模型（User, Session）
├── services/
│   └── auth_service.rs      # 认证服务（密码验证、Session 管理）
├── db/
│   ├── mod.rs               # 数据库连接
│   └── migrations/
│       └── 001_create_users_table.sql
└── main.rs
```

### 前端文件结构

```
src/
├── views/
│   ├── LoginView.vue        # 登录页面
│   └── RegisterView.vue     # 注册页面
├── components/
│   └── auth/
│       ├── LoginForm.vue    # 登录表单组件
│       ├── RegisterForm.vue # 注册表单组件
│       └── UserMenu.vue     # 用户菜单（登出）
├── stores/
│   └── auth.ts              # 认证状态管理（Pinia）
├── services/
│   └── api.ts               # API 客户端（Axios 配置）
└── router/
    └── index.ts             # 路由配置（认证守卫）
```

---

## Testing Requirements

### 后端测试（Rust）

```rust
// tests/auth_test.rs

#[tokio::test]
async fn test_user_registration_success() {
    // 测试注册成功场景
}

#[tokio::test]
async fn test_user_registration_duplicate_username() {
    // 测试用户名重复
}

#[tokio::test]
async fn test_user_login_success() {
    // 测试登录成功
}

#[tokio::test]
async fn test_user_login_invalid_credentials() {
    // 测试登录失败
}

#[tokio::test]
async fn test_password_hashing() {
    // 测试密码加密
}
```

### 前端测试（Vitest）

```typescript
// tests/auth.test.ts

describe('AuthForm', () => {
  it('should validate username format', () => {
    // 测试用户名验证
  });

  it('should validate password strength', () => {
    // 测试密码强度验证
  });

  it('should handle login success', async () => {
    // 测试登录成功
  });

  it('should handle login failure', async () => {
    // 测试登录失败
  });
});
```

---

## UX Design Requirements

### 设计令牌

```css
:root {
  /* 背景层 */
  --color-bg-primary: #1E1E1E;
  --color-bg-secondary: #252526;
  --color-bg-tertiary: #2D2D30;
  
  /* 内容层 */
  --color-text-primary: #D4D4D4;
  --color-text-secondary: #858585;
  
  /* 功能色 */
  --color-accent: #4EC9B0;
  --color-success: #6A9955;
  --color-warning: #D19A66;
  --color-error: #F48771;
  
  /* 间距 */
  --spacing-m: 16px;
  
  /* 圆角 */
  --radius-sm: 4px;
  --radius-md: 8px;
}
```

### 页面布局

**登录页面：**
```
┌─────────────────────────────────────────┐
│                                          │
│           Novel Agent                    │
│                                          │
│    ┌─────────────────────────┐          │
│    │  用户名                  │          │
│    │  [____________]          │          │
│    │  密码                    │          │
│    │  [____________]          │          │
│    │  [      登录      ]      │          │
│    │  还没有账户？注册        │          │
│    └─────────────────────────┘          │
│                                          │
└─────────────────────────────────────────┘
```

### 交互模式

| 交互 | 设计方式 |
|------|---------|
| 用户名验证 | 失去焦点时即时验证 |
| 密码强度 | 输入时显示强度条 |
| 登录成功 | Toast 通知（顶部居中，3 秒） |
| 登录失败 | 表单上方显示错误消息 |
| 表单错误 | 输入框红色边框 + 错误文字 |

---

## Implementation Notes

### 关键实现细节

1. **密码加密：**
   - 使用 bcrypt 算法，cost factor = 12
   - 永远不要存储明文密码

2. **Session 管理：**
   - 使用 UUID 生成 session token
   - session 有效期 7 天
   - 每次登录更新 session_expires_at

3. **错误处理：**
   - 后端返回统一错误格式
   - 前端根据错误类型显示友好提示
   - 不泄露敏感信息（如"用户名已存在"而非"该用户已注册"）

4. **安全性：**
   - 登录接口限流（防止暴力破解）
   - HTTPS 传输（部署时）
   - SQL 注入防护（使用 SQLx 参数化查询）

---

## Story Completion Status

- [x] 后端：数据库 Schema 实现
- [x] 后端：用户模型定义
- [x] 后端：认证服务实现
- [x] 后端：API 端点实现
- [x] 前端：登录页面组件
- [x] 前端：注册页面组件
- [x] 前端：认证状态管理（Pinia）
- [x] 前端：路由认证守卫
- [x] 测试：后端单元测试
- [x] 测试：前端组件测试
- [x] 集成：端到端测试

## Dev Agent Record

### Implementation Plan

- 基于文档约束建立前后端分离骨架：`backend/` 负责 Axum + SQLite 认证 API，`frontend/` 负责 Vue 3 + Pinia + Naive UI 认证体验。
- 先用测试锁定注册、登录、密码加密和前端认证流，再补齐注册自动登录、Session 恢复、登出与路由守卫。
- 保持 Story 1.1 边界，只交付认证与落地页占位，不提前实现 Story 1.2 的上传分析能力。

### Debug Log

- 2026-04-09：安装本机缺失的 Rust 工具链后完成后端依赖拉取。
- 2026-04-09：完成 `cargo test`、`npm test`、`npm run build` 验证，修正了 Naive UI 测试环境与 Vitest 类型声明问题。

### Completion Notes

- 已实现本地 SQLite 用户表迁移、bcrypt 密码散列、7 天 Session、`/api/auth/register|login|logout|me` 四个认证接口。
- 已实现登录页、注册页、项目列表占位页、风格档案引导页，以及基于 Pinia 的自动登录恢复和登出流程。
- 已补充 5 个 Rust 认证测试、3 个前端校验/组件测试、2 个前端应用级认证流测试，并通过前端生产构建。

## File List

- `backend/.gitignore`
- `backend/Cargo.lock`
- `backend/Cargo.toml`
- `backend/migrations/001_create_users_table.sql`
- `backend/src/app_state.rs`
- `backend/src/db.rs`
- `backend/src/errors.rs`
- `backend/src/handlers/auth.rs`
- `backend/src/handlers/mod.rs`
- `backend/src/lib.rs`
- `backend/src/main.rs`
- `backend/src/models/mod.rs`
- `backend/src/models/user.rs`
- `backend/src/services/auth_service.rs`
- `backend/src/services/mod.rs`
- `backend/src/validation.rs`
- `backend/tests/auth_test.rs`
- `frontend/index.html`
- `frontend/package-lock.json`
- `frontend/package.json`
- `frontend/src/App.vue`
- `frontend/src/components/auth/LoginForm.vue`
- `frontend/src/components/auth/RegisterForm.vue`
- `frontend/src/components/auth/UserMenu.vue`
- `frontend/src/main.ts`
- `frontend/src/router/index.ts`
- `frontend/src/services/api.ts`
- `frontend/src/stores/auth.ts`
- `frontend/src/styles.css`
- `frontend/src/types/auth.ts`
- `frontend/src/utils/http.ts`
- `frontend/src/utils/validation.ts`
- `frontend/src/views/LoginView.vue`
- `frontend/src/views/ProjectsView.vue`
- `frontend/src/views/RegisterView.vue`
- `frontend/src/views/StyleOnboardingView.vue`
- `frontend/src/vite-env.d.ts`
- `frontend/tests/auth-flow.e2e.test.ts`
- `frontend/tests/auth.test.ts`
- `frontend/tests/setup.ts`
- `frontend/tsconfig.json`
- `frontend/tsconfig.node.json`
- `frontend/vite.config.ts`
- `_bmad-output/implementation-artifacts/1-1-user-registration.md`
- `_bmad-output/implementation-artifacts/sprint-status.yaml`

## Change Log

- 2026-04-10：完成 Code Review 安全漏洞修复并标记为 done。
  - **修复问题：**
    - 添加 JWT Token 签名验证（替代纯 UUID）
    - 登录/注册接口添加速率限制（5 次/10 秒）
    - 注册/登录使用数据库事务确保原子性
    - 密码长度限制 72 字节（bcrypt 限制）
    - 时序攻击防护（常量时间验证）
    - 错误日志脱敏（不返回内部详情）
    - LocalStorage 加密存储（AES）
    - 表单输入 trim 和 maxlength 限制
    - 密码确认字段
    - 网络错误分类处理（401/403 vs 其他）
  - **新增依赖：**
    - Backend: `jsonwebtoken`, `governor`, `chrono`, `nonzero_ext`
    - Frontend: `crypto-js`

- 2026-04-09：完成 Story 1.1 的前后端认证实现、测试补齐与构建验证，状态推进到 `review`。

---

**Story created:** 2026-04-09  
**Last updated:** 2026-04-10  
**Status:** done
