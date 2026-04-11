<template>
  <div class="create-project-view">
    <div class="form-container">
      <h1>创建新项目</h1>
      
      <n-form
        ref="formRef"
        :model="formData"
        :rules="formRules"
        label-placement="top"
      >
        <n-form-item label="项目名称" path="title">
          <n-input
            v-model:value="formData.title"
            placeholder="给小说起个名字..."
            maxlength="100"
            show-count
          />
        </n-form-item>
        
        <n-form-item label="项目描述" path="description">
          <n-input
            v-model:value="formData.description"
            type="textarea"
            placeholder="简短描述你的小说（可选）"
            maxlength="500"
            show-count
            :rows="3"
          />
        </n-form-item>
        
        <n-form-item label="选择风格档案" path="style_profile_id">
          <div class="style-selector">
            <n-select
              v-model:value="formData.style_profile_id"
              :options="styleOptions"
              placeholder="选择或搜索风格档案"
              filterable
              clearable
            />
            
            <div v-if="selectedStyle" class="style-preview">
              <n-card size="small">
                <template #header>
                  {{ selectedStyle.name }}
                </template>
                <p class="style-source">来源：{{ selectedStyle.source_file || '未知' }}</p>
                <n-button text @click="viewStyleDetail">查看详情</n-button>
              </n-card>
            </div>
            
            <n-alert
              v-if="!formData.style_profile_id"
              type="info"
              title="暂不选择风格"
            >
              你可以先创建项目，稍后再关联风格档案。
            </n-alert>
          </div>
        </n-form-item>
        
        <n-form-item>
          <n-space>
            <n-button @click="handleCancel">取消</n-button>
            <n-button type="primary" @click="handleSubmit" :loading="isSubmitting">
              创建项目
            </n-button>
          </n-space>
        </n-form-item>
      </n-form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { NForm, NFormItem, NInput, NSelect, NCard, NButton, NAlert, NSpace, useMessage } from 'naive-ui';
import axios from 'axios';

const router = useRouter();
const message = useMessage();

const formRef = ref(null);
const isSubmitting = ref(false);
const styleProfiles = ref<any[]>([]);

const formData = ref({
  title: '',
  description: '',
  style_profile_id: null as string | null,
});

const formRules = {
  title: [
    { required: true, message: '请输入项目名称', trigger: 'blur' },
    { min: 2, max: 100, message: '名称长度必须在 2-100 字符之间', trigger: 'blur' },
  ],
};

const styleOptions = computed(() => {
  return styleProfiles.value.map(style => ({
    label: style.name,
    value: style.id,
  }));
});

const selectedStyle = computed(() => {
  if (!formData.value.style_profile_id) return null;
  return styleProfiles.value.find(s => s.id === formData.value.style_profile_id);
});

onMounted(async () => {
  await loadStyleProfiles();
});

async function loadStyleProfiles() {
  try {
    const response = await axios.get('/api/style-profiles');
    styleProfiles.value = response.data.data;
  } catch (error) {
    console.error('加载风格档案失败:', error);
  }
}

async function handleSubmit() {
  try {
    await (formRef.value as any)?.validate();
  } catch {
    return;
  }

  isSubmitting.value = true;

  try {
    const response = await axios.post('/api/projects', {
      title: formData.value.title,
      description: formData.value.description || null,
      style_profile_id: formData.value.style_profile_id,
    });

    message.success('项目创建成功');
    router.push(`/projects/${response.data.data.id}`);
  } catch (error: any) {
    const errorMsg = error.response?.data?.message || '创建失败';
    message.error(errorMsg);
  } finally {
    isSubmitting.value = false;
  }
}

function handleCancel() {
  router.back();
}

function viewStyleDetail() {
  if (selectedStyle.value) {
    router.push(`/styles/${selectedStyle.value.id}/report`);
  }
}
</script>

<style scoped>
.create-project-view {
  padding: 24px;
  max-width: 800px;
  margin: 0 auto;
}

.form-container {
  background: #252526;
  padding: 32px;
  border-radius: 8px;
}

.form-container h1 {
  color: #D4D4D4;
  margin-bottom: 24px;
}

.style-selector {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.style-preview {
  margin-top: 8px;
}

.style-source {
  color: #858585;
  font-size: 14px;
  margin-bottom: 8px;
}
</style>