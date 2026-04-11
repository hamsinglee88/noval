<template>
  <div class="foreshadow-list">
    <div class="list-header">
      <h3>伏笔追踪器</h3>
      <n-button size="small" @click="$emit('refresh')" :loading="loading">
        刷新
      </n-button>
    </div>
    
    <!-- 加载状态 -->
    <div v-if="loading" class="loading-state">
      <n-spin size="medium" />
      <p>加载伏笔中...</p>
    </div>
    
    <!-- 错误状态 -->
    <div v-else-if="error" class="error-state">
      <n-result status="error" title="加载失败" :description="error">
        <template #footer>
          <n-button @click="$emit('refresh')">重试</n-button>
        </template>
      </n-result>
    </div>
    
    <!-- 空状态 -->
    <div v-else-if="foreshadows.length === 0" class="empty-state">
      <n-empty description="暂无伏笔" />
    </div>
    
    <!-- 伏笔列表 -->
    <div v-else class="foreshadow-items">
      <div 
        v-for="item in sortedForeshadows" 
        :key="item.id" 
        class="foreshadow-item"
        :class="getStatusClass(item.status)"
        @click="$emit('select', item)"
      >
        <div class="item-header">
          <n-tag :type="getTypeTag(item.foreshadow_type)" size="small">
            {{ getTypeLabel(item.foreshadow_type) }}
          </n-tag>
          <n-tag :type="getStatusTag(item.status)" size="small">
            {{ getStatusLabel(item.status) }}
          </n-tag>
          <n-tag v-if="item.confidence_score >= 0.8" type="warning" size="small">
            高置信度
          </n-tag>
        </div>
        
        <p class="item-content">{{ truncateContent(item.content) }}</p>
        
        <div class="item-meta">
          <span class="confidence">
            <span class="confidence-bar" :style="{ width: (item.confidence_score * 100) + '%' }"></span>
            {{ (item.confidence_score * 100).toFixed(0) }}%
          </span>
          <span class="chapter">章节: {{ formatChapterId(item.chapter_id) }}</span>
        </div>
      </div>
    </div>
    
    <div v-if="!loading && !error && foreshadows.length > 0" class="list-summary">
      <span>共 {{ foreshadows.length }} 个伏笔</span>
      <span class="active">{{ activeCount }} 个活跃</span>
      <span v-if="overdueCount > 0" class="overdue">{{ overdueCount }} 个逾期</span>
      <span class="high-confidence">{{ highConfidenceCount }} 个高置信度</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { NButton, NTag, NEmpty, NSpin, NResult } from 'naive-ui'

interface Foreshadow {
  id: string
  content: string
  chapter_id: string
  foreshadow_type: string
  status: string
  confidence_score: number
}

const props = defineProps<{
  foreshadows: Foreshadow[]
  loading?: boolean
  error?: string | null
}>()

defineEmits(['refresh', 'select'])

const sortedForeshadows = computed(() => 
  [...props.foreshadows].sort((a, b) => b.confidence_score - a.confidence_score)
)

const activeCount = computed(() => 
  props.foreshadows.filter(f => f.status === 'Active').length
)

const overdueCount = computed(() => 
  props.foreshadows.filter(f => f.status === 'Overdue').length
)

const highConfidenceCount = computed(() => 
  props.foreshadows.filter(f => f.confidence_score >= 0.8).length
)

function truncateContent(content: string, maxLength = 100): string {
  if (content.length <= maxLength) return content
  return content.slice(0, maxLength) + '...'
}

function formatChapterId(id: string): string {
  return id.slice(0, 8) + '...'
}

function getStatusClass(status: string): string {
  return `status-${status.toLowerCase()}`
}

function getTypeTag(type: string): 'default' | 'primary' | 'info' | 'success' | 'warning' | 'error' {
  const map: Record<string, any> = {
    Plot: 'primary',
    Character: 'success',
    World: 'info',
    Emotional: 'warning',
  }
  return map[type] || 'default'
}

function getTypeLabel(type: string): string {
  const map: Record<string, string> = {
    Plot: '情节',
    Character: '角色',
    World: '世界观',
    Emotional: '情感',
  }
  return map[type] || type
}

function getStatusTag(status: string): 'default' | 'primary' | 'info' | 'success' | 'warning' | 'error' {
  const map: Record<string, any> = {
    Active: 'warning',
    Resolved: 'success',
    Abandoned: 'default',
    Overdue: 'error',
  }
  return map[status] || 'default'
}

function getStatusLabel(status: string): string {
  const map: Record<string, string> = {
    Active: '活跃',
    Resolved: '已回收',
    Abandoned: '已废弃',
    Overdue: '逾期',
  }
  return map[status] || status
}
</script>

<style scoped>
.foreshadow-list {
  background: #252526;
  border-radius: 8px;
  padding: 16px;
}

.list-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.list-header h3 {
  margin: 0;
  color: #d4d4d4;
}

.loading-state,
.error-state,
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 40px 0;
  gap: 16px;
}

.foreshadow-items {
  display: flex;
  flex-direction: column;
  gap: 12px;
  max-height: 400px;
  overflow-y: auto;
}

.foreshadow-item {
  background: #2d2d2d;
  border-radius: 6px;
  padding: 12px;
  border-left: 3px solid #3c3c3c;
  cursor: pointer;
  transition: background 0.2s;
}

.foreshadow-item:hover {
  background: #333;
}

.foreshadow-item.status-active {
  border-left-color: #f0ad4e;
}

.foreshadow-item.status-resolved {
  border-left-color: #5cb85c;
}

.foreshadow-item.status-overdue {
  border-left-color: #d9534f;
}

.item-header {
  display: flex;
  gap: 8px;
  margin-bottom: 8px;
  flex-wrap: wrap;
}

.item-content {
  color: #d4d4d4;
  font-size: 14px;
  line-height: 1.5;
  margin: 0 0 8px 0;
}

.item-meta {
  display: flex;
  gap: 16px;
  color: #858585;
  font-size: 12px;
  align-items: center;
}

.confidence {
  display: flex;
  align-items: center;
  gap: 4px;
}

.confidence-bar {
  display: inline-block;
  width: 40px;
  height: 4px;
  background: #3c3c3c;
  border-radius: 2px;
  position: relative;
}

.confidence-bar::after {
  content: '';
  position: absolute;
  left: 0;
  top: 0;
  height: 100%;
  background: #4EC9B0;
  border-radius: 2px;
}

.list-summary {
  display: flex;
  gap: 16px;
  margin-top: 16px;
  padding-top: 12px;
  border-top: 1px solid #3c3c3c;
  color: #858585;
  font-size: 12px;
  flex-wrap: wrap;
}

.list-summary .active {
  color: #f0ad4e;
}

.list-summary .overdue {
  color: #d9534f;
}

.list-summary .high-confidence {
  color: #4EC9B0;
}
</style>