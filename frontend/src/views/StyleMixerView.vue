<template>
  <div class="style-mixer-view">
    <div class="mixer-header">
      <h1>风格混合器</h1>
      <p class="subtitle">选择多个风格档案，调整权重，创造独特的混合风格</p>
    </div>

    <div class="mixer-content">
      <!-- 选择风格 -->
      <div class="select-section">
        <h2>选择风格档案（2-5个）</h2>
        <div class="style-grid">
          <n-card
            v-for="profile in availableProfiles"
            :key="profile.id"
            :class="{ selected: selectedStyles.includes(profile.id) }"
            @click="toggleStyle(profile.id)"
          >
            <div class="style-card">
              <h4>{{ profile.name }}</h4>
              <p>{{ profile.source_file || '未知来源' }}</p>
            </div>
          </n-card>
        </div>
      </div>

      <!-- 权重设置 -->
      <div v-if="selectedStyles.length >= 2" class="weight-section">
        <h2>设置权重</h2>
        <div class="weight-list">
          <div v-for="styleId in selectedStyles" :key="styleId" class="weight-item">
            <span class="style-name">{{ getStyleName(styleId) }}</span>
            <n-slider
              v-model:value="weights[styleId]"
              :min="0"
              :max="100"
              :step="1"
              style="width: 200px;"
            />
            <span class="weight-value">{{ weights[styleId] || 0 }}%</span>
          </div>
        </div>
        <div class="weight-summary">
          <span>权重总和: {{ totalWeight }}%</span>
          <n-tag :type="totalWeight === 100 ? 'success' : 'error'">
            {{ totalWeight === 100 ? '有效' : '需调整' }}
          </n-tag>
        </div>
      </div>

      <!-- 预览 -->
      <div v-if="preview && totalWeight === 100" class="preview-section">
        <h2>混合预览</h2>
        <n-card>
          <div class="preview-content">
            <h3>{{ preview.name }}</h3>
            <div class="layer-scores">
              <div v-for="(score, index) in preview.layer_scores" :key="index" class="layer-item">
                <span>{{ layerNames[index] }}</span>
                <div class="score-bar">
                  <div class="score-fill" :style="{ width: (score * 100) + '%' }"></div>
                </div>
                <span>{{ (score * 100).toFixed(0) }}%</span>
              </div>
            </div>
          </div>
        </n-card>
      </div>

      <!-- 操作 -->
      <div class="actions">
        <n-button @click="cancel">取消</n-button>
        <n-button @click="previewMix" :disabled="totalWeight !== 100">
          预览混合效果
        </n-button>
        <n-button type="primary" @click="saveMix" :disabled="!preview || totalWeight !== 100">
          保存为新风格
        </n-button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { NCard, NButton, NSlider, NTag, useMessage } from 'naive-ui'
import axios from 'axios'

const router = useRouter()
const message = useMessage()

const availableProfiles = ref<any[]>([])
const selectedStyles = ref<string[]>([])
const weights = ref<Record<string, number>>({})
const preview = ref<any>(null)

const layerNames = ['词汇层', '句式层', '修辞层', '叙事层', '情感层', '节奏层', '对话层', '描写层']

const totalWeight = computed(() => {
  return selectedStyles.value.reduce((sum, id) => sum + (weights.value[id] || 0), 0)
})

onMounted(async () => {
  await loadProfiles()
})

async function loadProfiles() {
  try {
    const response = await axios.get('/api/style-profiles')
    availableProfiles.value = response.data.data
  } catch (error) {
    console.error('加载风格档案失败:', error)
    message.error('加载风格档案失败')
  }
}

function toggleStyle(id: string) {
  const index = selectedStyles.value.indexOf(id)
  if (index > -1) {
    selectedStyles.value.splice(index, 1)
    delete weights.value[id]
  } else if (selectedStyles.value.length < 5) {
    selectedStyles.value.push(id)
    weights.value[id] = Math.floor(100 / (selectedStyles.value.length + 1))
  }
}

function getStyleName(id: string): string {
  return availableProfiles.value.find(p => p.id === id)?.name || '未知'
}

async function previewMix() {
  if (totalWeight.value !== 100) {
    message.warning('权重总和必须为 100%')
    return
  }

  try {
    const response = await axios.post('/api/styles/mix/preview', {
      style_ids: selectedStyles.value,
      weights: selectedStyles.value.map(id => weights.value[id] || 0)
    })
    preview.value = response.data.data
  } catch (error: any) {
    message.error(error.response?.data?.message || '预览失败')
  }
}

async function saveMix() {
  if (!preview.value || totalWeight.value !== 100) return

  try {
    const response = await axios.post('/api/styles/mix/save', {
      style_ids: selectedStyles.value,
      weights: selectedStyles.value.map(id => weights.value[id] || 0)
    })
    message.success('混合风格保存成功')
    router.push('/style-library')
  } catch (error: any) {
    message.error(error.response?.data?.message || '保存失败')
  }
}

function cancel() {
  router.back()
}
</script>

<style scoped>
.style-mixer-view {
  padding: 24px;
  max-width: 1200px;
  margin: 0 auto;
}

.mixer-header {
  margin-bottom: 32px;
}

.mixer-header h1 {
  color: #D4D4D4;
  margin-bottom: 8px;
}

.subtitle {
  color: #858585;
}

.mixer-content {
  display: flex;
  flex-direction: column;
  gap: 32px;
}

.select-section,
.weight-section,
.preview-section {
  background: #252526;
  padding: 24px;
  border-radius: 8px;
}

.select-section h2,
.weight-section h2,
.preview-section h2 {
  color: #D4D4D4;
  font-size: 18px;
  margin-bottom: 16px;
}

.style-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 16px;
}

.style-card {
  cursor: pointer;
}

.style-card h4 {
  color: #D4D4D4;
  margin: 0 0 8px 0;
}

.style-card p {
  color: #858585;
  font-size: 12px;
}

.selected {
  border: 2px solid #4EC9B0 !important;
}

.weight-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.weight-item {
  display: flex;
  align-items: center;
  gap: 16px;
}

.style-name {
  width: 150px;
  color: #D4D4D4;
}

.weight-value {
  width: 50px;
  text-align: right;
  color: #858585;
}

.weight-summary {
  display: flex;
  align-items: center;
  gap: 16px;
  margin-top: 16px;
  padding-top: 16px;
  border-top: 1px solid #3C3C3C;
}

.layer-scores {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.layer-item {
  display: flex;
  align-items: center;
  gap: 12px;
}

.layer-item span:first-child {
  width: 80px;
  color: #D4D4D4;
}

.score-bar {
  flex: 1;
  height: 8px;
  background: #3C3C3C;
  border-radius: 4px;
  overflow: hidden;
}

.score-fill {
  height: 100%;
  background: #4EC9B0;
  border-radius: 4px;
}

.actions {
  display: flex;
  gap: 16px;
  justify-content: flex-end;
}
</style>