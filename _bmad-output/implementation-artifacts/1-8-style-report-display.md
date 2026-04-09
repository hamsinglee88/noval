---
status: ready-for-dev
epic: 1
story: 8
story_key: 1-8-style-report-display
last_updated: 2026-04-09
---

# Story 1.8: 显示风格分析报告

## Story Header

| 属性 | 值 |
|------|-----|
| **Epic** | Epic 1: 项目初始化与风格管理 |
| **Story ID** | 1.8 |
| **Story Key** | 1-8-style-report-display |
| **优先级** | P0 |
| **估算复杂度** | 中 |
| **状态** | ready-for-dev |
| **前置依赖** | Story 1.7 (生成风格向量化表示) |

---

## User Story Statement

**As a** 作家，  
**I want** 查看我的风格分析报告，  
**So that** 我可以确认系统准确捕捉了我的风格。

---

## Acceptance Criteria (BDD Format)

### AC1: 风格报告页面加载

**Given** 风格分析完成（100%）  
**When** 用户进入风格报告页面  
**Then** 显示风格名称和来源文件信息  
**And** 显示分析完成时间和总字数  
**And** 页面加载时间 < 2 秒

### AC2: 风格雷达图展示

**Given** 报告页面加载完成  
**When** 雷达图组件渲染  
**Then** 显示七层风格的雷达图可视化  
**And** 每层特征有清晰的标签和数值  
**And** 雷达图支持 hover 显示详细数值

### AC3: 七层特征详情展示

**Given** 雷达图显示完成  
**When** 用户查看特征详情  
**Then** 逐层展示七层风格特征的详细数据  
**And** 每层特征有关键指标和解读说明  
**And** 数据格式易于理解（如百分比、频率）

### AC4: 示例段落展示

**Given** 特征详情显示完成  
**When** 用户查看示例段落  
**Then** 显示 1-3 段体现目标风格的原文段落  
**And** 段落旁边标注风格特征说明  
**And** 支持点击段落查看高亮标注

### AC5: 风格报告导出

**Given** 用户正在查看报告  
**When** 用户点击"导出报告"  
**Then** 生成 PDF 或图片格式的风格报告  
**And** 下载文件包含雷达图和关键特征

### AC6: 保存风格档案入口

**Given** 用户查看报告完成  
**When** 用户认为风格准确  
**Then** 显示"保存风格档案"按钮  
**And** 点击后跳转到 Story 1.9 的保存流程

### AC7: 重新分析选项

**Given** 用户认为风格不准确  
**When** 用户点击"重新分析"  
**Then** 提供调整参数的选项（如权重）  
**And** 支持重新上传参考小说

### AC8: 空状态和错误处理

**Given** 分析尚未完成或失败  
**When** 用户访问报告页面  
**Then** 显示相应的等待或错误状态  
**And** 提供重试或返回的选项

---

## Story Requirements

### 功能需求

| ID | 需求 | 优先级 |
|----|------|--------|
| FR1.8.1 | 系统显示风格报告页面 | P0 |
| FR1.8.2 | 系统渲染风格雷达图 | P0 |
| FR1.8.3 | 系统展示七层特征详情 | P0 |
| FR1.8.4 | 系统展示示例段落及标注 | P0 |
| FR1.8.5 | 系统支持报告导出 | P1 |
| FR1.8.6 | 系统提供保存档案入口 | P0 |
| FR1.8.7 | 系统支持重新分析 | P1 |
| FR1.8.8 | 系统处理空状态和错误 | P0 |

### 非功能需求

| ID | 需求 | 优先级 |
|----|------|--------|
| NFR1.8.1 | 页面加载时间 < 2 秒 | P0 |
| NFR1.8.2 | 雷达图渲染流畅（60fps） | P0 |
| NFR1.8.3 | 支持深色主题 | P0 |
| NFR1.8.4 | 图表无障碍访问（ARIA） | P1 |

---

## Developer Context Section

### Story Scope Guardrails

**本 Story 必须完成：**
- 风格报告页面 UI 实现
- 风格雷达图可视化（Chart.js 或 ECharts）
- 七层特征详情展示组件
- 示例段落展示及标注
- 保存档案入口（ Story 1.9 的 CTA）

**本 Story 明确不做：**
- 风格档案保存逻辑（Story 1.9）
- 风格档案管理/列表/删除（Story 1.9）
- 风格混合功能（Story 1.11）

### 技术栈要求

**前端（Vue 3）：**
- **框架：** Vue 3.4+ Composition API
- **语言：** TypeScript 5.x
- **UI 组件：** Naive UI（深色主题）
- **图表库：** Chart.js 或 ECharts（雷达图）
- **状态管理：** Pinia
- **HTTP 客户端：** Axios

**后端（Rust）：**
- **框架：** Axum
- **API：** 提供风格分析结果查询接口

### 架构合规要求

1. **组件化设计** - 雷达图、特征详情、示例段落应该是独立组件
2. **深色主题** - 使用统一的设计令牌
3. **响应式布局** - 支持不同屏幕尺寸
4. **无障碍访问** - 图表提供文字描述

### 组件设计

```vue
<!-- components/style/StyleRadarChart.vue -->
<template>
  <div class="style-radar-chart">
    <Radar 
      :data="radarData" 
      :options="radarOptions"
      role="img"
      aria-label="风格雷达图：展示七层风格特征"
    />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { Radar } from 'vue-chartjs';
import { Chart as ChartJS, RadialLinearScale, PointElement, LineElement, Filler, Tooltip, Legend } from 'chart.js';

ChartJS.register(RadialLinearScale, PointElement, LineElement, Filler, Tooltip, Legend);

const props = defineProps<{
  styleData: StyleAnalysisResult;
}>();

const radarData = computed(() => ({
  labels: ['词汇层', '句式层', '修辞层', '叙事层', '情感层', '节奏层', '对话层', '描写层'],
  datasets: [{
    label: '风格特征强度',
    data: [
      calculateLayerScore(props.styleData.vocabulary),
      calculateLayerScore(props.styleData.sentence),
      calculateLayerScore(props.styleData.rhetoric),
      calculateLayerScore(props.styleData.narrative),
      calculateLayerScore(props.styleData.emotion),
      calculateLayerScore(props.styleData.pacing),
      calculateLayerScore(props.styleData.dialogue),
      calculateLayerScore(props.styleData.description),
    ],
    backgroundColor: 'rgba(78, 201, 176, 0.2)',
    borderColor: '#4EC9B0',
    pointBackgroundColor: '#4EC9B0',
    pointRadius: 4,
  }],
}));

const radarOptions = {
  scales: {
    r: {
      angleLines: { color: '#3C3C3C' },
      grid: { color: '#3C3C3C' },
      pointLabels: {
        color: '#D4D4D4',
        font: { size: 12 },
      },
      ticks: {
        color: '#858585',
        backdropColor: 'transparent',
      },
    },
  },
  plugins: {
    legend: { display: false },
    tooltip: {
      backgroundColor: '#2D2D30',
      titleColor: '#D4D4D4',
      bodyColor: '#858585',
      borderColor: '#4EC9B0',
      borderWidth: 1,
    },
  },
};

function calculateLayerScore(layerData: any): number {
  // 根据各层数据计算 0-1 的强度分数
  // 简化示例，实际需要更复杂的计算
  return 0.7;
}
</script>
```

```vue
<!-- components/style/FeatureDetailPanel.vue -->
<template>
  <div class="feature-detail-panel">
    <n-collapse>
      <n-collapse-item title="词汇层特征" name="vocabulary">
        <div class="feature-content">
          <n-descriptions bordered :column="2">
            <n-descriptions-item label="词汇丰富度 (TTR)">
              {{ styleData.vocabulary.ttr.toFixed(2) }}
            </n-descriptions-item>
            <n-descriptions-item label="修正 TTR">
              {{ styleData.vocabulary.root_ttr.toFixed(2) }}
            </n-descriptions-item>
            <n-descriptions-item label="总词数" span="2">
              {{ styleData.vocabulary.total_words }}
            </n-descriptions-item>
            <n-descriptions-item label="常用形容词" span="2">
              {{ styleData.vocabulary.common_adjectives.slice(0, 5).map(([word, freq]) => `${word}(${freq})`).join(', ') }}
            </n-descriptions-item>
          </n-descriptions>
        </div>
      </n-collapse-item>
      
      <!-- 其他六层类似 -->
    </n-collapse>
  </div>
</template>
```

```vue
<!-- components/style/ExamplePassages.vue -->
<template>
  <div class="example-passages">
    <h3>示例段落</h3>
    <n-carousel v-if="passages.length > 1" show-dots>
      <n-card 
        v-for="(passage, index) in passages" 
        :key="index"
        class="passage-card"
      >
        <p class="passage-text">{{ passage.text }}</p>
        <div class="passage-annotations">
          <n-tag 
            v-for="(annotation, i) in passage.annotations" 
            :key="i"
            size="small"
            type="info"
          >
            {{ annotation }}
          </n-tag>
        </div>
      </n-card>
    </n-carousel>
    
    <div v-else class="passage-list">
      <n-card v-for="(passage, index) in passages" :key="index" class="passage-card">
        <p class="passage-text">{{ passage.text }}</p>
        <div class="passage-annotations">
          <n-tag v-for="(annotation, i) in passage.annotations" :key="i" size="small">
            {{ annotation }}
          </n-tag>
        </div>
      </n-card>
    </div>
  </div>
</template>
```

```vue
<!-- views/StyleReportView.vue -->
<template>
  <div class="style-report-view">
    <div class="report-header">
      <h1>风格分析报告</h1>
      <div class="report-meta">
        <span>来源：{{ sourceFile }}</span>
        <span>字数：{{ totalChars }}</span>
        <span>分析完成：{{ completedAt }}</span>
      </div>
    </div>
    
    <div class="report-content">
      <div class="radar-section">
        <h2>风格雷达图</h2>
        <StyleRadarChart :style-data="styleData" />
      </div>
      
      <div class="features-section">
        <h2>七层特征详情</h2>
        <FeatureDetailPanel :style-data="styleData" />
      </div>
      
      <div class="examples-section">
        <h2>示例段落</h2>
        <ExamplePassages :passages="examplePassages" />
      </div>
    </div>
    
    <div class="report-actions">
      <n-button @click="handleExport">导出报告</n-button>
      <n-button type="primary" @click="handleSaveStyle">
        保存风格档案
      </n-button>
      <n-button @click="handleReanalyze">重新分析</n-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRoute } from 'vue-router';
import { NButton, NCard, NTag, NDescriptions, NDescriptionsItem } from 'naive-ui';
import StyleRadarChart from '../components/style/StyleRadarChart.vue';
import FeatureDetailPanel from '../components/style/FeatureDetailPanel.vue';
import ExamplePassages from '../components/style/ExamplePassages.vue';

const route = useRoute();
const taskId = route.params.id as string;

const styleData = ref<StyleAnalysisResult | null>(null);
const sourceFile = ref('');
const totalChars = ref(0);
const completedAt = ref('');
const examplePassages = ref([]);

onMounted(async () => {
  await loadStyleData();
});

async function loadStyleData() {
  try {
    const response = await axios.get(`/api/style-analysis/${taskId}`);
    styleData.value = response.data.data;
    sourceFile.value = response.data.data.source_file;
    totalChars.value = response.data.data.total_chars;
    completedAt.value = formatDate(response.data.data.completed_at);
    examplePassages.value = extractExamplePassages(styleData.value);
  } catch (error) {
    console.error('加载风格数据失败:', error);
  }
}

function handleSaveStyle() {
  // 跳转到 Story 1.9 的保存流程
  router.push(`/styles/${taskId}/save`);
}

function handleExport() {
  // 导出 PDF 或图片
  window.open(`/api/style-analysis/${taskId}/export`, '_blank');
}

function handleReanalyze() {
  // 重新分析
  router.push(`/styles/upload`);
}
</script>

<style scoped lang="css">
.style-report-view {
  padding: 24px;
  max-width: 1200px;
  margin: 0 auto;
}

.report-header {
  margin-bottom: 32px;
}

.report-meta {
  display: flex;
  gap: 24px;
  color: #858585;
  margin-top: 8px;
}

.report-content {
  display: flex;
  flex-direction: column;
  gap: 32px;
}

.radar-section {
  background: #252526;
  padding: 24px;
  border-radius: 8px;
}

.features-section {
  background: #252526;
  padding: 24px;
  border-radius: 8px;
}

.report-actions {
  display: flex;
  gap: 16px;
  justify-content: flex-end;
  margin-top: 32px;
  padding-top: 24px;
  border-top: 1px solid #3C3C3C;
}
</style>
```

### API 端点

```
# 获取风格分析结果
GET    /api/style-analysis/:id              # 获取完整分析结果
GET    /api/style-analysis/:id/report       # 获取报告专用格式（含示例段落）

# 导出报告
GET    /api/style-analysis/:id/export       # 导出 PDF/图片
```

### 响应格式

```json
{
  "success": true,
  "data": {
    "task_id": "uuid",
    "status": "completed",
    "progress": 1.0,
    "source_file": "uploaded/novel.epub",
    "total_chars": 350000,
    "completed_at": "2026-04-09T10:30:00Z",
    "vocabulary": { ... },
    "sentence": { ... },
    "rhetoric": { ... },
    "narrative": { ... },
    "emotion": { ... },
    "pacing": { ... },
    "dialogue": { ... },
    "description": { ... },
    "style_vector": [...],
    "example_passages": [
      {
        "text": "萧峰纵身一跃，已落在擂台中央，提气喝道：'今日便要领教各位的高招！'",
        "annotations": ["动词选择有力", "短句节奏", "动作描写风格"]
      }
    ]
  }
}
```

---

## File Structure Requirements

### 前端文件结构

```
src/
├── views/
│   └── StyleReportView.vue        # 风格报告页面（NEW）
├── components/
│   └── style/
│       ├── StyleRadarChart.vue    # 风格雷达图（NEW）
│       ├── FeatureDetailPanel.vue # 特征详情面板（NEW）
│       ├── ExamplePassages.vue    # 示例段落展示（NEW）
│       └── ReportActions.vue      # 报告操作按钮（NEW）
├── stores/
│   └── style.ts                   # 风格状态管理
└── services/
    └── style.ts                   # 风格 API 客户端
```

---

## Testing Requirements

### 前端测试（Vitest）

```typescript
// tests/StyleReportView.test.ts

describe('StyleReportView', () => {
  it('should load and display style data', async () => {
    // 测试报告加载
  });

  it('should render radar chart with 7 layers', async () => {
    // 测试雷达图渲染
  });

  it('should display feature details for each layer', async () => {
    // 测试特征详情展示
  });

  it('should show example passages with annotations', async () => {
    // 测试示例段落展示
  });

  it('should handle export action', async () => {
    // 测试导出功能
  });

  it('should navigate to save style flow', async () => {
    // 测试保存入口跳转
  });
});
```

---

## Story Completion Status

- [ ] 前端：风格报告页面框架
- [ ] 前端：风格雷达图组件
- [ ] 前端：特征详情面板
- [ ] 前端：示例段落展示组件
- [ ] 前端：报告操作按钮
- [ ] 后端：报告数据查询接口
- [ ] 后端：报告导出接口
- [ ] 测试：组件单元测试
- [ ] 测试：端到端测试

---

**Story created:** 2026-04-09  
**Last updated:** 2026-04-09  
**Status:** ready-for-dev
