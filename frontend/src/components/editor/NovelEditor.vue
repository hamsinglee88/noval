<template>
  <div class="novel-editor">
    <!-- 工具栏 -->
    <div class="editor-toolbar">
      <n-button-group>
        <n-button size="small" @click="toggleBold" :type="isBold ? 'primary' : 'default'">
          B
        </n-button>
        <n-button size="small" @click="toggleItalic" :type="isItalic ? 'primary' : 'default'">
          I
        </n-button>
        <n-button size="small" @click="toggleHeading">
          H
        </n-button>
      </n-button-group>
      
      <n-button-group>
        <n-button size="small" @click="undo">撤销</n-button>
        <n-button size="small" @click="redo">重做</n-button>
      </n-button-group>
    </div>

    <!-- 编辑器内容区 -->
    <editor-content :editor="editor" class="editor-content" />
    
    <!-- 底部状态栏 -->
    <div class="editor-status-bar">
      <span>字数: {{ wordCount }}</span>
      <span>字符: {{ charCount }}</span>
      <span>{{ saveStatus }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onBeforeUnmount } from 'vue'
import { useEditor, EditorContent } from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'
import Placeholder from '@tiptap/extension-placeholder'
import CharacterCount from '@tiptap/extension-character-count'
import { NButton, NButtonGroup } from 'naive-ui'

const props = defineProps<{
  modelValue?: string
  placeholder?: string
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void
  (e: 'save', content: string): void
}>()

const saveStatus = ref('已保存')

const editor = useEditor({
  content: props.modelValue || '',
  extensions: [
    StarterKit,
    Placeholder.configure({
      placeholder: props.placeholder || '开始创作...'
    }),
    CharacterCount
  ],
  onUpdate: ({ editor }) => {
    const html = editor.getHTML()
    emit('update:modelValue', html)
    saveStatus.value = '未保存'
  }
})

const wordCount = computed(() => {
  if (!editor.value) return 0
  return editor.value.storage.characterCount.words()
})

const charCount = computed(() => {
  if (!editor.value) return 0
  return editor.value.storage.characterCount.characters()
})

const isBold = computed(() => editor.value?.isActive('bold') || false)
const isItalic = computed(() => editor.value?.isActive('italic') || false)

function toggleBold() {
  editor.value?.chain().focus().toggleBold().run()
}

function toggleItalic() {
  editor.value?.chain().focus().toggleItalic().run()
}

function toggleHeading() {
  editor.value?.chain().focus().toggleHeading({ level: 2 }).run()
}

function undo() {
  editor.value?.chain().focus().undo().run()
}

function redo() {
  editor.value?.chain().focus().redo().run()
}

function save() {
  if (editor.value) {
    emit('save', editor.value.getHTML())
    saveStatus.value = '已保存'
  }
}

function getContent(): string {
  return editor.value?.getHTML() || ''
}

function setContent(content: string) {
  editor.value?.commands.setContent(content)
}

// 暴露方法给父组件
defineExpose({
  save,
  getContent,
  setContent
})

watch(() => props.modelValue, (newValue) => {
  if (editor.value && newValue !== editor.value.getHTML()) {
    editor.value.commands.setContent(newValue || '')
  }
})

onBeforeUnmount(() => {
  editor.value?.destroy()
})
</script>

<style scoped>
.novel-editor {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: #1e1e1e;
}

.editor-toolbar {
  display: flex;
  gap: 8px;
  padding: 8px;
  background: #252526;
  border-bottom: 1px solid #3c3c3c;
}

.editor-content {
  flex: 1;
  overflow-y: auto;
  padding: 24px;
}

.editor-content :deep(.tiptap) {
  outline: none;
  min-height: 100%;
  color: #d4d4d4;
  font-size: 16px;
  line-height: 1.8;
}

.editor-content :deep(.tiptap p) {
  margin-bottom: 1em;
}

.editor-content :deep(.tiptap h1),
.editor-content :deep(.tiptap h2),
.editor-content :deep(.tiptap h3) {
  color: #ffffff;
  margin-top: 1.5em;
  margin-bottom: 0.5em;
}

.editor-content :deep(.tiptap .is-editor-empty:first-child::before) {
  content: attr(data-placeholder);
  float: left;
  color: #858585;
  pointer-events: none;
  height: 0;
}

.editor-status-bar {
  display: flex;
  gap: 16px;
  padding: 4px 8px;
  background: #007acc;
  color: #ffffff;
  font-size: 12px;
}
</style>