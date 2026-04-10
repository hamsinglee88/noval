<template>
  <div class="style-report">
    <n-card title="风格分析报告">
      <n-spin :show="loading">
        <div v-if="report" class="report-content">
          <n-descriptions bordered :column="2">
            <n-descriptions-item label="任务ID">
              {{ report.task_id }}
            </n-descriptions-item>
            <n-descriptions-item label="状态">
              <n-tag :type="statusType">{{ report.status }}</n-tag>
            </n-descriptions-item>
            <n-descriptions-item label="创建时间">
              {{ formatDate(report.created_at) }}
            </n-descriptions-item>
            <n-descriptions-item label="更新时间">
              {{ formatDate(report.updated_at) }}
            </n-descriptions-item>
          </n-descriptions>

          <n-divider>词汇特征</n-divider>
          <n-descriptions bordered :column="2" v-if="report.vocabulary_json">
            <n-descriptions-item label="平均词长">
              {{ report.vocabulary_json?.avg_word_length?.toFixed(2) || '-' }}
            </n-descriptions-item>
            <n-descriptions-item label="词汇多样性">
              {{ report.vocabulary_json?.vocabulary_diversity?.toFixed(2) || '-' }}
            </n-descriptions-item>
          </n-descriptions>

          <n-divider>句式特征</n-divider>
          <n-descriptions bordered :column="2" v-if="report.sentence_json">
            <n-descriptions-item label="平均句长">
              {{ report.sentence_json?.avg_sentence_length?.toFixed(2) || '-' }}
            </n-descriptions-item>
            <n-descriptions-item label="句式多样性">
              {{ report.sentence_json?.sentence_diversity?.toFixed(2) || '-' }}
            </n-descriptions-item>
          </n-descriptions>

          <n-divider>风格向量</n-divider>
          <div v-if="report.style_vector_json" class="vector-display">
            <n-code :code="JSON.stringify(report.style_vector_json, null, 2)" language="json" />
          </div>
          <n-empty v-else description="暂无风格向量数据" />
        </div>
        <n-empty v-else description="暂无报告数据" />
      </n-spin>
    </n-card>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import {
  NCard,
  NDescriptions,
  NDescriptionsItem,
  NTag,
  NSpin,
  NDivider,
  NEmpty,
  NCode,
  useMessage
} from 'naive-ui'

const route = useRoute()
const message = useMessage()
const loading = ref(false)
const report = ref<any>(null)

const taskId = computed(() => route.params.taskId as string)

const statusType = computed(() => {
  if (!report.value) return 'default'
  switch (report.value.status) {
    case 'completed':
      return 'success'
    case 'processing':
      return 'info'
    case 'failed':
      return 'error'
    default:
      return 'default'
  }
})

const formatDate = (dateStr: string) => {
  if (!dateStr) return '-'
  return new Date(dateStr).toLocaleString('zh-CN')
}

const fetchReport = async () => {
  if (!taskId.value) return
  
  loading.value = true
  try {
    const response = await fetch(`/api/style-analysis/${taskId.value}`)
    const data = await response.json()
    if (data.success) {
      report.value = data.data
    } else {
      message.error('获取报告失败')
    }
  } catch (error) {
    message.error('获取报告失败')
    console.error(error)
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  fetchReport()
})
</script>

<style scoped>
.style-report {
  padding: 20px;
}

.report-content {
  margin-top: 16px;
}

.vector-display {
  max-height: 300px;
  overflow: auto;
}
</style>
