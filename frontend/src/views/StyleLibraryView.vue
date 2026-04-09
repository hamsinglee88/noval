<script setup lang="ts">
import { NLayout, NLayoutContent } from 'naive-ui';
import StyleUploader from '@/components/style/StyleUploader.vue';
</script>

<template>
  <n-layout>
    <n-layout-content content-style="padding: 24px;">
      <div class="style-library-header">
        <h1>风格管理</h1>
        <p class="subtitle">上传您的代表作用于风格分析，系统将学习您的写作风格</p>
      </div>

      <div class="style-library-content">
        <StyleUploader />
      </div>
    </n-layout-content>
  </n-layout>
</template>

<style scoped>
.style-library-header {
  margin-bottom: 24px;
}

.style-library-header h1 {
  font-size: 24px;
  font-weight: 600;
  margin-bottom: 8px;
}

.subtitle {
  color: var(--n-color-text-secondary);
  font-size: 14px;
}

.style-library-content {
  max-width: 800px;
  margin: 0 auto;
}
</style>
