---
status: ready-for-dev
epic: 1
story: 10
story_key: 1-10-project-creation
last_updated: 2026-04-09
---

# Story 1.10: 创建小说项目并选择风格

## Story Header

| 属性 | 值 |
|------|-----|
| **Epic** | Epic 1: 项目初始化与风格管理 |
| **Story ID** | 1.10 |
| **Story Key** | 1-10-project-creation |
| **优先级** | P0 |
| **估算复杂度** | 中 |
| **状态** | ready-for-dev |
| **前置依赖** | Story 1.9 (保存和管理风格档案) |

---

## User Story Statement

**As a** 作家，  
**I want** 创建新的小说项目并选择风格档案，  
**So that** 我可以开始创作。

---

## Acceptance Criteria (BDD Format)

### AC1: 创建新项目入口

**Given** 用户已登录  
**When** 用户进入项目列表页面  
**Then** 显示"创建新项目"按钮  
**And** 用户点击后进入项目创建流程

### AC2: 输入项目基本信息

**Given** 用户进入项目创建页面  
**When** 用户填写项目信息  
**Then** 输入项目名称（必填，2-100 字符）  
**And** 输入项目描述（可选，最多 500 字符）  
**And** 系统验证项目名称不与已有项目重复

### AC3: 选择风格档案

**Given** 用户正在创建项目  
**When** 用户选择关联的风格档案  
**Then** 从用户已有的风格档案列表中选择  
**And** 显示所选风格的简要信息（名称、来源）  
**And** 支持预览风格详情（雷达图、特征）

### AC4: 不使用风格档案（可选）

**Given** 用户没有风格档案或想稍后选择  
**When** 用户选择"暂不选择风格"  
**Then** 项目可以无关联风格创建  
**And** 用户可在创建后补选风格

### AC5: 项目创建成功

**Given** 用户填写完所有信息  
**When** 用户点击"创建项目"  
**Then** 系统创建项目并关联风格（如有）  
**And** 跳转到项目详情页  
**And** 显示"创建成功"提示

### AC6: 项目列表展示

**Given** 用户有已创建的项目  
**When** 用户进入项目列表页面  
**Then** 显示所有项目卡片（名称、描述、风格、创建时间）  
**And** 支持按时间/名称排序  
**And** 支持搜索项目名称

### AC7: 进入项目创作

**Given** 用户正在查看项目列表  
**When** 用户点击某个项目  
**Then** 进入项目详情/创作页面  
**And** 显示项目基本信息和风格信息

### AC8: 空状态引导

**Given** 用户没有项目  
**When** 用户进入项目列表页面  
**Then** 显示空状态和引导文案  
**And** 提供"创建第一个项目"按钮

---

## Story Requirements

### 功能需求

| ID | 需求 | 优先级 |
|----|------|--------|
| FR1.10.1 | 用户可以创建新项目 | P0 |
| FR1.10.2 | 用户可以为新项目选择风格档案 | P0 |
| FR1.10.3 | 用户可以不选择风格创建项目 | P1 |
| FR1.10.4 | 用户可以查看项目列表 | P0 |
| FR1.10.5 | 用户可以搜索/筛选项目 | P1 |
| FR1.10.6 | 用户可以进入项目详情 | P0 |
| FR1.10.7 | 系统验证项目名称唯一性 | P0 |

### 非功能需求

| ID | 需求 | 优先级 |
|----|------|--------|
| NFR1.10.1 | 项目列表加载时间 < 1 秒 | P0 |
| NFR1.10.2 | 支持至少 50 个项目 | P0 |
| NFR1.10.3 | 项目数据本地存储 | P0 |

---

## Developer Context Section

### Story Scope Guardrails

**本 Story 必须完成：**
- 项目创建表单（名称、描述、风格选择）
- 项目列表展示
- 项目详情/创作入口页面
- 空状态和引导
- 项目 - 风格关联

**本 Story 明确不做：**
- 章节管理功能（Story 2.3）
- 编辑器功能（Story 2.2）
- 风格混合功能（Story 1.11）

### 技术栈要求

**前端（Vue 3）：**
- **框架：** Vue 3.4+ Composition API
- **语言：** TypeScript 5.x
- **UI 组件：** Naive UI（深色主题）
- **状态管理：** Pinia
- **HTTP 客户端：** Axios

**后端（Rust）：**
- **框架：** Axum
- **数据库：** SQLite + SQLx
- **存储策略：** SQLite + 本地文件系统混合存储

### 架构合规要求

1. **项目 - 风格关联** - 项目通过 `novel_style_bindings` 表关联风格
2. **本地存储** - 项目数据存储在本地 SQLite，章节内容存储在文件系统
3. **组件复用** - 风格选择组件复用 Story 1.9 的风格卡片
4. **空状态设计** - 符合 UX 设计规范

### 核心 API 设计

```rust
// handlers/projects.rs

/// 创建新项目
#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub title: String,
    pub description: Option<String>,
    pub style_profile_id: Option<String>, // 可选的风格档案 ID
}

#[post("/api/projects")]
pub async fn create_project(
    Json(req): Json<CreateProjectRequest>,
    db: Extension<SqlitePool>,
) -> Result<Json<CreateProjectResponse>> {
    // 1. 验证项目名称不重复
    // 2. 创建项目记录
    // 3. 如有风格，创建 novel_style_bindings 关联
    // 4. 创建项目目录（用于存储章节文件）
}

/// 获取项目列表
#[get("/api/projects")]
pub async fn list_projects(
    db: Extension<SqlitePool>,
    Query(params): Query<ListParams>,
) -> Result<Json<Vec<ProjectSummary>>> {
    // 支持分页、排序、搜索
}

/// 获取项目详情
#[get("/api/projects/{id}")]
pub async fn get_project(
    path: Path<String>,
    db: Extension<SqlitePool>,
) -> Result<Json<ProjectDetail>> {
    // 返回项目信息和关联的风格信息
}

/// 更新项目
#[put("/api/projects/{id}")]
pub async fn update_project(
    path: Path<String>,
    Json(req): Json<UpdateProjectRequest>,
    db: Extension<SqlitePool>,
) -> Result<Json<UpdateProjectResponse>> {
    // 更新项目信息
}

/// 删除项目
#[delete("/api/projects/{id}")]
pub async fn delete_project(
    path: Path<String>,
    db: Extension<SqlitePool>,
) -> Result<Json<DeleteResponse>> {
    // 1. 验证项目下无章节或章节已备份
    // 2. 删除项目
    // 3. 清理项目目录
}
```

### 数据库 Schema

```sql
-- 小说项目表
CREATE TABLE novels (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 小说 - 风格关联表
CREATE TABLE novel_style_bindings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    novel_id UUID REFERENCES novels(id) ON DELETE CASCADE,
    style_profile_id UUID REFERENCES style_profiles(id),
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 索引
CREATE INDEX idx_novels_title ON novels(title);
CREATE INDEX idx_novel_style_bindings_novel ON novel_style_bindings(novel_id);
```

### 前端组件设计

```vue
<!-- views/ProjectListView.vue -->
<template>
  <div class="project-list-view">
    <div class="list-header">
      <h1>我的项目</h1>
      <n-button type="primary" @click="handleCreate">
        创建新项目
      </n-button>
    </div>
    
    <div class="search-bar">
      <n-input
        v-model:value="searchQuery"
        placeholder="搜索项目名称..."
        clearable
        @input="handleSearch"
      >
        <template #prefix>
          <n-icon><SearchIcon /></n-icon>
        </template>
      </n-input>
    </div>
    
    <div class="project-grid" v-if="projects.length > 0">
      <n-card
        v-for="project in projects"
        :key="project.id"
        class="project-card"
        @click="enterProject(project.id)"
      >
        <template #header>
          <div class="card-header">
            <h3>{{ project.title }}</h3>
            <n-tag v-if="project.style_name" size="small" type="info">
              {{ project.style_name }}
            </n-tag>
          </div>
        </template>
        
        <p class="project-description">{{ project.description || '暂无描述' }}</p>
        
        <div class="card-footer">
          <span class="created-at">{{ formatDate(project.created_at) }}</span>
          <span class="chapter-count">{{ project.chapter_count }} 章</span>
        </div>
      </n-card>
    </div>
    
    <!-- 空状态 -->
    <n-empty
      v-if="projects.length === 0 && !searchQuery"
      description="还没有项目，创建第一个小说项目开始创作吧"
    >
      <template #extra>
        <n-button type="primary" @click="handleCreate">
          创建第一个项目
        </n-button>
      </template>
    </n-empty>
    
    <!-- 搜索无结果 -->
    <n-empty
      v-if="projects.length === 0 && searchQuery"
      description="未找到匹配的项目"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { NCard, NButton, NTag, NInput, NEmpty, NIcon } from 'naive-ui';
import { Search as SearchIcon } from '@vicons/ionicons5';

const router = useRouter();
const projects = ref([]);
const searchQuery = ref('');

onMounted(async () => {
  await loadProjects();
});

async function loadProjects() {
  const response = await axios.get('/api/projects');
  projects.value = response.data.data;
}

function handleCreate() {
  router.push('/projects/create');
}

function enterProject(id: string) {
  router.push(`/projects/${id}`);
}

function handleSearch() {
  // 前端搜索或调用搜索 API
}
</script>

<style scoped lang="css">
.project-list-view {
  padding: 24px;
  max-width: 1400px;
  margin: 0 auto;
}

.list-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
}

.search-bar {
  margin-bottom: 24px;
  max-width: 400px;
}

.project-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  gap: 24px;
}

.project-card {
  cursor: pointer;
  transition: transform 0.2s;
}

.project-card:hover {
  transform: translateY(-4px);
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.project-description {
  color: #858585;
  font-size: 14px;
  line-height: 1.6;
  max-height: 60px;
  overflow: hidden;
  text-overflow: ellipsis;
}

.card-footer {
  display: flex;
  justify-content: space-between;
  color: #858585;
  font-size: 12px;
  margin-top: 16px;
}
</style>
```

```vue
<!-- views/CreateProjectView.vue -->
<template>
  <div class="create-project-view">
    <div class="form-container">
      <h1>创建新项目</h1>
      
      <n-form
        ref="formRef"
        :model="formData"
        :rules="formRules"
        label-placement="top"
      >
        <n-form-item label="项目名称" path="title">
          <n-input
            v-model:value="formData.title"
            placeholder="给小说起个名字..."
            maxlength="100"
            show-count
          />
        </n-form-item>
        
        <n-form-item label="项目描述" path="description">
          <n-input
            v-model:value="formData.description"
            type="textarea"
            placeholder="简短描述你的小说（可选）"
            maxlength="500"
            show-count
            :rows="3"
          />
        </n-form-item>
        
        <n-form-item label="选择风格档案" path="style_profile_id">
          <div class="style-selector">
            <n-select
              v-model:value="formData.style_profile_id"
              :options="styleOptions"
              placeholder="选择或搜索风格档案"
              filterable
              clearable
            />
            
            <div v-if="selectedStyle" class="style-preview">
              <n-card size="small">
                <template #header>
                  {{ selectedStyle.name }}
                </template>
                <p class="style-source">来源：{{ selectedStyle.source }}</p>
                <n-button text @click="viewStyleDetail">查看详情</n-button>
              </n-card>
            </div>
            
            <n-alert
              v-if="!formData.style_profile_id"
              type="info"
              title="暂不选择风格"
            >
              你可以先创建项目，稍后再关联风格档案。
            </n-alert>
          </div>
        </n-form-item>
        
        <n-form-item>
          <n-space>
            <n-button @click="handleCancel">取消</n-button>
            <n-button type="primary" @click="handleSubmit" :loading="isSubmitting">
              创建项目
            </n-button>
          </n-space>
        </n-form-item>
      </n-form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { NForm, NFormItem, NInput, NSelect, NCard, NButton, NAlert, NSpace } from 'naive-ui';
import { useFormRules } from '@/composables/useFormRules';

const router = useRouter();
const { required, maxLength } = useFormRules();

const formData = ref({
  title: '',
  description: '',
  style_profile_id: null as string | null,
});

const styleOptions = ref([]);
const isSubmitting = ref(false);

const selectedStyle = computed(() => {
  if (!formData.value.style_profile_id) return null;
  return styleOptions.value.find(opt => opt.value === formData.value.style_profile_id);
});

const formRules = {
  title: [
    required('请输入项目名称'),
    maxLength(100, '名称不能超过 100 个字符'),
  ],
};

const formRef = ref(null);

onMounted(async () => {
  await loadStyleProfiles();
});

async function loadStyleProfiles() {
  const response = await axios.get('/api/style-profiles');
  styleOptions.value = response.data.data.map((style: any) => ({
    label: style.name,
    value: style.id,
  }));
}

function viewStyleDetail() {
  router.push(`/styles/${formData.value.style_profile_id}`);
}

function handleSubmit() {
  formRef.value?.validate(async (errors: any) => {
    if (errors) return;
    
    try {
      isSubmitting.value = true;
      await axios.post('/api/projects', {
        title: formData.value.title,
        description: formData.value.description || null,
        style_profile_id: formData.value.style_profile_id || null,
      });
      
      router.push('/projects');
    } catch (error) {
      if (error.response?.data?.code === 'DUPLICATE_TITLE') {
        formRef.value?.errors = [{ key: 'title', message: '项目名称已存在' }];
      }
    } finally {
      isSubmitting.value = false;
    }
  });
}

function handleCancel() {
  router.back();
}
</script>

<style scoped lang="css">
.create-project-view {
  padding: 24px;
}

.form-container {
  max-width: 600px;
  margin: 0 auto;
}

.style-selector {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.style-preview {
  margin-top: 8px;
}

.style-source {
  color: #858585;
  font-size: 14px;
  margin-bottom: 8px;
}
</style>
```

---

## File Structure Requirements

### 前端文件结构

```
src/
├── views/
│   ├── ProjectListView.vue          # 项目列表页面（NEW）
│   ├── CreateProjectView.vue        # 创建项目页面（NEW）
│   └── ProjectDetailView.vue        # 项目详情页面（NEW）
├── components/
│   └── project/
│       ├── ProjectCard.vue          # 项目卡片组件（NEW）
│       ├── StyleSelector.vue        # 风格选择器组件（NEW）
│       └── EmptyState.vue           # 空状态组件（复用）
├── stores/
│   └── project.ts                   # 项目状态管理（NEW）
└── services/
    └── project.ts                   # 项目 API 客户端（NEW）
```

### 后端文件结构

```
src/
├── handlers/
│   └── projects.rs                  # 项目 CRUD 处理器（NEW）
├── models/
│   └── novel.rs                     # 小说项目模型（NEW）
├── services/
│   └── project_service.rs           # 项目服务（NEW）
└── db/
    └── migrations/
        └── 009_create_novels_table.sql
```

---

## Testing Requirements

### 后端测试（Rust）

```rust
// tests/projects_test.rs

#[tokio::test]
async fn test_create_project_success() {
    // 测试创建成功
}

#[tokio::test]
async fn test_create_project_duplicate_title() {
    // 测试名称重复
}

#[tokio::test]
async fn test_create_project_with_style() {
    // 测试关联风格创建
}

#[tokio::test]
async fn test_create_project_without_style() {
    // 测试无风格创建
}

#[tokio::test]
async fn test_list_projects() {
    // 测试列表查询
}
```

### 前端测试（Vitest）

```typescript
// tests/CreateProjectView.test.ts

describe('CreateProjectView', () => {
  it('should validate project title', () => {
    // 测试标题验证
  });

  it('should display style selector', async () => {
    // 测试风格选择器
  });

  it('should create project successfully', async () => {
    // 测试创建成功
  });

  it('should handle duplicate title error', async () => {
    // 测试重复名称错误处理
  });
});
```

---

## Story Completion Status

- [ ] 后端：项目 CRUD 接口
- [ ] 后端：项目 - 风格关联逻辑
- [ ] 后端：数据库迁移
- [ ] 前端：项目列表页面
- [ ] 前端：创建项目页面
- [ ] 前端：项目详情页面
- [ ] 前端：风格选择器组件
- [ ] 前端：项目状态管理
- [ ] 测试：后端单元测试
- [ ] 测试：前端组件测试

---

**Story created:** 2026-04-09  
**Last updated:** 2026-04-09  
**Status:** ready-for-dev
