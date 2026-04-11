<template>
  <div class="foreshadow-map">
    <h3>伏笔地图</h3>
    <div class="map-container">
      <div v-for="(chapter, index) in chapters" :key="index" class="chapter-node">
        <div class="chapter-label">第{{ index + 1 }}章</div>
        <div class="foreshadow-dots">
          <div 
            v-for="f in getChapterForeshadows(chapter.id)" 
            :key="f.id"
            class="foreshadow-dot"
            :class="f.status.toLowerCase()"
            :title="f.content"
          ></div>
        </div>
      </div>
    </div>
    <div class="legend">
      <span class="dot active"></span> 活跃
      <span class="dot resolved"></span> 已回收
      <span class="dot overdue"></span> 逾期
    </div>
  </div>
</template>

<script setup lang="ts">
interface Chapter { id: string }
interface Foreshadow { id: string; chapter_id: string; status: string; content: string }

const props = defineProps<{
  chapters: Chapter[]
  foreshadows: Foreshadow[]
}>()

function getChapterForeshadows(chapterId: string) {
  return props.foreshadows.filter(f => f.chapter_id === chapterId)
}
</script>

<style scoped>
.foreshadow-map { background: #252526; border-radius: 8px; padding: 16px; }
.map-container { display: flex; gap: 8px; overflow-x: auto; padding: 16px 0; }
.chapter-node { min-width: 60px; text-align: center; }
.chapter-label { color: #858585; font-size: 12px; margin-bottom: 8px; }
.foreshadow-dots { display: flex; flex-direction: column; gap: 4px; align-items: center; }
.foreshadow-dot { width: 12px; height: 12px; border-radius: 50%; cursor: pointer; }
.foreshadow-dot.active { background: #f0ad4e; }
.foreshadow-dot.resolved { background: #5cb85c; }
.foreshadow-dot.overdue { background: #d9534f; }
.legend { display: flex; gap: 16px; margin-top: 16px; color: #858585; font-size: 12px; }
.dot { width: 10px; height: 10px; border-radius: 50%; display: inline-block; margin-right: 4px; }
.dot.active { background: #f0ad4e; }
.dot.resolved { background: #5cb85c; }
.dot.overdue { background: #d9534f; }
</style>