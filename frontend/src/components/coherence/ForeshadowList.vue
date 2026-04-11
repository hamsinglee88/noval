<template>
  <div class="foreshadow-list">
    <div class="list-header">
      <h3>伏笔追踪器</h3>
      <n-button size="small" @click="$emit('refresh')">刷新</n-button>
    </div>
    
    <div v-if="foreshadows.length === 0" class="empty-state">
      <n-empty description="暂无伏笔" />
    </div>
    
    <div v-else class="foreshadow-items">
      <div 
        v-for="item in foreshadows" 
        :key="item.id" 
        class="foreshadow-item"
        :class="getStatusClass(item.status)"
      >
        <div class="item-header">
          <n-tag :type="getTypeTag(item.foreshadow_type)" size="small">
            {{ getTypeLabel(item.foreshadow_type) }}
          </n-tag>
          <n-tag :type="getStatusTag(item.status)" size="small">
            {{ getStatusLabel(item.status) }}
          </n-tag>
        </div>
        
        <p class="item-content">{{ item.content }}</p>
        
        <div class="item-meta">
          <span>置信度: {{ (item.confidence_score * 100).toFixed(0) }}%</span>
          <span>章节: {{ item.chapter_id.slice(0, 8) }}...</span>
        </div>
      </div>
    </div>
    
    <div v-if="foreshadows.length > 0" class="list-summary">
      <span>共 {{ foreshadows.length }} 个伏笔</span>
      <span>{{ activeCount }} 个活跃</span>
      <span>{{ overdueCount }} 个逾期</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { NButton, NTag, NEmpty } from 'naive-ui'

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
}>()

defineEmits(['refresh'])

const activeCount = computed(() => 
  props.foreshadows.filter(f => f.status === 'Active').length
)

const overdueCount = computed(() => 
  props.foreshadows.filter(f => f.status === 'Overdue').length
)

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
}

.list-summary {
  display: flex;
  gap: 16px;
  margin-top: 16px;
  padding-top: 12px;
  border-top: 1px solid #3c3c3c;
  color: #858585;
  font-size: 12px;
}

.empty-state {
  padding: 40px 0;
}
</style>