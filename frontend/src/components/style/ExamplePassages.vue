<template>
  <div class="example-passages">
    <h3>示例段落</h3>
    
    <div v-if="passages.length === 0" class="empty-state">
      <n-empty description="暂无示例段落" />
    </div>
    
    <n-carousel 
      v-else-if="passages.length > 1" 
      show-dots
      :dot-type="'line'"
      :autoplay="false"
    >
      <n-card 
        v-for="(passage, index) in passages" 
        :key="index"
        class="passage-card"
      >
        <p class="passage-text">{{ passage.text }}</p>
        <div class="passage-annotations">
          <n-tag 
            v-for="(annotation, i) in passage.annotations" 
            :key="i"
            size="small"
            type="info"
          >
            {{ annotation }}
          </n-tag>
        </div>
      </n-card>
    </n-carousel>
    
    <div v-else class="passage-list">
      <n-card 
        v-for="(passage, index) in passages" 
        :key="index" 
        class="passage-card"
      >
        <p class="passage-text">{{ passage.text }}</p>
        <div class="passage-annotations">
          <n-tag 
            v-for="(annotation, i) in passage.annotations" 
            :key="i" 
            size="small"
            type="info"
          >
            {{ annotation }}
          </n-tag>
        </div>
      </n-card>
    </div>
  </div>
</template>

<script setup lang="ts">
import { NCard, NTag, NCarousel, NEmpty } from 'naive-ui';

interface Passage {
  text: string;
  annotations: string[];
}

defineProps<{
  passages: Passage[];
}>();
</script>

<style scoped>
.example-passages {
  width: 100%;
}

.example-passages h3 {
  margin-bottom: 16px;
  color: #D4D4D4;
}

.passage-card {
  margin-bottom: 16px;
}

.passage-text {
  font-size: 15px;
  line-height: 1.8;
  color: #D4D4D4;
  margin-bottom: 16px;
  white-space: pre-wrap;
}

.passage-annotations {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.empty-state {
  padding: 40px 0;
}
</style>