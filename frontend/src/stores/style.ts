import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { uploadStyleFile, getTaskStatus, cancelTask } from '@/services/style';

export interface StyleTask {
  task_id: string;
  status: 'pending' | 'processing' | 'failed' | 'completed';
  progress: number;
  source_file_path: string;
  status_message?: string;
}

interface UploadState {
  isUploading: boolean;
  progress: number;
  currentTask: StyleTask | null;
  error: string | null;
}

export const useStyleStore = defineStore('style', () => {
  const uploadState = ref<UploadState>({
    isUploading: false,
    progress: 0,
    currentTask: null,
    error: null,
  });

  const isUploading = computed(() => uploadState.value.isUploading);
  const uploadProgress = computed(() => uploadState.value.progress);
  const currentTask = computed(() => uploadState.value.currentTask);
  const uploadError = computed(() => uploadState.value.error);

  async function uploadFile(file: File): Promise<StyleTask> {
    uploadState.value.isUploading = true;
    uploadState.value.progress = 0;
    uploadState.value.error = null;

    try {
      const task = await uploadStyleFile(file, (progressEvent) => {
        if (progressEvent.total) {
          uploadState.value.progress = (progressEvent.loaded / progressEvent.total) * 100;
        }
      });

      uploadState.value.currentTask = task;
      uploadState.value.progress = 100;

      return task;
    } catch (error) {
      uploadState.value.error = error instanceof Error ? error.message : '上传失败';
      throw error;
    } finally {
      uploadState.value.isUploading = false;
    }
  }

  async function refreshTaskStatus(taskId: string) {
    try {
      const task = await getTaskStatus(taskId);
      uploadState.value.currentTask = task;
      return task;
    } catch (error) {
      uploadState.value.error = error instanceof Error ? error.message : '获取状态失败';
      throw error;
    }
  }

  async function cancelCurrentTask(taskId: string) {
    try {
      await cancelTask(taskId);
      uploadState.value.currentTask = null;
      uploadState.value.progress = 0;
    } catch (error) {
      uploadState.value.error = error instanceof Error ? error.message : '取消失败';
      throw error;
    }
  }

  function resetUpload() {
    uploadState.value = {
      isUploading: false,
      progress: 0,
      currentTask: null,
      error: null,
    };
  }

  return {
    isUploading,
    uploadProgress,
    currentTask,
    uploadError,
    uploadFile,
    refreshTaskStatus,
    cancelCurrentTask,
    resetUpload,
  };
});
