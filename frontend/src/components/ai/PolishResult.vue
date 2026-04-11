<template>
  <div class="polish-result" v-if="result">
    <div class="result-header">
      <h4>润色结果</h4>
      <n-button-group>
        <n-button size="small" @click="$emit('accept')">接受</n-button>
        <n-button size="small" @click="$emit('reject')">拒绝</n-button>
      </n-button-group>
    </div>
    <div class="result-content">{{ result }}</div>
  </div>
</template>
<script setup lang="ts">
import { NButton, NButtonGroup } from 'naive-ui'
defineProps<{ result: string }>()
defineEmits(['accept', 'reject'])
</script>
<style scoped>
.polish-result { background: #252526; border-radius: 8px; padding: 16px; margin: 8px 0; }
.result-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px; }
.result-content { color: #4EC9B0; line-height: 1.6; }
</style>
