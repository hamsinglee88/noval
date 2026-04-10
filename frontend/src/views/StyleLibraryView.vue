<template>
  <div class="style-library-view">
    <div class="library-header">
      <h1>风格档案库</h1>
      <div class="library-actions">
        <n-button @click="goToUpload">
          上传参考小说
        </n-button>
      </div>
    </div>
    
    <div class="library-content">
      <!-- 加载状态 -->
      <div v-if="loading" class="loading-state">
        <n-spin size="large" />
      </div>
      
      <!-- 空状态 -->
      <n-empty 
        v-else-if="styleProfiles.length === 0"
        description="还没有风格档案，上传参考小说来创建第一个风格吧"
      >
        <template #extra>
          <n-button type="primary" @click="goToUpload">
            上传参考小说
          </n-button>
        </template>
      </n-empty>
      
      <!-- 风格列表 -->
      <n-data-table
        v-else
        :columns="columns"
        :data="styleProfiles"
        :bordered="false"
      />
    </div>
    
    <!-- 删除确认对话框 -->
    <n-modal
      v-model:show="showDeleteModal"
      preset="dialog"
      title="确认删除"
      content="确定要删除这个风格档案吗？删除后不可恢复。"
      positive-text="确认删除"
      negative-text="取消"
      @positive-click="confirmDelete"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, h } from 'vue';
import { useRouter } from 'vue-router';
import { NButton, NEmpty, NDataTable, NSpin, NModal, NTag, useMessage } from 'naive-ui';
import axios from 'axios';

interface StyleProfile {
  id: string;
  name: string;
  description: string | null;
  source_file: string | null;
  total_chars: number | null;
  created_at: string;
}

const router = useRouter();
const message = useMessage();

const loading = ref(true);
const styleProfiles = ref<StyleProfile[]>([]);
const showDeleteModal = ref(false);
const deletingId = ref<string | null>(null);

const columns: any[] = [
  {
    title: '名称',
    key: 'name',
    sorter: true,
    render: (row: StyleProfile) => 
      h('a', { 
        style: 'color: #4EC9B0; cursor: pointer;',
        onClick: () => viewDetail(row.id) 
      }, row.name)
  },
  {
    title: '描述',
    key: 'description',
    ellipsis: { tooltip: true },
    render: (row: StyleProfile) => row.description || '-'
  },
  {
    title: '来源文件',
    key: 'source_file',
    ellipsis: { tooltip: true },
    render: (row: StyleProfile) => {
      if (!row.source_file) return '-';
      const parts = row.source_file.split('/');
      return parts[parts.length - 1];
    }
  },
  {
    title: '创建时间',
    key: 'created_at',
    sorter: true,
    render: (row: StyleProfile) => formatDate(row.created_at)
  },
  {
    title: '操作',
    key: 'actions',
    width: 200,
    render: (row: StyleProfile) => 
      h('div', { style: 'display: flex; gap: 8px;' }, [
        h(NButton, { 
          size: 'small',
          type: 'primary',
          onClick: () => viewDetail(row.id) 
        }, { default: () => '查看' }),
        h(NButton, { 
          size: 'small',
          onClick: () => exportStyle(row.id) 
        }, { default: () => '导出' }),
        h(NButton, { 
          size: 'small',
          type: 'error',
          onClick: () => showDeleteConfirm(row.id) 
        }, { default: () => '删除' }),
      ])
  },
];

onMounted(async () => {
  await loadStyleProfiles();
});

async function loadStyleProfiles() {
  loading.value = true;
  try {
    const response = await axios.get('/api/style-profiles');
    styleProfiles.value = response.data.data;
  } catch (error) {
    console.error('加载风格档案失败:', error);
    message.error('加载风格档案失败');
  } finally {
    loading.value = false;
  }
}

function formatDate(dateStr: string): string {
  const date = new Date(dateStr);
  return date.toLocaleString('zh-CN');
}

function viewDetail(id: string) {
  router.push(`/styles/${id}/report`);
}

function exportStyle(id: string) {
  window.open(`/api/style-profiles/${id}/export`, '_blank');
}

function showDeleteConfirm(id: string) {
  deletingId.value = id;
  showDeleteModal.value = true;
}

async function confirmDelete() {
  if (!deletingId.value) return;
  
  try {
    await axios.delete(`/api/style-profiles/${deletingId.value}`);
    message.success('风格档案已删除');
    await loadStyleProfiles();
  } catch (error: any) {
    const errorMsg = error.response?.data?.message || '删除失败';
    message.error(errorMsg);
  } finally {
    showDeleteModal.value = false;
    deletingId.value = null;
  }
}

function goToUpload() {
  router.push('/style-profiles/onboarding');
}
</script>

<style scoped>
.style-library-view {
  padding: 24px;
  max-width: 1200px;
  margin: 0 auto;
}

.library-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
}

.library-header h1 {
  color: #D4D4D4;
  margin: 0;
}

.library-content {
  background: #252526;
  padding: 24px;
  border-radius: 8px;
}

.loading-state {
  display: flex;
  justify-content: center;
  padding: 40px 0;
}
</style>