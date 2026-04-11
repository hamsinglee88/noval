<template>
  <div class="status-bar">
    <span>{{ projectName || 'Novel Agent' }}</span>
    <span>字数: {{ wordCount || 0 }}</span>
    <span>{{ saveStatus || '已保存' }}</span>
    <span>{{ currentTime }}</span>
  </div>
</template>
<script setup lang="ts">
import { ref, onMounted } from 'vue'
defineProps<{ projectName?: string; wordCount?: number; saveStatus?: string }>()
const currentTime = ref('')
onMounted(() => {
  const update = () => { currentTime.value = new Date().toLocaleTimeString('zh-CN') }
  update(); setInterval(update, 1000)
})
</script>
<style scoped>
.status-bar { display: flex; gap: 16px; padding: 4px 8px; background: #007acc; color: #fff; font-size: 12px; }
</style>