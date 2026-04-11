<template>
  <Teleport to="body">
    <div 
      v-if="visible" 
      class="ai-menu"
      :style="menuStyle"
      @keydown="handleKeydown"
    >
      <div class="menu-header">
        <span class="menu-title">AI 操作</span>
        <span class="shortcut">⌘K</span>
      </div>
      
      <div class="menu-items" ref="menuItemsRef">
        <div
          v-for="(item, index) in menuItems"
          :key="item.id"
          class="menu-item"
          :class="{ active: activeIndex === index }"
          @click="handleSelect(item)"
          @mouseenter="activeIndex = index"
        >
          <span class="item-icon">{{ item.icon }}</span>
          <div class="item-content">
            <span class="item-label">{{ item.label }}</span>
            <span class="item-desc">{{ item.description }}</span>
          </div>
          <span class="item-shortcut">{{ item.shortcut }}</span>
        </div>
      </div>
      
      <div class="menu-footer">
        <span>↑↓ 选择</span>
        <span>Enter 确认</span>
        <span>Esc 关闭</span>
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick } from 'vue'

interface MenuItem {
  id: string
  label: string
  description: string
  icon: string
  shortcut?: string
  action: string
}

const props = defineProps<{
  visible: boolean
  position: { x: number; y: number }
  selectedText: string
}>()

const emit = defineEmits<{
  (e: 'select', action: string, text: string): void
  (e: 'close'): void
}>()

const activeIndex = ref(0)
const menuItemsRef = ref<HTMLElement | null>(null)

const menuItems: MenuItem[] = [
  {
    id: 'continue',
    label: '续写',
    description: '基于选中文本继续创作',
    icon: '✍️',
    shortcut: '⌘1',
    action: 'continue'
  },
  {
    id: 'polish',
    label: '润色',
    description: '改进文本表达和文采',
    icon: '✨',
    shortcut: '⌘2',
    action: 'polish'
  },
  {
    id: 'expand',
    label: '扩写',
    description: '将大纲扩展为详细场景',
    icon: '📝',
    shortcut: '⌘3',
    action: 'expand'
  },
  {
    id: 'summarize',
    label: '总结',
    description: '生成文本摘要',
    icon: '📋',
    shortcut: '⌘4',
    action: 'summarize'
  },
  {
    id: 'translate',
    label: '改写',
    description: '用不同风格改写文本',
    icon: '🔄',
    shortcut: '⌘5',
    action: 'rewrite'
  }
]

const menuStyle = computed(() => ({
  left: `${props.position.x}px`,
  top: `${props.position.y}px`
}))

function handleKeydown(event: KeyboardEvent) {
  switch (event.key) {
    case 'ArrowUp':
      event.preventDefault()
      activeIndex.value = Math.max(0, activeIndex.value - 1)
      break
    case 'ArrowDown':
      event.preventDefault()
      activeIndex.value = Math.min(menuItems.length - 1, activeIndex.value + 1)
      break
    case 'Enter':
      event.preventDefault()
      handleSelect(menuItems[activeIndex.value])
      break
    case 'Escape':
      event.preventDefault()
      emit('close')
      break
  }
}

function handleSelect(item: MenuItem) {
  emit('select', item.action, props.selectedText)
}

watch(() => props.visible, async (newVal) => {
  if (newVal) {
    activeIndex.value = 0
    await nextTick()
    menuItemsRef.value?.focus()
  }
})
</script>

<style scoped>
.ai-menu {
  position: fixed;
  z-index: 1000;
  background: #252526;
  border: 1px solid #3c3c3c;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  min-width: 280px;
  overflow: hidden;
}

.menu-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  border-bottom: 1px solid #3c3c3c;
  background: #2d2d2d;
}

.menu-title {
  color: #d4d4d4;
  font-weight: 500;
}

.shortcut {
  color: #858585;
  font-size: 12px;
  background: #3c3c3c;
  padding: 2px 6px;
  border-radius: 4px;
}

.menu-items {
  max-height: 300px;
  overflow-y: auto;
}

.menu-item {
  display: flex;
  align-items: center;
  padding: 12px 16px;
  cursor: pointer;
  transition: background 0.1s;
}

.menu-item:hover,
.menu-item.active {
  background: #2a2d2e;
}

.menu-item.active {
  background: #094771;
}

.item-icon {
  font-size: 18px;
  margin-right: 12px;
}

.item-content {
  flex: 1;
  display: flex;
  flex-direction: column;
}

.item-label {
  color: #d4d4d4;
  font-size: 14px;
}

.item-desc {
  color: #858585;
  font-size: 12px;
  margin-top: 2px;
}

.item-shortcut {
  color: #858585;
  font-size: 11px;
  background: #3c3c3c;
  padding: 2px 6px;
  border-radius: 4px;
}

.menu-footer {
  display: flex;
  justify-content: space-around;
  padding: 8px 16px;
  border-top: 1px solid #3c3c3c;
  background: #2d2d2d;
}

.menu-footer span {
  color: #858585;
  font-size: 11px;
}
</style>