---
status: ready-for-dev
epic: 1
story: 9
story_key: 1-9-style-profile-management
last_updated: 2026-04-09
---

# Story 1.9: 保存和管理风格档案

## Story Header

| 属性 | 值 |
|------|-----|
| **Epic** | Epic 1: 项目初始化与风格管理 |
| **Story ID** | 1.9 |
| **Story Key** | 1-9-style-profile-management |
| **优先级** | P0 |
| **估算复杂度** | 中 |
| **状态** | ready-for-dev |
| **前置依赖** | Story 1.8 (显示风格分析报告) |

---

## User Story Statement

**As a** 作家，  
**I want** 保存和管理我的风格档案，  
**So that** 我可以在创作时使用这些风格。

---

## Acceptance Criteria (BDD Format)

### AC1: 确认保存风格档案

**Given** 用户正在查看风格分析报告  
**When** 用户点击"保存风格档案"  
**Then** 显示确认对话框，要求输入风格名称  
**And** 显示风格来源文件和字数信息  
**And** 用户确认后保存风格到数据库

### AC2: 风格命名和描述

**Given** 用户进入保存流程  
**When** 用户输入风格名称  
**Then** 系统验证名称格式（2-50 字符）  
**And** 验证名称不与已有风格重复  
**And** 用户可填写可选描述

### AC3: 风格档案列表展示

**Given** 用户有已保存的风格档案  
**When** 用户进入风格库页面  
**Then** 显示所有已保存的风格档案列表  
**And** 每个档案显示名称、来源、创建时间  
**And** 支持按名称/时间排序

### AC4: 查看风格档案详情

**Given** 用户在风格列表页面  
**When** 用户点击某个风格档案  
**Then** 显示该风格的详细信息  
**And** 可复用 Story 1.8 的报告组件

### AC5: 使用风格档案

**Given** 用户正在查看风格档案  
**When** 用户点击"使用此风格"  
**Then** 该风格被标记为当前项目风格  
**And** 跳转到项目创建页面（Story 1.10）

### AC6: 删除风格档案

**Given** 用户正在查看风格档案  
**When** 用户点击"删除"  
**Then** 显示确认对话框（删除后不可恢复）  
**And** 用户确认后删除风格档案  
**And** 删除关联的源文件和分析数据

### AC7: 导出风格档案

**Given** 用户正在查看风格档案  
**When** 用户点击"导出"  
**Then** 导出风格档案为 JSON 格式  
**And** 包含七层特征数据和风格向量

### AC8: 导入风格档案

**Given** 用户在风格库页面  
**When** 用户点击"导入风格"  
**Then** 支持选择 JSON 格式的风格文件  
**And** 验证文件格式和数据完整性  
**And** 导入成功后显示在列表中

---

## Story Requirements

### 功能需求

| ID | 需求 | 优先级 |
|----|------|--------|
| FR1.9.1 | 用户可以保存风格档案 | P0 |
| FR1.9.2 | 用户可以查看风格档案列表 | P0 |
| FR1.9.3 | 用户可以查看风格详情 | P0 |
| FR1.9.4 | 用户可以删除风格档案 | P0 |
| FR1.9.5 | 用户可以导出风格档案 | P1 |
| FR1.9.6 | 用户可以导入风格档案 | P1 |
| FR1.9.7 | 用户可以使用风格档案 | P0 |
| FR1.9.8 | 系统验证风格名称唯一性 | P0 |

### 非功能需求

| ID | 需求 | 优先级 |
|----|------|--------|
| NFR1.9.1 | 列表加载时间 < 1 秒 | P0 |
| NFR1.9.2 | 支持至少 100 个风格档案 | P0 |
| NFR1.9.3 | 导出文件格式标准（JSON） | P0 |
| NFR1.9.4 | 导入文件格式验证 | P0 |

---

## Developer Context Section

### Story Scope Guardrails

**本 Story 必须完成：**
- 风格档案保存流程（命名、验证、存储）
- 风格档案列表展示
- 风格档案详情查看（复用 Story 1.8）
- 风格档案删除功能
- 风格档案导出/导入功能
- 使用风格档案入口（Story 1.10 的 CTA）

**本 Story 明确不做：**
- 风格混合功能（Story 1.11）
- 风格相似度计算（Story 1.12）
- 项目创建功能（Story 1.10）

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
- **文件存储：** 本地文件系统

### 架构合规要求

1. **数据完整性** - 删除风格档案时应清理关联数据
2. **导入验证** - 导入 JSON 文件时需验证结构完整性
3. **命名唯一性** - 同一用户的风格名称不能重复
4. **组件复用** - 风格详情应复用 Story 1.8 的报告组件

### 核心 API 设计

```rust
// handlers/style_profiles.rs

/// 保存风格档案
#[derive(Debug, Deserialize)]
pub struct SaveStyleProfileRequest {
    pub task_id: String,
    pub name: String,
    pub description: Option<String>,
}

#[post("/api/style-profiles/save")]
pub async fn save_style_profile(
    Json(req): Json<SaveStyleProfileRequest>,
    db: Extension<SqlitePool>,
) -> Result<Json<SaveStyleProfileResponse>> {
    // 1. 验证分析任务已完成
    // 2. 验证风格名称不重复
    // 3. 从分析结果创建 StyleProfile
    // 4. 保存到数据库
    // 5. 更新分析任务的 result_profile_id
}

/// 获取风格档案列表
#[get("/api/style-profiles")]
pub async fn list_style_profiles(
    db: Extension<SqlitePool>,
    Query(params): Query<ListParams>,
) -> Result<Json<Vec<StyleProfileSummary>>> {
    // 支持分页和排序
}

/// 获取风格档案详情
#[get("/api/style-profiles/{id}")]
pub async fn get_style_profile(
    path: Path<String>,
    db: Extension<SqlitePool>,
) -> Result<Json<StyleProfileDetail>> {
    // 返回完整风格档案（含七层特征和向量）
}

/// 删除风格档案
#[delete("/api/style-profiles/{id}")]
pub async fn delete_style_profile(
    path: Path<String>,
    db: Extension<SqlitePool>,
) -> Result<Json<DeleteResponse>> {
    // 1. 验证风格未被项目使用
    // 2. 删除风格档案
    // 3. 清理关联的源文件
}

/// 导出风格档案
#[get("/api/style-profiles/{id}/export")]
pub async fn export_style_profile(
    path: Path<String>,
    db: Extension<SqlitePool>,
) -> Result<Json<ExportableStyleProfile>> {
    // 导出为标准 JSON 格式
}

/// 导入风格档案
#[post("/api/style-profiles/import")]
pub async fn import_style_profile(
    Json(req): Json<ImportRequest>,
    db: Extension<SqlitePool>,
) -> Result<Json<ImportResponse>> {
    // 1. 验证 JSON 结构
    // 2. 验证名称不重复
    // 3. 导入到数据库
}

/// 使用风格档案
#[post("/api/style-profiles/{id}/use")]
pub async fn use_style_profile(
    path: Path<String>,
    db: Extension<SqlitePool>,
) -> Result<Json<UseStyleResponse>> {
    // 将风格标记为当前使用
}
```

### 数据库 Schema

```sql
-- style_profiles 表已存在，扩展字段
CREATE TABLE style_profiles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    description TEXT,
    source_file_path TEXT,
    source_novels TEXT[],
    total_chars INTEGER,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
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

-- 索引
CREATE INDEX idx_style_profiles_name ON style_profiles(name);
CREATE INDEX idx_style_profiles_created_at ON style_profiles(created_at);

-- 风格使用记录（可选）
CREATE TABLE style_usage_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    style_profile_id UUID REFERENCES style_profiles(id),
    used_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    context TEXT  -- 使用场景（如：project_id）
);
```

### 前端组件设计

```vue
<!-- views/StyleLibraryView.vue -->
<template>
  <div class="style-library-view">
    <div class="library-header">
      <h1>风格档案库</h1>
      <div class="library-actions">
        <n-button @click="handleImport">导入风格</n-button>
        <n-button type="primary" @click="handleCreate">
          创建新风格
        </n-button>
      </div>
    </div>
    
    <div class="library-content">
      <n-data-table
        :columns="columns"
        :data="styleProfiles"
        :pagination="pagination"
        :sort-by="sortBy"
        :sort-order="sortOrder"
        @update:sort-by="handleSort"
      />
    </div>
    
    <!-- 空状态 -->
    <n-empty 
      v-if="styleProfiles.length === 0"
      description="还没有风格档案，上传参考小说来创建第一个风格吧"
    >
      <template #extra>
        <n-button type="primary" @click="handleCreate">
          创建风格档案
        </n-button>
      </template>
    </n-empty>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, h } from 'vue';
import { NButton, NTag, NEmpty, NDataTable } from 'naive-ui';
import { useRouter } from 'vue-router';
import type { StyleProfileSummary } from '@/types/style';

const router = useRouter();
const styleProfiles = ref<StyleProfileSummary[]>([]);

const columns = [
  {
    title: '名称',
    key: 'name',
    sorter: 'default',
    render: (row: StyleProfileSummary) => 
      h('a', { onClick: () => viewDetail(row.id) }, row.name)
  },
  {
    title: '来源文件',
    key: 'source_file',
  },
  {
    title: '字数',
    key: 'total_chars',
    render: (row: StyleProfileSummary) => 
      `${(row.total_chars / 1000).toFixed(0)}千字`
  },
  {
    title: '创建时间',
    key: 'created_at',
    sorter: 'default',
    render: (row: StyleProfileSummary) => formatDate(row.created_at)
  },
  {
    title: '操作',
    key: 'actions',
    render: (row: StyleProfileSummary) => 
      h('div', { style: 'display: flex; gap: 8px;' }, [
        h(NButton, { 
          size: 'small', 
          onClick: () => useStyle(row.id) 
        }, { default: () => '使用' }),
        h(NButton, { 
          size: 'small',
          onClick: () => exportStyle(row.id)
        }, { default: () => '导出' }),
        h(NButton, { 
          size: 'small',
          type: 'error',
          onClick: () => deleteStyle(row.id)
        }, { default: () => '删除' }),
      ])
  },
];

onMounted(async () => {
  await loadStyleProfiles();
});

async function loadStyleProfiles() {
  const response = await axios.get('/api/style-profiles');
  styleProfiles.value = response.data.data;
}

function viewDetail(id: string) {
  router.push(`/styles/${id}`);
}

function useStyle(id: string) {
  // 标记为当前使用风格
  axios.post(`/api/style-profiles/${id}/use`);
  // 跳转到项目创建
  router.push(`/projects/create?styleId=${id}`);
}

function exportStyle(id: string) {
  window.open(`/api/style-profiles/${id}/export`, '_blank');
}

async function deleteStyle(id: string) {
  await axios.delete(`/api/style-profiles/${id}`);
  await loadStyleProfiles();
}

function handleImport() {
  router.push('/styles/import');
}

function handleCreate() {
  router.push('/styles/upload');
}
</script>
```

```vue
<!-- views/SaveStyleProfileView.vue -->
<template>
  <div class="save-style-profile-view">
    <n-modal
      v-model:show="showModal"
      :close-on-esc="false"
      preset="dialog"
      title="保存风格档案"
    >
      <n-form
        ref="formRef"
        :model="formData"
        :rules="formRules"
        label-placement="top"
      >
        <n-form-item label="风格名称" path="name">
          <n-input
            v-model:value="formData.name"
            placeholder="如：金庸武侠风格"
          />
        </n-form-item>
        
        <n-form-item label="描述（可选）" path="description">
          <n-input
            v-model:value="formData.description"
            type="textarea"
            placeholder="简短描述这个风格的特点"
          />
        </n-form-item>
        
        <n-descriptions bordered :column="1">
          <n-descriptions-item label="来源文件">
            {{ styleData.source_file }}
          </n-descriptions-item>
          <n-descriptions-item label="总字数">
            {{ styleData.total_chars }}
          </n-descriptions-item>
          <n-descriptions-item label="分析完成时间">
            {{ formatDate(styleData.completed_at) }}
          </n-descriptions-item>
        </n-descriptions>
      </n-form>
      
      <template #action>
        <n-button @click="showModal = false">取消</n-button>
        <n-button type="primary" @click="handleSubmit" :loading="isSubmitting">
          确认保存
        </n-button>
      </template>
    </n-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { NModal, NForm, NFormItem, NInput, NButton, NDescriptions } from 'naive-ui';
import { useFormRules } from '@/composables/useFormRules';

const route = useRoute();
const router = useRouter();
const { required, maxLength } = useFormRules();

const showModal = ref(true);
const formData = ref({
  name: '',
  description: '',
});
const isSubmitting = ref(false);

// 从 Story 1.8 报告页面传来的数据
const styleData = computed(() => route.query.styleData || {});

const formRules = {
  name: [
    required('请输入风格名称'),
    maxLength(50, '名称不能超过 50 个字符'),
  ],
};

const formRef = ref(null);

async function handleSubmit() {
  try {
    isSubmitting.value = true;
    await formRef.value?.validate();
    
    await axios.post('/api/style-profiles/save', {
      task_id: route.params.taskId,
      name: formData.value.name,
      description: formData.value.description || null,
    });
    
    // 保存成功，跳转到风格库或项目创建
    router.push('/styles');
  } catch (error) {
    if (error.response?.data?.code === 'DUPLICATE_NAME') {
      formRef.value?.errors = [{ key: 'name', message: '该名称已存在，请使用其他名称' }];
    }
  } finally {
    isSubmitting.value = false;
  }
}
</script>
```

---

## File Structure Requirements

### 前端文件结构

```
src/
├── views/
│   ├── StyleLibraryView.vue         # 风格档案库列表（NEW）
│   ├── SaveStyleProfileView.vue     # 保存风格档案（NEW）
│   ├── StyleProfileDetailView.vue   # 风格档案详情（复用 Story 1.8）
│   └── ImportStyleView.vue          # 导入风格档案（NEW）
├── components/
│   └── style/
│       ├── StyleRadarChart.vue      # 复用 Story 1.8
│       ├── FeatureDetailPanel.vue   # 复用 Story 1.8
│       └── StyleProfileCard.vue     # 风格档案卡片（NEW）
├── stores/
│   └── style.ts                     # 风格状态管理（扩展）
└── services/
    └── style.ts                     # 风格 API 客户端（扩展）
```

### 后端文件结构

```
src/
├── handlers/
│   └── style_profiles.rs            # 风格档案 CRUD 处理器（NEW）
├── models/
│   └── style_profile.rs             # 风格档案模型（NEW）
├── services/
│   └── style_profile_service.rs     # 风格档案服务（NEW）
└── db/
    └── migrations/
        └── 008_create_style_profiles_table.sql
```

---

## Testing Requirements

### 后端测试（Rust）

```rust
// tests/style_profiles_test.rs

#[tokio::test]
async fn test_save_style_profile_success() {
    // 测试保存成功场景
}

#[tokio::test]
async fn test_save_style_profile_duplicate_name() {
    // 测试名称重复
}

#[tokio::test]
async fn test_delete_style_profile_success() {
    // 测试删除成功
}

#[tokio::test]
async fn test_delete_style_profile_in_use() {
    // 测试被项目使用的风格不能被删除
}

#[tokio::test]
async fn test_export_style_profile() {
    // 测试导出功能
}

#[tokio::test]
async fn test_import_style_profile_valid() {
    // 测试导入有效 JSON
}

#[tokio::test]
async fn test_import_style_profile_invalid() {
    // 测试导入无效 JSON 被拒绝
}
```

### 前端测试（Vitest）

```typescript
// tests/StyleLibraryView.test.ts

describe('StyleLibraryView', () => {
  it('should display empty state when no styles', () => {
    // 测试空状态
  });

  it('should display style list when styles exist', async () => {
    // 测试列表显示
  });

  it('should delete style after confirmation', async () => {
    // 测试删除功能
  });

  it('should navigate to save flow', async () => {
    // 测试保存入口
  });
});
```

---

## Story Completion Status

- [ ] 后端：风格档案 CRUD 接口
- [ ] 后端：风格档案验证逻辑
- [ ] 后端：导入导出接口
- [ ] 后端：数据库迁移
- [ ] 前端：风格档案库列表页面
- [ ] 前端：保存风格档案页面
- [ ] 前端：导入风格档案页面
- [ ] 前端：风格档案卡片组件
- [ ] 测试：后端单元测试
- [ ] 测试：前端组件测试

---

**Story created:** 2026-04-09  
**Last updated:** 2026-04-09  
**Status:** ready-for-dev
