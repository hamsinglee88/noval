<template>
  <div class="expand-result" v-if="result">
    <div class="result-header">
      <h4>扩写结果</h4>
      <n-tag type="info">{{ wordCount }} 字</n-tag>
    </div>
    <div class="result-content">{{ result }}</div>
  </div>
</template>
<script setup lang="ts">
import { computed } from 'vue'
import { NTag } from 'naive-ui'
const props = defineProps<{ result: string }>()
const wordCount = computed(() => props.result?.length || 0)
</script>
<style scoped>
.expand-result { background: #252526; border-radius: 8px; padding: 16px; margin: 8px 0; }
.result-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px; }
.result-content { color: #d4d4d4; line-height: 1.8; }
</style>