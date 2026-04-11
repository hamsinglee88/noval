<template>
  <div class="project-detail-view">
    <!-- 加载状态 -->
    <div v-if="loading" class="loading-state">
      <n-spin size="large" />
    </div>
    
    <!-- 错误状态 -->
    <div v-else-if="error" class="error-state">
      <n-result status="error" title="加载失败" :description="error">
        <template #footer>
          <n-button @click="loadProject">重试</n-button>
        </template>
      </n-result>
    </div>
    
    <!-- 项目详情 -->
    <template v-else-if="project">
      <div class="project-header">
        <div class="header-left">
          <h1>{{ project.title }}</h1>
          <p v-if="project.description" class="project-description">{{ project.description }}</p>
        </div>
        <div class="header-right">
          <n-button @click="handleEdit">编辑</n-button>
          <n-button type="error" @click="showDeleteConfirm">删除</n-button>
        </div>
      </div>
      
      <div class="project-content">
        <!-- 风格信息 -->
        <div class="style-section" v-if="project.style_name">
          <h2>关联风格</h2>
          <n-card>
            <div class="style-info">
              <span class="style-name">{{ project.style_name }}</span>
              <n-button text @click="viewStyleDetail">查看详情</n-button>
            </div>
          </n-card>
        </div>
        
        <!-- 章节列表 -->
        <div class="chapters-section">
          <div class="section-header">
            <h2>章节列表</h2>
            <n-button type="primary" size="small" @click="createChapter">
              新建章节
            </n-button>
          </div>
          
          <n-empty v-if="project.chapter_count === 0" description="还没有章节">
            <template #extra>
              <n-button type="primary" @click="createChapter">
                创建第一章
              </n-button>
            </template>
          </n-empty>
          
          <div v-else class="chapter-list">
            <p class="muted">共 {{ project.chapter_count }} 章</p>
            <!-- 章节列表将在后续 Story 实现 -->
          </div>
        </div>
      </div>
    </template>
    
    <!-- 删除确认对话框 -->
    <n-modal
      v-model:show="showDeleteModal"
      preset="dialog"
      title="确认删除"
      content="确定要删除这个项目吗？所有章节内容将被永久删除，不可恢复。"
      positive-text="确认删除"
      negative-text="取消"
      @positive-click="confirmDelete"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { NButton, NSpin, NResult, NCard, NEmpty, NModal, useMessage } from 'naive-ui';
import axios from 'axios';

const route = useRoute();
const router = useRouter();
const message = useMessage();

const projectId = route.params.id as string;

const loading = ref(true);
const error = ref<string | null>(null);
const project = ref<any>(null);
const showDeleteModal = ref(false);

onMounted(async () => {
  await loadProject();
});

async function loadProject() {
  loading.value = true;
  error.value = null;
  
  try {
    const response = await axios.get(`/api/projects/${projectId}`);
    project.value = response.data.data;
  } catch (err: any) {
    console.error('加载项目失败:', err);
    error.value = err.response?.data?.message || '加载项目失败';
  } finally {
    loading.value = false;
  }
}

function handleEdit() {
  // 编辑功能可以在后续实现
  message.info('编辑功能即将上线');
}

function showDeleteConfirm() {
  showDeleteModal.value = true;
}

async function confirmDelete() {
  try {
    await axios.delete(`/api/projects/${projectId}`);
    message.success('项目已删除');
    router.push('/projects');
  } catch (error: any) {
    const errorMsg = error.response?.data?.message || '删除失败';
    message.error(errorMsg);
  }
}

function viewStyleDetail() {
  if (project.value?.style_profile_id) {
    router.push(`/styles/${project.value.style_profile_id}/report`);
  }
}

function createChapter() {
  // 章节创建功能将在后续 Story 实现
  message.info('章节创建功能即将上线');
}
</script>

<style scoped>
.project-detail-view {
  padding: 24px;
  max-width: 1200px;
  margin: 0 auto;
}

.loading-state,
.error-state {
  display: flex;
  justify-content: center;
  padding: 100px 0;
}

.project-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 32px;
}

.header-left h1 {
  color: #D4D4D4;
  margin: 0 0 8px 0;
}

.project-description {
  color: #858585;
  margin: 0;
}

.header-right {
  display: flex;
  gap: 8px;
}

.project-content {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.style-section,
.chapters-section {
  background: #252526;
  padding: 24px;
  border-radius: 8px;
}

.style-section h2,
.chapters-section h2 {
  color: #D4D4D4;
  font-size: 18px;
  margin: 0 0 16px 0;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.section-header h2 {
  margin: 0;
}

.style-info {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.style-name {
  color: #4EC9B0;
  font-weight: 500;
}

.chapter-list {
  padding: 16px 0;
}

.muted {
  color: #858585;
}
</style>