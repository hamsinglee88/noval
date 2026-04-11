<template>
  <div class="coherence-report">
    <h3>连贯性检查报告</h3>
    
    <div class="report-summary">
      <div class="summary-item">
        <span class="label">总章节</span>
        <span class="value">{{ report.total_chapters }}</span>
      </div>
      <div class="summary-item">
        <span class="label">伏笔数</span>
        <span class="value">{{ report.foreshadow_count }}</span>
      </div>
      <div class="summary-item">
        <span class="label">逾期伏笔</span>
        <span class="value warning">{{ report.overdue_count }}</span>
      </div>
      <div class="summary-item">
        <span class="label">一致性评分</span>
        <span class="value">{{ (report.consistency_score * 100).toFixed(0) }}%</span>
      </div>
    </div>
    
    <div v-if="report.issues.length > 0" class="issues-section">
      <h4>发现的问题</h4>
      <div v-for="(issue, index) in report.issues" :key="index" class="issue-item">
        <n-tag type="warning" size="small">{{ index + 1 }}</n-tag>
        <span>{{ issue }}</span>
      </div>
    </div>
    
    <div v-else class="no-issues">
      <n-empty description="未发现问题" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { NTag, NEmpty } from 'naive-ui'

interface ScanResult {
  total_chapters: number
  foreshadow_count: number
  overdue_count: number
  consistency_score: number
  issues: string[]
}

defineProps<{
  report: ScanResult
}>()
</script>

<style scoped>
.coherence-report { background: #252526; border-radius: 8px; padding: 16px; }
.report-summary { display: grid; grid-template-columns: repeat(4, 1fr); gap: 16px; margin-bottom: 24px; }
.summary-item { text-align: center; }
.summary-item .label { display: block; color: #858585; font-size: 12px; margin-bottom: 4px; }
.summary-item .value { font-size: 24px; color: #4EC9B0; }
.summary-item .value.warning { color: #d9534f; }
.issues-section h4 { color: #d4d4d4; margin-bottom: 12px; }
.issue-item { display: flex; gap: 8px; align-items: center; padding: 8px; background: #2d2d2d; border-radius: 4px; margin-bottom: 8px; }
.no-issues { padding: 40px 0; }
</style>