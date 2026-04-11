<template>
  <div class="foreshadow-map">
    <div class="map-header">
      <h3>伏笔地图</h3>
      <n-button size="small" @click="$emit('refresh')" :loading="loading">
        刷新
      </n-button>
    </div>
    
    <!-- 加载状态 -->
    <div v-if="loading" class="loading-state">
      <n-spin size="medium" />
      <p>加载伏笔地图中...</p>
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
    <div v-else-if="chapters.length === 0" class="empty-state">
      <n-empty description="暂无章节" />
    </div>
    
    <!-- 地图内容 -->
    <div v-else class="map-container">
      <div 
        v-for="(chapter, index) in chapters" 
        :key="chapter.id" 
        class="chapter-node"
        :class="{ 'has-overdue': hasOverdueForeshadows(chapter.id) }"
        @click="$emit('select-chapter', chapter)"
      >
        <div class="chapter-label">第{{ index + 1 }}章</div>
        <div class="foreshadow-dots">
          <div 
            v-for="f in getChapterForeshadows(chapter.id)" 
            :key="f.id"
            class="foreshadow-dot"
            :class="f.status.toLowerCase()"
            :title="f.content"
            @click.stop="$emit('select-foreshadow', f)"
          ></div>
        </div>
        <div v-if="getChapterForeshadows(chapter.id).length > 0" class="chapter-count">
          {{ getChapterForeshadows(chapter.id).length }}
        </div>
      </div>
    </div>
    
    <div v-if="!loading && !error && chapters.length > 0" class="map-legend">
      <div class="legend-item">
        <span class="dot active"></span> 活跃
      </div>
      <div class="legend-item">
        <span class="dot resolved"></span> 已回收
      </div>
      <div class="legend-item">
        <span class="dot overdue"></span> 逾期
      </div>
      <div class="legend-summary">
        共 {{ foreshadows.length }} 个伏笔
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { NButton, NSpin, NResult, NEmpty } from 'naive-ui'

interface Chapter { 
  id: string 
  title?: string
}

interface Foreshadow { 
  id: string
  chapter_id: string
  status: string
  content: string
}

const props = defineProps<{
  chapters: Chapter[]
  foreshadows: Foreshadow[]
  loading?: boolean
  error?: string | null
}>()

defineEmits(['refresh', 'select-chapter', 'select-foreshadow'])

// 使用 computed 缓存计算结果
const foreshadowsByChapter = computed(() => {
  const map = new Map<string, Foreshadow[]>()
  for (const f of props.foreshadows) {
    const list = map.get(f.chapter_id) || []
    list.push(f)
    map.set(f.chapter_id, list)
  }
  return map
})

function getChapterForeshadows(chapterId: string): Foreshadow[] {
  return foreshadowsByChapter.value.get(chapterId) || []
}

function hasOverdueForeshadows(chapterId: string): boolean {
  return getChapterForeshadows(chapterId).some(f => f.status === 'Overdue')
}
</script>

<style scoped>
.foreshadow-map {
  background: #252526;
  border-radius: 8px;
  padding: 16px;
}

.map-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.map-header h3 {
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

.map-container {
  display: flex;
  gap: 8px;
  overflow-x: auto;
  padding: 16px 0;
}

.chapter-node {
  min-width: 60px;
  text-align: center;
  cursor: pointer;
  padding: 8px;
  border-radius: 4px;
  transition: background 0.2s;
}

.chapter-node:hover {
  background: #2d2d2d;
}

.chapter-node.has-overdue {
  background: rgba(217, 83, 79, 0.1);
}

.chapter-label {
  color: #858585;
  font-size: 12px;
  margin-bottom: 8px;
}

.chapter-count {
  color: #4EC9B0;
  font-size: 10px;
  margin-top: 4px;
}

.foreshadow-dots {
  display: flex;
  flex-direction: column;
  gap: 4px;
  align-items: center;
}

.foreshadow-dot {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  cursor: pointer;
  transition: transform 0.2s;
}

.foreshadow-dot:hover {
  transform: scale(1.3);
}

.foreshadow-dot.active {
  background: #f0ad4e;
}

.foreshadow-dot.resolved {
  background: #5cb85c;
}

.foreshadow-dot.overdue {
  background: #d9534f;
}

.map-legend {
  display: flex;
  gap: 16px;
  margin-top: 16px;
  padding-top: 12px;
  border-top: 1px solid #3c3c3c;
  color: #858585;
  font-size: 12px;
  align-items: center;
  flex-wrap: wrap;
}

.legend-item {
  display: flex;
  align-items: center;
  gap: 4px;
}

.legend-summary {
  margin-left: auto;
  color: #4EC9B0;
}

.dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  display: inline-block;
}

.dot.active {
  background: #f0ad4e;
}

.dot.resolved {
  background: #5cb85c;
}

.dot.overdue {
  background: #d9534f;
}
</style>