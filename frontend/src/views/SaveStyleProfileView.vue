<template>
  <div class="save-style-profile-view">
    <n-modal
      v-model:show="showModal"
      :close-on-esc="false"
      preset="dialog"
      title="保存风格档案"
    >
      <n-form
        ref="formRef"
        :model="formData"
        :rules="formRules"
        label-placement="top"
      >
        <n-form-item label="风格名称" path="name">
          <n-input
            v-model:value="formData.name"
            placeholder="如：金庸武侠风格"
          />
        </n-form-item>
        
        <n-form-item label="描述（可选）" path="description">
          <n-input
            v-model:value="formData.description"
            type="textarea"
            placeholder="简短描述这个风格的特点"
          />
        </n-form-item>
        
        <n-descriptions bordered :column="1">
          <n-descriptions-item label="来源文件">
            {{ sourceFile }}
          </n-descriptions-item>
          <n-descriptions-item label="总字数">
            {{ formatNumber(totalChars) }}
          </n-descriptions-item>
        </n-descriptions>
      </n-form>
      
      <template #action>
        <n-button @click="handleCancel">取消</n-button>
        <n-button type="primary" @click="handleSubmit" :loading="isSubmitting">
          确认保存
        </n-button>
      </template>
    </n-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { NModal, NForm, NFormItem, NInput, NButton, NDescriptions, NDescriptionsItem, useMessage } from 'naive-ui';
import axios from 'axios';

const route = useRoute();
const router = useRouter();
const message = useMessage();

const taskId = route.params.taskId as string;
const showModal = ref(true);
const formRef = ref(null);
const isSubmitting = ref(false);
const sourceFile = ref('');
const totalChars = ref(0);

const formData = ref({
  name: '',
  description: '',
});

const formRules = {
  name: [
    { required: true, message: '请输入风格名称', trigger: 'blur' },
    { min: 2, max: 50, message: '名称长度必须在 2-50 字符之间', trigger: 'blur' },
  ],
};

onMounted(async () => {
  await loadTaskInfo();
});

async function loadTaskInfo() {
  try {
    const response = await axios.get(`/api/styles/analyze/${taskId}`);
    const data = response.data.data;
    sourceFile.value = data.source_file_path || '未知';
    totalChars.value = data.total_chars || 0;
  } catch (error) {
    console.error('加载任务信息失败:', error);
  }
}

function formatNumber(num: number): string {
  return num.toLocaleString();
}

async function handleSubmit() {
  try {
    await (formRef.value as any)?.validate();
  } catch {
    return;
  }

  isSubmitting.value = true;

  try {
    await axios.post('/api/style-profiles/save', {
      task_id: taskId,
      name: formData.value.name,
      description: formData.value.description || null,
    });

    message.success('风格档案保存成功');
    showModal.value = false;
    router.push('/style-library');
  } catch (error: any) {
    const errorMsg = error.response?.data?.message || '保存失败';
    message.error(errorMsg);
  } finally {
    isSubmitting.value = false;
  }
}

function handleCancel() {
  showModal.value = false;
  router.back();
}
</script>

<style scoped>
.save-style-profile-view {
  /* Modal 样式由 Naive UI 处理 */
}
</style>