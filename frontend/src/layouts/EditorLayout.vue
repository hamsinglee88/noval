<template>
  <div class="editor-layout">
    <!-- 侧边栏 -->
    <aside class="sidebar" :class="{ collapsed: sidebarCollapsed }">
      <div class="sidebar-header">
        <n-button text @click="toggleSidebar">
          <span class="icon">{{ sidebarCollapsed ? '→' : '←' }}</span>
        </n-button>
      </div>
      
      <div class="sidebar-content" v-if="!sidebarCollapsed">
        <div class="sidebar-section">
          <h4>项目浏览器</h4>
          <n-tree
            :data="projectTree"
            block-line
            selectable
            @update:selected-keys="handleSelectNode"
          />
        </div>
        
        <div class="sidebar-section">
          <h4>大纲</h4>
          <div class="outline-list">
            <div v-for="chapter in chapters" :key="chapter.id" class="outline-item">
              {{ chapter.title }}
            </div>
          </div>
        </div>
      </div>
    </aside>

    <!-- 主编辑区 -->
    <main class="main-content">
      <!-- 标签页栏 -->
      <div class="tabs-bar">
        <div 
          v-for="tab in openTabs" 
          :key="tab.id"
          class="tab"
          :class="{ active: activeTabId === tab.id }"
          @click="setActiveTab(tab.id)"
        >
          <span class="tab-title">{{ tab.title }}</span>
          <n-button text size="tiny" @click.stop="closeTab(tab.id)">×</n-button>
        </div>
      </div>

      <!-- 编辑器区域 -->
      <div class="editor-area">
        <slot></slot>
      </div>
    </main>

    <!-- 右侧面板 -->
    <aside class="right-panel" :class="{ collapsed: rightPanelCollapsed }">
      <div class="panel-header">
        <n-button text @click="toggleRightPanel">
          <span class="icon">{{ rightPanelCollapsed ? '←' : '→' }}</span>
        </n-button>
      </div>
      
      <div class="panel-content" v-if="!rightPanelCollapsed">
        <n-tabs type="line">
          <n-tab-pane name="info" tab="信息">
            <div class="info-content">
              <p>字数: {{ wordCount }}</p>
              <p>字符: {{ charCount }}</p>
              <p>段落: {{ paragraphCount }}</p>
            </div>
          </n-tab-pane>
          <n-tab-pane name="ai" tab="AI 助手">
            <div class="ai-content">
              <n-input type="textarea" placeholder="输入指令..." :rows="3" />
              <n-button type="primary" block style="margin-top: 8px;">
                生成
              </n-button>
            </div>
          </n-tab-pane>
        </n-tabs>
      </div>
    </aside>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { NButton, NTree, NTabs, NTabPane, NInput } from 'naive-ui'

interface Tab {
  id: string
  title: string
  content?: string
}

interface Chapter {
  id: string
  title: string
}

const sidebarCollapsed = ref(false)
const rightPanelCollapsed = ref(false)
const activeTabId = ref<string | null>(null)
const openTabs = ref<Tab[]>([])
const chapters = ref<Chapter[]>([])
const wordCount = ref(0)
const charCount = ref(0)
const paragraphCount = ref(0)

const projectTree = computed(() => [
  {
    key: 'novel',
    label: '我的小说',
    children: chapters.value.map(ch => ({
      key: ch.id,
      label: ch.title
    }))
  }
])

function toggleSidebar() {
  sidebarCollapsed.value = !sidebarCollapsed.value
}

function toggleRightPanel() {
  rightPanelCollapsed.value = !rightPanelCollapsed.value
}

function setActiveTab(id: string) {
  activeTabId.value = id
}

function closeTab(id: string) {
  const index = openTabs.value.findIndex(t => t.id === id)
  if (index > -1) {
    openTabs.value.splice(index, 1)
    if (activeTabId.value === id) {
      activeTabId.value = openTabs.value[0]?.id || null
    }
  }
}

function handleSelectNode(keys: string[]) {
  console.log('Selected:', keys)
}
</script>

<style scoped>
.editor-layout {
  display: flex;
  height: 100vh;
  background: #1e1e1e;
  color: #d4d4d4;
}

.sidebar {
  width: 250px;
  background: #252526;
  border-right: 1px solid #3c3c3c;
  display: flex;
  flex-direction: column;
  transition: width 0.2s;
}

.sidebar.collapsed {
  width: 40px;
}

.sidebar-header,
.panel-header {
  padding: 8px;
  border-bottom: 1px solid #3c3c3c;
  display: flex;
  justify-content: flex-end;
}

.sidebar-content,
.panel-content {
  flex: 1;
  overflow-y: auto;
  padding: 8px;
}

.sidebar-section {
  margin-bottom: 16px;
}

.sidebar-section h4 {
  color: #858585;
  font-size: 11px;
  text-transform: uppercase;
  margin-bottom: 8px;
}

.outline-list {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.outline-item {
  padding: 4px 8px;
  border-radius: 4px;
  cursor: pointer;
}

.outline-item:hover {
  background: #2a2d2e;
}

.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-width: 0;
}

.tabs-bar {
  display: flex;
  background: #252526;
  border-bottom: 1px solid #3c3c3c;
  min-height: 35px;
}

.tab {
  display: flex;
  align-items: center;
  padding: 8px 16px;
  border-right: 1px solid #3c3c3c;
  cursor: pointer;
  background: #2d2d2d;
}

.tab.active {
  background: #1e1e1e;
  border-bottom: 1px solid #1e1e1e;
  margin-bottom: -1px;
}

.tab-title {
  margin-right: 8px;
}

.editor-area {
  flex: 1;
  overflow: auto;
  padding: 16px;
}

.right-panel {
  width: 300px;
  background: #252526;
  border-left: 1px solid #3c3c3c;
  display: flex;
  flex-direction: column;
  transition: width 0.2s;
}

.right-panel.collapsed {
  width: 40px;
}

.info-content p {
  margin: 8px 0;
  color: #858585;
}

.ai-content {
  padding: 8px 0;
}

.icon {
  color: #858585;
}
</style>