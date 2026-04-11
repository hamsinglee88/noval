<template>
  <div class="project-list-view">
    <div class="list-header">
      <h1>我的项目</h1>
      <n-button type="primary" @click="handleCreate">
        创建新项目
      </n-button>
    </div>
    
    <div class="search-bar">
      <n-input
        v-model:value="searchQuery"
        placeholder="搜索项目名称..."
        clearable
        @input="handleSearch"
      />
    </div>
    
    <div class="project-grid" v-if="projects.length > 0">
      <n-card
        v-for="project in projects"
        :key="project.id"
        class="project-card"
        @click="enterProject(project.id)"
      >
        <template #header>
          <div class="card-header">
            <h3>{{ project.title }}</h3>
            <n-tag v-if="project.style_name" size="small" type="info">
              {{ project.style_name }}
            </n-tag>
          </div>
        </template>
        
        <p class="project-description">{{ project.description || '暂无描述' }}</p>
        
        <div class="card-footer">
          <span class="created-at">{{ formatDate(project.created_at) }}</span>
          <span class="chapter-count">{{ project.chapter_count }} 章</span>
        </div>
      </n-card>
    </div>
    
    <!-- 空状态 -->
    <n-empty
      v-if="projects.length === 0 && !searchQuery"
      description="还没有项目，创建第一个小说项目开始创作吧"
    >
      <template #extra>
        <n-button type="primary" @click="handleCreate">
          创建第一个项目
        </n-button>
      </template>
    </n-empty>
    
    <!-- 搜索无结果 -->
    <n-empty
      v-if="projects.length === 0 && searchQuery"
      description="未找到匹配的项目"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { NCard, NButton, NTag, NInput, NEmpty, useMessage } from 'naive-ui';
import axios from 'axios';

interface Project {
  id: string;
  title: string;
  description: string | null;
  style_name: string | null;
  chapter_count: number;
  created_at: string;
}

const router = useRouter();
const message = useMessage();

const projects = ref<Project[]>([]);
const searchQuery = ref('');
const loading = ref(false);

onMounted(async () => {
  await loadProjects();
});

async function loadProjects() {
  loading.value = true;
  try {
    const params: any = {};
    if (searchQuery.value) {
      params.search = searchQuery.value;
    }
    const response = await axios.get('/api/projects', { params });
    projects.value = response.data.data;
  } catch (error) {
    console.error('加载项目失败:', error);
    message.error('加载项目失败');
  } finally {
    loading.value = false;
  }
}

function formatDate(dateStr: string): string {
  const date = new Date(dateStr);
  return date.toLocaleString('zh-CN');
}

function handleCreate() {
  router.push('/projects/create');
}

function enterProject(id: string) {
  router.push(`/projects/${id}`);
}

let searchTimer: any = null;
function handleSearch() {
  clearTimeout(searchTimer);
  searchTimer = setTimeout(() => {
    loadProjects();
  }, 300);
}
</script>

<style scoped>
.project-list-view {
  padding: 24px;
  max-width: 1400px;
  margin: 0 auto;
}

.list-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
}

.list-header h1 {
  color: #D4D4D4;
  margin: 0;
}

.search-bar {
  margin-bottom: 24px;
  max-width: 400px;
}

.project-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  gap: 24px;
}

.project-card {
  cursor: pointer;
  transition: transform 0.2s;
}

.project-card:hover {
  transform: translateY(-4px);
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.card-header h3 {
  margin: 0;
  color: #D4D4D4;
}

.project-description {
  color: #858585;
  font-size: 14px;
  line-height: 1.6;
  max-height: 60px;
  overflow: hidden;
  text-overflow: ellipsis;
}

.card-footer {
  display: flex;
  justify-content: space-between;
  color: #858585;
  font-size: 12px;
  margin-top: 16px;
}
</style>