<template>
  <div class="coherence-report">
    <div class="report-header">
      <h3>连贯性检查报告</h3>
      <n-button size="small" @click="$emit('refresh')" :loading="loading">
        重新扫描
      </n-button>
    </div>
    
    <!-- 加载状态 -->
    <div v-if="loading" class="loading-state">
      <n-spin size="medium" />
      <p>正在扫描全书连贯性...</p>
    </div>
    
    <!-- 错误状态 -->
    <div v-else-if="error" class="error-state">
      <n-result status="error" title="扫描失败" :description="error">
        <template #footer>
          <n-button @click="$emit('refresh')">重试</n-button>
        </template>
      </n-result>
    </div>
    
    <!-- 报告内容 -->
    <template v-else-if="report">
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
          <span class="value" :class="{ warning: report.overdue_count > 0 }">
            {{ report.overdue_count }}
          </span>
        </div>
        <div class="summary-item">
          <span class="label">一致性评分</span>
          <span class="value" :class="getScoreClass(report.consistency_score)">
            {{ (report.consistency_score * 100).toFixed(0) }}%
          </span>
        </div>
      </div>
      
      <div class="score-bar">
        <div 
          class="score-fill" 
          :style="{ width: (report.consistency_score * 100) + '%' }"
          :class="getScoreClass(report.consistency_score)"
        ></div>
      </div>
      
      <div v-if="report.issues.length > 0" class="issues-section">
        <h4>发现的问题</h4>
        <div v-for="(issue, index) in report.issues" :key="index" class="issue-item">
          <n-tag type="warning" size="small">{{ index + 1 }}</n-tag>
          <span>{{ issue }}</span>
        </div>
      </div>
      
      <div v-else class="no-issues">
        <n-result status="success" title="检查通过" description="未发现连贯性问题" />
      </div>
    </template>
    
    <!-- 无数据状态 -->
    <div v-else class="empty-state">
      <n-empty description="暂无扫描数据" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { NButton, NSpin, NResult, NTag, NEmpty } from 'naive-ui'

interface ScanResult {
  total_chapters: number
  foreshadow_count: number
  overdue_count: number
  consistency_score: number
  issues: string[]
}

defineProps<{
  report: ScanResult | null
  loading?: boolean
  error?: string | null
}>()

defineEmits(['refresh'])

function getScoreClass(score: number): string {
  if (score >= 0.9) return 'excellent'
  if (score >= 0.7) return 'good'
  if (score >= 0.5) return 'fair'
  return 'poor'
}
</script>

<style scoped>
.coherence-report {
  background: #252526;
  border-radius: 8px;
  padding: 16px;
}

.report-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.report-header h3 {
  margin: 0;
  color: #d4d4d4;
}

.loading-state,
.error-state,
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 40px 0;
  gap: 16px;
}

.report-summary {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 16px;
  margin-bottom: 16px;
}

.summary-item {
  text-align: center;
}

.summary-item .label {
  display: block;
  color: #858585;
  font-size: 12px;
  margin-bottom: 4px;
}

.summary-item .value {
  font-size: 24px;
  color: #4EC9B0;
}

.summary-item .value.warning {
  color: #d9534f;
}

.summary-item .value.excellent {
  color: #5cb85c;
}

.summary-item .value.good {
  color: #4EC9B0;
}

.summary-item .value.fair {
  color: #f0ad4e;
}

.summary-item .value.poor {
  color: #d9534f;
}

.score-bar {
  height: 8px;
  background: #3c3c3c;
  border-radius: 4px;
  overflow: hidden;
  margin-bottom: 24px;
}

.score-fill {
  height: 100%;
  border-radius: 4px;
  transition: width 0.3s ease;
}

.score-fill.excellent {
  background: #5cb85c;
}

.score-fill.good {
  background: #4EC9B0;
}

.score-fill.fair {
  background: #f0ad4e;
}

.score-fill.poor {
  background: #d9534f;
}

.issues-section h4 {
  color: #d4d4d4;
  margin-bottom: 12px;
}

.issue-item {
  display: flex;
  gap: 8px;
  align-items: center;
  padding: 8px;
  background: #2d2d2d;
  border-radius: 4px;
  margin-bottom: 8px;
}

.no-issues {
  padding: 20px 0;
}
</style>