<template>
  <div class="llm-config-view">
    <div class="config-header">
      <h1>LLM 路由配置</h1>
      <n-button type="primary" @click="showCreateModal = true">
        添加配置
      </n-button>
    </div>

    <div class="config-content">
      <!-- 加载状态 -->
      <div v-if="loading" class="loading-state">
        <n-spin size="large" />
      </div>

      <!-- 空状态 -->
      <n-empty 
        v-else-if="configs.length === 0"
        description="还没有 LLM 配置"
      >
        <template #extra>
          <n-button type="primary" @click="showCreateModal = true">
            添加第一个配置
          </n-button>
        </template>
      </n-empty>

      <!-- 配置列表 -->
      <div v-else class="config-list">
        <n-card 
          v-for="config in configs" 
          :key="config.id"
          class="config-card"
        >
          <template #header>
            <div class="card-header">
              <span>{{ getProviderLabel(config.provider) }} - {{ config.model }}</span>
              <n-tag v-if="config.is_default" type="success">默认</n-tag>
              <n-tag v-if="!config.is_active" type="warning">已禁用</n-tag>
            </div>
          </template>

          <n-descriptions :column="2" bordered>
            <n-descriptions-item label="提供商">
              {{ getProviderLabel(config.provider) }}
            </n-descriptions-item>
            <n-descriptions-item label="模型">
              {{ config.model }}
            </n-descriptions-item>
            <n-descriptions-item label="最大 Tokens">
              {{ config.max_tokens }}
            </n-descriptions-item>
            <n-descriptions-item label="温度">
              {{ config.temperature }}
            </n-descriptions-item>
          </n-descriptions>

          <template #footer>
            <n-space>
              <n-button size="small" @click="setDefault(config.id)" :disabled="config.is_default">
                设为默认
              </n-button>
              <n-button size="small" @click="toggleActive(config.id)">
                {{ config.is_active ? '禁用' : '启用' }}
              </n-button>
              <n-button size="small" type="error" @click="confirmDelete(config.id)">
                删除
              </n-button>
            </n-space>
          </template>
        </n-card>
      </div>
    </div>

    <!-- 创建配置对话框 -->
    <n-modal v-model:show="showCreateModal" preset="dialog" title="添加 LLM 配置">
      <n-form :model="newConfig" label-placement="top">
        <n-form-item label="提供商">
          <n-select 
            v-model:value="newConfig.provider" 
            :options="providerOptions"
          />
        </n-form-item>
        <n-form-item label="模型">
          <n-input v-model:value="newConfig.model" placeholder="如: claude-3-sonnet" />
        </n-form-item>
        <n-form-item label="API Key">
          <n-input v-model:value="newConfig.api_key" type="password" placeholder="可选" />
        </n-form-item>
        <n-form-item label="Base URL">
          <n-input v-model:value="newConfig.base_url" placeholder="可选，如: http://localhost:11434" />
        </n-form-item>
        <n-form-item label="设为默认">
          <n-switch v-model:value="newConfig.is_default" />
        </n-form-item>
      </n-form>
      <template #action>
        <n-button @click="showCreateModal = false">取消</n-button>
        <n-button type="primary" @click="createConfig" :loading="creating">创建</n-button>
      </template>
    </n-modal>

    <!-- 删除确认对话框 -->
    <n-modal v-model:show="showDeleteModal" preset="dialog" title="确认删除">
      <p>确定要删除这个 LLM 配置吗？</p>
      <template #action>
        <n-button @click="showDeleteModal = false">取消</n-button>
        <n-button type="error" @click="deleteConfig">删除</n-button>
      </template>
    </n-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { 
  NButton, NCard, NTag, NEmpty, NSpin, NSpace, NDescriptions, 
  NDescriptionsItem, NModal, NForm, NFormItem, NInput, NSelect, NSwitch, useMessage 
} from 'naive-ui'
import axios from 'axios'

interface LLMConfig {
  id: string
  provider: string
  model: string
  is_default: boolean
  is_active: boolean
  max_tokens: number
  temperature: number
}

const message = useMessage()
const loading = ref(false)
const creating = ref(false)
const configs = ref<LLMConfig[]>([])
const showCreateModal = ref(false)
const showDeleteModal = ref(false)
const deletingId = ref<string | null>(null)

const newConfig = ref({
  provider: 'claude',
  model: '',
  api_key: '',
  base_url: '',
  is_default: false
})

const providerOptions = [
  { label: 'Claude (Anthropic)', value: 'claude' },
  { label: 'OpenAI', value: 'openai' },
  { label: 'Ollama (本地)', value: 'ollama' }
]

onMounted(async () => {
  await loadConfigs()
})

async function loadConfigs() {
  loading.value = true
  try {
    const response = await axios.get('/api/llm/configs')
    configs.value = response.data.data
  } catch (error) {
    console.error('加载配置失败:', error)
    message.error('加载配置失败')
  } finally {
    loading.value = false
  }
}

function getProviderLabel(provider: string): string {
  const map: Record<string, string> = {
    claude: 'Claude',
    openai: 'OpenAI',
    ollama: 'Ollama'
  }
  return map[provider] || provider
}

async function createConfig() {
  creating.value = true
  try {
    await axios.post('/api/llm/configs', newConfig.value)
    message.success('配置创建成功')
    showCreateModal.value = false
    newConfig.value = { provider: 'claude', model: '', api_key: '', base_url: '', is_default: false }
    await loadConfigs()
  } catch (error: any) {
    message.error(error.response?.data?.message || '创建失败')
  } finally {
    creating.value = false
  }
}

async function setDefault(id: string) {
  try {
    await axios.put(`/api/llm/configs/${id}`, { is_default: true })
    message.success('已设为默认')
    await loadConfigs()
  } catch (error) {
    message.error('设置失败')
  }
}

async function toggleActive(id: string) {
  const config = configs.value.find(c => c.id === id)
  if (!config) return
  
  try {
    await axios.put(`/api/llm/configs/${id}`, { is_active: !config.is_active })
    message.success(config.is_active ? '已禁用' : '已启用')
    await loadConfigs()
  } catch (error) {
    message.error('操作失败')
  }
}

function confirmDelete(id: string) {
  deletingId.value = id
  showDeleteModal.value = true
}

async function deleteConfig() {
  if (!deletingId.value) return
  
  try {
    await axios.delete(`/api/llm/configs/${deletingId.value}`)
    message.success('配置已删除')
    showDeleteModal.value = false
    await loadConfigs()
  } catch (error) {
    message.error('删除失败')
  }
}
</script>

<style scoped>
.llm-config-view {
  padding: 24px;
  max-width: 1200px;
  margin: 0 auto;
}

.config-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
}

.config-header h1 {
  color: #d4d4d4;
  margin: 0;
}

.loading-state {
  display: flex;
  justify-content: center;
  padding: 60px 0;
}

.config-list {
  display: grid;
  gap: 16px;
}

.config-card {
  background: #252526;
}

.card-header {
  display: flex;
  align-items: center;
  gap: 12px;
}
</style>