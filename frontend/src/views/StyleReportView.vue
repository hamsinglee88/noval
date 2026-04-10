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
          <span><strong>来源：</strong>{{ styleData.source_file_path || styleData.source_file || '未知' }}</span>
          <span><strong>字数：</strong>{{ formatNumber(styleData.total_chars || 0) }}</span>
          <span><strong>创建时间：</strong>{{ formatDate(styleData.created_at || styleData.completed_at) }}</span>
        </div>
      </div>
      
      <div class="report-content">
        <!-- 雷达图区域 -->
        <div class="radar-section">
          <h2>风格雷达图</h2>
          <div class="radar-placeholder">
            <p>雷达图可视化（七层风格特征）</p>
            <div class="layer-list">
              <div v-for="(layer, index) in layerScores" :key="index" class="layer-item">
                <span class="layer-name">{{ layer.name }}</span>
                <div class="layer-bar">
                  <div class="layer-fill" :style="{ width: (layer.score * 100) + '%' }"></div>
                </div>
                <span class="layer-score">{{ (layer.score * 100).toFixed(0) }}%</span>
              </div>
            </div>
          </div>
        </div>
        
        <!-- 特征详情区域 -->
        <div class="features-section">
          <h2>七层特征详情</h2>
          <n-collapse>
            <n-collapse-item title="词汇层特征" name="vocabulary">
              <n-descriptions bordered :column="2">
                <n-descriptions-item label="词汇丰富度 (TTR)">
                  {{ styleData.vocabulary?.ttr?.toFixed(2) || '-' }}
                </n-descriptions-item>
                <n-descriptions-item label="修正 TTR">
                  {{ styleData.vocabulary?.root_ttr?.toFixed(2) || '-' }}
                </n-descriptions-item>
                <n-descriptions-item label="总词数" :span="2">
                  {{ styleData.vocabulary?.total_words || '-' }}
                </n-descriptions-item>
              </n-descriptions>
            </n-collapse-item>
            
            <n-collapse-item title="句式层特征" name="sentence">
              <n-descriptions bordered :column="2">
                <n-descriptions-item label="平均句长">
                  {{ styleData.sentence?.avg_sentence_length?.toFixed(1) || '-' }} 字
                </n-descriptions-item>
                <n-descriptions-item label="短句比例">
                  {{ formatPercent(styleData.sentence?.short_sentence_ratio) }}
                </n-descriptions-item>
              </n-descriptions>
            </n-collapse-item>

            <n-collapse-item title="修辞层特征" name="rhetoric">
              <n-descriptions bordered :column="2">
                <n-descriptions-item label="隐喻频率">
                  {{ styleData.rhetoric?.metaphor_frequency?.toFixed(1) || '-' }} 次/万字
                </n-descriptions-item>
                <n-descriptions-item label="明喻频率">
                  {{ styleData.rhetoric?.simile_frequency?.toFixed(1) || '-' }} 次/万字
                </n-descriptions-item>
              </n-descriptions>
            </n-collapse-item>

            <n-collapse-item title="叙事层特征" name="narrative">
              <n-descriptions bordered :column="2">
                <n-descriptions-item label="叙事视角">
                  {{ styleData.narrative?.pov_type || '-' }}
                </n-descriptions-item>
                <n-descriptions-item label="Show vs Tell">
                  {{ formatPercent(styleData.narrative?.show_vs_tell_ratio) }}
                </n-descriptions-item>
              </n-descriptions>
            </n-collapse-item>

            <n-collapse-item title="情感层特征" name="emotion">
              <n-descriptions bordered :column="2">
                <n-descriptions-item label="情感基调">
                  {{ styleData.emotion?.overall_tone || '-' }}
                </n-descriptions-item>
                <n-descriptions-item label="基调置信度">
                  {{ formatPercent(styleData.emotion?.tone_confidence) }}
                </n-descriptions-item>
              </n-descriptions>
            </n-collapse-item>

            <n-collapse-item title="节奏层特征" name="pacing">
              <n-descriptions bordered :column="2">
                <n-descriptions-item label="平均章节长度">
                  {{ styleData.pacing?.avg_chapter_length?.toFixed(0) || '-' }} 字
                </n-descriptions-item>
                <n-descriptions-item label="悬念结尾比例">
                  {{ formatPercent(styleData.pacing?.cliffhanger_ratio) }}
                </n-descriptions-item>
              </n-descriptions>
            </n-collapse-item>

            <n-collapse-item title="对话层特征" name="dialogue">
              <n-descriptions bordered :column="2">
                <n-descriptions-item label="对话比例">
                  {{ formatPercent(styleData.dialogue?.dialogue_ratio) }}
                </n-descriptions-item>
                <n-descriptions-item label="角色声音区分度">
                  {{ formatPercent(styleData.dialogue?.character_voice_distinction) }}
                </n-descriptions-item>
              </n-descriptions>
            </n-collapse-item>

            <n-collapse-item title="描写层特征" name="description">
              <n-descriptions bordered :column="2">
                <n-descriptions-item label="描写比例">
                  {{ formatPercent(styleData.description_data?.description_ratio || styleData.description?.description_ratio) }}
                </n-descriptions-item>
                <n-descriptions-item label="详细程度">
                  {{ formatPercent(styleData.description_data?.detail_granularity || styleData.description?.detail_granularity) }}
                </n-descriptions-item>
              </n-descriptions>
            </n-collapse-item>
          </n-collapse>
        </div>
      </div>
      
      <!-- 操作按钮 -->
      <div class="report-actions">
        <n-button @click="handleExport">
          导出报告
        </n-button>
        <n-button v-if="taskId && !styleData.name" type="primary" @click="handleSaveStyle">
          保存风格档案
        </n-button>
        <n-button @click="goBack">
          返回
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
  NCollapse,
  NCollapseItem,
  NDescriptions,
  NDescriptionsItem,
  useMessage 
} from 'naive-ui';
import axios from 'axios';

const route = useRoute();
const router = useRouter();
const message = useMessage();

const id = route.params.id as string;
const isTaskId = route.name === 'style-report';

const loading = ref(true);
const error = ref<string | null>(null);
const styleData = ref<any>(null);
const taskId = ref<string | null>(isTaskId ? id : null);

// 计算各层分数用于显示
const layerScores = computed(() => {
  if (!styleData.value) return [];
  
  return [
    { name: '词汇层', score: calculateLayerScore(styleData.value.vocabulary, 'vocabulary') },
    { name: '句式层', score: calculateLayerScore(styleData.value.sentence, 'sentence') },
    { name: '修辞层', score: calculateLayerScore(styleData.value.rhetoric, 'rhetoric') },
    { name: '叙事层', score: calculateLayerScore(styleData.value.narrative, 'narrative') },
    { name: '情感层', score: calculateLayerScore(styleData.value.emotion, 'emotion') },
    { name: '节奏层', score: calculateLayerScore(styleData.value.pacing, 'pacing') },
    { name: '对话层', score: calculateLayerScore(styleData.value.dialogue, 'dialogue') },
    { name: '描写层', score: calculateLayerScore(styleData.value.description_data || styleData.value.description, 'description') },
  ];
});

onMounted(async () => {
  await loadStyleData();
});

async function loadStyleData() {
  loading.value = true;
  error.value = null;
  
  try {
    // 根据路由判断是任务还是档案
    const endpoint = isTaskId 
      ? `/api/styles/analyze/${id}`
      : `/api/style-profiles/${id}`;
    
    const response = await axios.get(endpoint);
    styleData.value = response.data.data;
    
    // 如果是档案，提取 task_id
    if (!isTaskId && styleData.value.task_id) {
      taskId.value = styleData.value.task_id;
    }
  } catch (err: any) {
    console.error('加载风格数据失败:', err);
    error.value = err.response?.data?.message || '加载风格数据失败';
  } finally {
    loading.value = false;
  }
}

function calculateLayerScore(layerData: any, layerType: string): number {
  if (!layerData) return 0;
  
  switch (layerType) {
    case 'vocabulary':
      return Math.min((layerData.ttr || 0) * 2, 1);
    case 'sentence':
      return Math.min((layerData.avg_sentence_length || 0) / 50, 1);
    case 'rhetoric':
      return Math.min(((layerData.metaphor_frequency || 0) + (layerData.simile_frequency || 0)) / 100, 1);
    case 'narrative':
      return layerData.show_vs_tell_ratio || 0.5;
    case 'emotion':
      return layerData.tone_confidence || 0.5;
    case 'pacing':
      return Math.min((layerData.avg_chapter_length || 0) / 10000, 1);
    case 'dialogue':
      return layerData.dialogue_ratio || 0.3;
    case 'description':
      return layerData.description_ratio || 0.5;
    default:
      return 0.5;
  }
}

function formatNumber(num: number): string {
  return num.toLocaleString();
}

function formatDate(dateStr: string | undefined): string {
  if (!dateStr) return '未知';
  const date = new Date(dateStr);
  return date.toLocaleString('zh-CN');
}

function formatPercent(value: number | undefined): string {
  if (value === undefined || value === null) return '-';
  return `${(value * 100).toFixed(1)}%`;
}

function handleExport() {
  if (isTaskId) {
    window.open(`/api/styles/analyze/${id}/export`, '_blank');
  } else {
    window.open(`/api/style-profiles/${id}/export`, '_blank');
  }
}

function handleSaveStyle() {
  if (taskId.value) {
    router.push(`/styles/${taskId.value}/save`);
  }
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
  flex-wrap: wrap;
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
.features-section {
  background: #252526;
  padding: 24px;
  border-radius: 8px;
}

.radar-placeholder {
  padding: 20px;
}

.layer-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.layer-item {
  display: flex;
  align-items: center;
  gap: 12px;
}

.layer-name {
  width: 80px;
  color: #D4D4D4;
}

.layer-bar {
  flex: 1;
  height: 8px;
  background: #3C3C3C;
  border-radius: 4px;
  overflow: hidden;
}

.layer-fill {
  height: 100%;
  background: #4EC9B0;
  border-radius: 4px;
  transition: width 0.3s ease;
}

.layer-score {
  width: 50px;
  text-align: right;
  color: #858585;
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