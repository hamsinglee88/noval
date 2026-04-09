<script setup lang="ts">
import { computed, ref } from 'vue';
import {
  NAlert,
  NButton,
  NCard,
  NDescriptions,
  NDescriptionsItem,
  NProgress,
  NSpace,
  NText,
  useMessage,
} from 'naive-ui';
import { useStyleStore } from '@/stores/style';

const message = useMessage();
const styleStore = useStyleStore();

const fileInputRef = ref<HTMLInputElement | null>(null);

const isUploading = computed(() => styleStore.isUploading);
const progress = computed(() => styleStore.uploadProgress);
const currentTask = computed(() => styleStore.currentTask);
const error = computed(() => styleStore.uploadError);

function triggerFileSelect() {
  fileInputRef.value?.click();
}

async function handleFileChange(event: Event) {
  const target = event.target as HTMLInputElement;
  const file = target.files?.[0];
  if (!file) return;

  // 验证文件类型
  const validTypes = ['text/plain', 'application/epub+zip'];
  const validExtensions = ['.txt', '.epub'];
  const hasValidExtension = validExtensions.some(ext => file.name.toLowerCase().endsWith(ext));

  if (!hasValidExtension) {
    message.error('仅支持 TXT 和 EPUB 文件格式');
    target.value = '';
    return;
  }

  // 验证文件大小（10MB）
  const maxSize = 10 * 1024 * 1024;
  if (file.size > maxSize) {
    message.error(`文件大小超过 10MB 限制`);
    target.value = '';
    return;
  }

  try {
    await styleStore.uploadFile(file);
    message.success('上传成功，开始分析...');
  } catch {
    // 错误消息已在 store 中处理
  } finally {
    target.value = '';
  }
}

async function handleCancel() {
  if (!currentTask.value) return;

  try {
    await styleStore.cancelCurrentTask(currentTask.value.task_id);
    message.success('已取消分析任务');
  } catch {
    // 错误消息已在 store 中处理
  }
}

function handleRetry() {
  styleStore.resetUpload();
  triggerFileSelect();
}

const statusLabels: Record<string, string> = {
  pending: '等待中',
  processing: '分析中',
  failed: '失败',
  completed: '已完成',
};

const statusTypes: Record<string, 'info' | 'error' | 'success'> = {
  pending: 'info',
  processing: 'info',
  failed: 'error',
  completed: 'success',
};
</script>

<template>
  <n-space vertical size="large">
    <!-- 空状态 -->
    <n-card
      v-if="!currentTask && !isUploading"
      class="upload-empty-state"
      :bordered="false"
    >
      <div class="empty-content">
        <n-text depth="3" style="font-size: 48px; font-weight: bold;">
          📚
        </n-text>
        <n-h3 style="margin-top: 16px;">
          创建风格档案
        </n-h3>
        <n-text depth="3">
          上传您的代表作用于风格分析，系统将学习您的写作风格
        </n-text>

        <n-space vertical size="large" style="margin-top: 24px; max-width: 300px; margin-left: auto; margin-right: auto;">
          <div>
            <n-button
              type="primary"
              size="large"
              block
              @click="triggerFileSelect"
            >
              上传参考小说
            </n-button>
            <input
              ref="fileInputRef"
              type="file"
              accept=".txt,.epub"
              style="display: none;"
              @change="handleFileChange"
            />
          </div>

          <n-alert type="info" title="支持格式">
            <template #icon>
              ℹ️
            </template>
            仅支持 TXT 和 EPUB 文件格式，文件大小不超过 10MB
          </n-alert>
        </n-space>
      </div>
    </n-card>

    <!-- 上传中状态 -->
    <n-card
      v-else-if="isUploading"
      :bordered="false"
    >
      <n-space vertical size="large">
        <n-text>正在上传参考小说...</n-text>
        <n-progress
          type="line"
          :percentage="Math.round(progress)"
          status="info"
        />
        <n-space justify="center">
          <n-button @click="styleStore.resetUpload()">
            取消上传
          </n-button>
        </n-space>
      </n-space>
    </n-card>

    <!-- 分析中状态 -->
    <n-card
      v-else-if="currentTask && currentTask.status === 'processing'"
      :bordered="false"
    >
      <n-space vertical size="large">
        <n-alert :type="statusTypes[currentTask.status]" :title="statusLabels[currentTask.status]">
          {{ currentTask.status_message || '正在分析...' }}
        </n-alert>

        <n-progress
          type="line"
          :percentage="Math.round(currentTask.progress * 100)"
          status="info"
        />

        <n-descriptions bordered>
          <n-descriptions-item label="任务 ID">
            {{ currentTask.task_id }}
          </n-descriptions-item>
          <n-descriptions-item label="文件路径">
            {{ currentTask.source_file_path }}
          </n-descriptions-item>
        </n-descriptions>

        <n-space justify="center">
          <n-button @click="handleCancel">
            取消分析
          </n-button>
        </n-space>
      </n-space>
    </n-card>

    <!-- 错误状态 -->
    <n-card
      v-else-if="currentTask && currentTask.status === 'failed'"
      :bordered="false"
    >
      <n-space vertical size="large">
        <n-alert type="error" title="分析失败">
          {{ currentTask.status_message || error || '未知错误' }}
        </n-alert>

        <n-space justify="center">
          <n-button type="primary" @click="handleRetry">
            重试
          </n-button>
          <n-button @click="styleStore.resetUpload()">
            关闭
          </n-button>
        </n-space>
      </n-space>
    </n-card>
  </n-space>
</template>

<style scoped>
.upload-empty-state {
  text-align: center;
  padding: 48px 24px;
}

.empty-content {
  display: flex;
  flex-direction: column;
  align-items: center;
}
</style>
