<template>
  <div class="style-report-view">
    <!-- 加载状态 -->
    <div v-if="loading" class="loading-state">
      <n-spin size="large" />
      <p>加载风格报告中...</p>
    </div>
    
    <!-- 错误状态 -->
    <div v-else-if="error" class="error-state">
      <n-result status="error" title="加载失败" :description="error">
        <template #footer>
          <n-space>
            <n-button @click="loadStyleData">重试</n-button>
            <n-button type="primary" @click="goBack">返回</n-button>
          </n-space>
        </template>
      </n-result>
    </div>
    
    <!-- 分析未完成状态 -->
    <div v-else-if="styleData && styleData.status !== 'completed'" class="pending-state">
      <n-result 
        status="info" 
        title="分析进行中" 
        :description="`当前进度: ${((styleData.progress || 0) * 100).toFixed(0)}%`"
      >
        <template #footer>
          <n-space>
            <n-button type="primary" @click="loadStyleData">刷新</n-button>
            <n-button @click="goBack">返回</n-button>
          </n-space>
        </template>
      </n-result>
    </div>
    
    <!-- 正常报告内容 -->
    <template v-else-if="styleData">
      <div class="report-header">
        <h1>风格分析报告</h1>
        <div class="report-meta">
          <span><strong>来源：</strong>{{ styleData.source_file || '未知' }}</span>
          <span><strong>字数：</strong>{{ formatNumber(styleData.total_chars || 0) }}</span>
          <span><strong>分析完成：</strong>{{ formatDate(styleData.completed_at) }}</span>
        </div>
      </div>
      
      <div class="report-content">
        <!-- 雷达图区域 -->
        <div class="radar-section">
          <h2>风格雷达图</h2>
          <StyleRadarChart :style-data="normalizedStyleData" />
        </div>
        
        <!-- 特征详情区域 -->
        <div class="features-section">
          <h2>七层特征详情</h2>
          <FeatureDetailPanel :style-data="styleData" />
        </div>
        
        <!-- 示例段落区域 -->
        <div class="examples-section">
          <h2>示例段落</h2>
          <ExamplePassages :passages="examplePassages" />
        </div>
      </div>
      
      <!-- 操作按钮 -->
      <div class="report-actions">
        <n-button @click="handleExport">
          导出报告
        </n-button>
        <n-button type="primary" @click="handleSaveStyle">
          保存风格档案
        </n-button>
        <n-button @click="handleReanalyze">
          重新分析
        </n-button>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { 
  NButton, 
  NSpin, 
  NResult, 
  NSpace, 
  useMessage 
} from 'naive-ui';
// 图标暂时不使用，可以后续添加
import axios from 'axios';
import StyleRadarChart from '../components/style/StyleRadarChart.vue';
import FeatureDetailPanel from '../components/style/FeatureDetailPanel.vue';
import ExamplePassages from '../components/style/ExamplePassages.vue';

const route = useRoute();
const router = useRouter();
const message = useMessage();

const taskId = route.params.id as string;

const loading = ref(true);
const error = ref<string | null>(null);
const styleData = ref<any>(null);

// 计算归一化的风格数据用于雷达图
const normalizedStyleData = computed(() => {
  if (!styleData.value) {
    return {
      vocabulary: { score: 0 },
      sentence: { score: 0 },
      rhetoric: { score: 0 },
      narrative: { score: 0 },
      emotion: { score: 0 },
      pacing: { score: 0 },
      dialogue: { score: 0 },
      description: { score: 0 },
    };
  }
  
  return {
    vocabulary: { ...styleData.value.vocabulary, score: calculateLayerScore(styleData.value.vocabulary) },
    sentence: { ...styleData.value.sentence, score: calculateLayerScore(styleData.value.sentence) },
    rhetoric: { ...styleData.value.rhetoric, score: calculateLayerScore(styleData.value.rhetoric) },
    narrative: { ...styleData.value.narrative, score: calculateLayerScore(styleData.value.narrative) },
    emotion: { ...styleData.value.emotion, score: calculateLayerScore(styleData.value.emotion) },
    pacing: { ...styleData.value.pacing, score: calculateLayerScore(styleData.value.pacing) },
    dialogue: { ...styleData.value.dialogue, score: calculateLayerScore(styleData.value.dialogue) },
    description: { ...styleData.value.description, score: calculateLayerScore(styleData.value.description) },
  };
});

// 示例段落
const examplePassages = computed(() => {
  if (!styleData.value?.example_passages) {
    return [];
  }
  return styleData.value.example_passages;
});

onMounted(async () => {
  await loadStyleData();
});

async function loadStyleData() {
  loading.value = true;
  error.value = null;
  
  try {
    const response = await axios.get(`/api/style-analysis/${taskId}`);
    styleData.value = response.data.data;
  } catch (err: any) {
    console.error('加载风格数据失败:', err);
    error.value = err.response?.data?.message || '加载风格数据失败';
  } finally {
    loading.value = false;
  }
}

function calculateLayerScore(layerData: any): number {
  if (!layerData) return 0;
  
  // 根据不同层的特征计算综合分数
  // 这里使用简化的计算方式，实际可以根据业务需求调整
  if (layerData.ttr !== undefined) {
    // 词汇层：TTR 越高，分数越高
    return Math.min(layerData.ttr * 2, 1);
  }
  if (layerData.avg_sentence_length !== undefined) {
    // 句式层：根据平均句长归一化
    return Math.min(layerData.avg_sentence_length / 50, 1);
  }
  if (layerData.metaphor_frequency !== undefined) {
    // 修辞层：根据修辞频率归一化
    return Math.min((layerData.metaphor_frequency + layerData.simile_frequency) / 100, 1);
  }
  if (layerData.pov_type !== undefined) {
    // 叙事层：根据 show_vs_tell 比例
    return layerData.show_vs_tell_ratio || 0.5;
  }
  if (layerData.overall_tone !== undefined) {
    // 情感层：根据基调置信度
    return layerData.tone_confidence || 0.5;
  }
  if (layerData.avg_chapter_length !== undefined) {
    // 节奏层：根据章节长度归一化
    return Math.min(layerData.avg_chapter_length / 10000, 1);
  }
  if (layerData.dialogue_ratio !== undefined) {
    // 对话层：根据对话比例
    return layerData.dialogue_ratio;
  }
  if (layerData.description_ratio !== undefined) {
    // 描写层：根据描写比例
    return layerData.description_ratio;
  }
  
  return 0.5;
}

function formatNumber(num: number): string {
  return num.toLocaleString();
}

function formatDate(dateStr: string | undefined): string {
  if (!dateStr) return '未知';
  const date = new Date(dateStr);
  return date.toLocaleString('zh-CN');
}

function handleExport() {
  window.open(`/api/style-analysis/${taskId}/export`, '_blank');
}

function handleSaveStyle() {
  router.push(`/styles/${taskId}/save`);
}

function handleReanalyze() {
  router.push('/styles/upload');
}

function goBack() {
  router.back();
}
</script>

<style scoped>
.style-report-view {
  padding: 24px;
  max-width: 1200px;
  margin: 0 auto;
}

.loading-state,
.error-state,
.pending-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 400px;
  gap: 16px;
}

.report-header {
  margin-bottom: 32px;
}

.report-header h1 {
  color: #D4D4D4;
  margin-bottom: 8px;
}

.report-meta {
  display: flex;
  gap: 24px;
  color: #858585;
  margin-top: 8px;
}

.report-meta span {
  font-size: 14px;
}

.report-meta strong {
  color: #A6A6A6;
}

.report-content {
  display: flex;
  flex-direction: column;
  gap: 32px;
}

.report-content h2 {
  color: #D4D4D4;
  font-size: 18px;
  margin-bottom: 16px;
}

.radar-section,
.features-section,
.examples-section {
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