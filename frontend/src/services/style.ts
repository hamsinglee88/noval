import { http } from './api';
import type { AxiosProgressEvent } from 'axios';

export interface StyleTask {
  task_id: string;
  status: 'pending' | 'processing' | 'failed' | 'completed';
  progress: number;
  source_file_path: string;
  status_message?: string;
}

interface UploadResponse {
  success: boolean;
  data: StyleTask;
}

/**
 * 上传风格分析文件
 */
export async function uploadStyleFile(
  file: File,
  onProgress?: (event: AxiosProgressEvent) => void
): Promise<StyleTask> {
  const formData = new FormData();
  formData.append('file', file);

  const response = await http.post<UploadResponse>('/styles/analyze', formData, {
    headers: {
      'Content-Type': 'multipart/form-data',
    },
    onUploadProgress: onProgress,
  });

  return response.data.data;
}

/**
 * 获取任务状态
 */
export async function getTaskStatus(taskId: string): Promise<StyleTask> {
  const response = await http.get<UploadResponse>(`/styles/analyze/${taskId}`);
  return response.data.data;
}

/**
 * 取消任务
 */
export async function cancelTask(taskId: string): Promise<void> {
  await http.post(`/styles/analyze/${taskId}/cancel`);
}
