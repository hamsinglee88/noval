<template>
  <div class="save-style-profile">
    <n-card title="保存风格档案">
      <n-form ref="formRef" :model="form" :rules="rules" label-placement="top">
        <n-form-item label="风格名称" path="name">
          <n-input v-model:value="form.name" placeholder="请输入风格名称" />
        </n-form-item>
        <n-form-item label="描述（可选）" path="description">
          <n-input
            v-model:value="form.description"
            type="textarea"
            placeholder="请输入风格描述"
            :rows="4"
          />
        </n-form-item>
        <n-form-item label="来源小说" path="source_novel">
          <n-input v-model:value="form.source_novel" placeholder="请输入来源小说名称" />
        </n-form-item>
      </n-form>

      <template #footer>
        <n-space justify="end">
          <n-button @click="handleCancel">取消</n-button>
          <n-button type="primary" :loading="saving" @click="handleSave">
            保存
          </n-button>
        </n-space>
      </template>
    </n-card>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import {
  NCard,
  NForm,
  NFormItem,
  NInput,
  NButton,
  NSpace,
  useMessage,
  type FormInst,
  type FormRules
} from 'naive-ui'

const route = useRoute()
const router = useRouter()
const message = useMessage()
const formRef = ref<FormInst | null>(null)
const saving = ref(false)

const taskId = ref(route.params.taskId as string)

const form = ref({
  name: '',
  description: '',
  source_novel: ''
})

const rules: FormRules = {
  name: [
    { required: true, message: '请输入风格名称', trigger: 'blur' },
    { min: 2, max: 50, message: '名称长度在 2-50 个字符之间', trigger: 'blur' }
  ]
}

const handleCancel = () => {
  router.back()
}

const handleSave = async () => {
  try {
    await formRef.value?.validate()
  } catch {
    return
  }

  saving.value = true
  try {
    const response = await fetch('/api/styles/save', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        task_id: taskId.value,
        name: form.value.name,
        description: form.value.description,
        source_novel: form.value.source_novel
      })
    })

    const data = await response.json()
    if (data.success) {
      message.success('风格档案保存成功')
      router.push('/style-library')
    } else {
      message.error(data.message || '保存失败')
    }
  } catch (error) {
    message.error('保存失败')
    console.error(error)
  } finally {
    saving.value = false
  }
}

onMounted(() => {
  // 如果没有 taskId，返回上一页
  if (!taskId.value) {
    message.warning('无效的任务ID')
    router.back()
  }
})
</script>

<style scoped>
.save-style-profile {
  padding: 20px;
  max-width: 600px;
  margin: 0 auto;
}
</style>