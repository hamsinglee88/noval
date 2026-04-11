<template>
  <div class="flow-mode" v-if="active">
    <div class="flow-content">
      <slot></slot>
    </div>
    <n-button class="exit-btn" @click="$emit('exit')">退出</n-button>
  </div>
</template>
<script setup lang="ts">
import { NButton } from 'naive-ui'
defineProps<{ active: boolean }>()
defineEmits(['exit'])
</script>
<style scoped>
.flow-mode { position: fixed; inset: 0; background: #1e1e1e; z-index: 1000; display: flex; flex-direction: column; }
.flow-content { flex: 1; overflow: auto; padding: 40px; max-width: 800px; margin: 0 auto; width: 100%; }
.exit-btn { position: fixed; top: 20px; right: 20px; }
</style>