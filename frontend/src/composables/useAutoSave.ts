import { ref, watch, onUnmounted } from 'vue'
import axios from 'axios'

interface AutoSaveOptions {
  delay?: number // 延迟时间（毫秒）
  onSave?: () => void
  onError?: (error: Error) => void
}

export function useAutoSave(
  contentRef: () => string,
  saveFn: (content: string) => Promise<void>,
  options: AutoSaveOptions = {}
) {
  const { delay = 30000, onSave, onError } = options
  
  const isSaving = ref(false)
  const lastSaved = ref<Date | null>(null)
  const saveError = ref<string | null>(null)
  
  let saveTimer: ReturnType<typeof setTimeout> | null = null
  let lastContent = ''

  async function save() {
    const content = contentRef()
    
    // 内容没变化，不保存
    if (content === lastContent) {
      return
    }
    
    isSaving.value = true
    saveError.value = null
    
    try {
      await saveFn(content)
      lastContent = content
      lastSaved.value = new Date()
      onSave?.()
    } catch (error: any) {
      saveError.value = error.message || '保存失败'
      onError?.(error)
    } finally {
      isSaving.value = false
    }
  }

  function scheduleSave() {
    if (saveTimer) {
      clearTimeout(saveTimer)
    }
    saveTimer = setTimeout(save, delay)
  }

  function saveNow() {
    if (saveTimer) {
      clearTimeout(saveTimer)
      saveTimer = null
    }
    return save()
  }

  // 监听内容变化
  watch(contentRef, () => {
    scheduleSave()
  }, { immediate: false })

  // 清理
  onUnmounted(() => {
    if (saveTimer) {
      clearTimeout(saveTimer)
    }
  })

  return {
    isSaving,
    lastSaved,
    saveError,
    saveNow,
    scheduleSave
  }
}

// 章节自动保存 hook
export function useChapterAutoSave(novelId: string, chapterId: string) {
  const content = ref('')
  
  async function saveChapter(content: string) {
    await axios.put(`/api/projects/${novelId}/chapters/${chapterId}`, {
      content
    })
  }

  const autoSave = useAutoSave(
    () => content.value,
    saveChapter,
    { delay: 30000 }
  )

  return {
    content,
    ...autoSave
  }
}