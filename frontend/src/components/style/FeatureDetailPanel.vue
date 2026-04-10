<template>
  <div class="feature-detail-panel">
    <n-collapse>
      <!-- 词汇层 -->
      <n-collapse-item title="词汇层特征" name="vocabulary">
        <div class="feature-content">
          <n-descriptions bordered :column="2">
            <n-descriptions-item label="词汇丰富度 (TTR)">
              {{ styleData.vocabulary?.ttr?.toFixed(2) || '-' }}
            </n-descriptions-item>
            <n-descriptions-item label="修正 TTR">
              {{ styleData.vocabulary?.root_ttr?.toFixed(2) || '-' }}
            </n-descriptions-item>
            <n-descriptions-item label="总词数" :span="2">
              {{ styleData.vocabulary?.total_words || '-' }}
            </n-descriptions-item>
            <n-descriptions-item label="唯一词数" :span="2">
              {{ styleData.vocabulary?.unique_words || '-' }}
            </n-descriptions-item>
            <n-descriptions-item label="常用形容词" :span="2">
              {{ formatWordList(styleData.vocabulary?.common_adjectives) }}
            </n-descriptions-item>
            <n-descriptions-item label="常用动词" :span="2">
              {{ formatWordList(styleData.vocabulary?.common_verbs) }}
            </n-descriptions-item>
          </n-descriptions>
        </div>
      </n-collapse-item>
      
      <!-- 句式层 -->
      <n-collapse-item title="句式层特征" name="sentence">
        <div class="feature-content">
          <n-descriptions bordered :column="2">
            <n-descriptions-item label="平均句长">
              {{ styleData.sentence?.avg_sentence_length?.toFixed(1) || '-' }} 字
            </n-descriptions-item>
            <n-descriptions-item label="句长方差">
              {{ styleData.sentence?.sentence_length_variance?.toFixed(2) || '-' }}
            </n-descriptions-item>
            <n-descriptions-item label="短句比例">
              {{ formatPercent(styleData.sentence?.short_sentence_ratio) }}
            </n-descriptions-item>
            <n-descriptions-item label="中句比例">
              {{ formatPercent(styleData.sentence?.medium_sentence_ratio) }}
            </n-descriptions-item>
            <n-descriptions-item label="长句比例">
              {{ formatPercent(styleData.sentence?.long_sentence_ratio) }}
            </n-descriptions-item>
            <n-descriptions-item label="复合句比例">
              {{ formatPercent(styleData.sentence?.complex_sentence_ratio) }}
            </n-descriptions-item>
          </n-descriptions>
        </div>
      </n-collapse-item>

      <!-- 修辞层 -->
      <n-collapse-item title="修辞层特征" name="rhetoric">
        <div class="feature-content">
          <n-descriptions bordered :column="2">
            <n-descriptions-item label="隐喻频率">
              {{ styleData.rhetoric?.metaphor_frequency?.toFixed(1) || '-' }} 次/万字
            </n-descriptions-item>
            <n-descriptions-item label="明喻频率">
              {{ styleData.rhetoric?.simile_frequency?.toFixed(1) || '-' }} 次/万字
            </n-descriptions-item>
            <n-descriptions-item label="排比频率">
              {{ styleData.rhetoric?.parallelism_frequency?.toFixed(1) || '-' }} 次/万字
            </n-descriptions-item>
            <n-descriptions-item label="感官偏好">
              {{ formatSensoryPreferences(styleData.rhetoric?.sensory_preferences) }}
            </n-descriptions-item>
          </n-descriptions>
        </div>
      </n-collapse-item>

      <!-- 叙事层 -->
      <n-collapse-item title="叙事层特征" name="narrative">
        <div class="feature-content">
          <n-descriptions bordered :column="2">
            <n-descriptions-item label="叙事视角">
              {{ styleData.narrative?.pov_type || '-' }}
            </n-descriptions-item>
            <n-descriptions-item label="视角一致性">
              {{ formatPercent(styleData.narrative?.pov_consistency) }}
            </n-descriptions-item>
            <n-descriptions-item label="Show vs Tell">
              {{ formatPercent(styleData.narrative?.show_vs_tell_ratio) }}
            </n-descriptions-item>
            <n-descriptions-item label="实体密度">
              {{ styleData.narrative?.entity_density?.toFixed(1) || '-' }} 个/千字
            </n-descriptions-item>
          </n-descriptions>
        </div>
      </n-collapse-item>

      <!-- 情感层 -->
      <n-collapse-item title="情感层特征" name="emotion">
        <div class="feature-content">
          <n-descriptions bordered :column="2">
            <n-descriptions-item label="情感基调">
              {{ styleData.emotion?.overall_tone || '-' }}
            </n-descriptions-item>
            <n-descriptions-item label="基调置信度">
              {{ formatPercent(styleData.emotion?.tone_confidence) }}
            </n-descriptions-item>
            <n-descriptions-item label="情感波动幅度">
              {{ styleData.emotion?.emotional_amplitude?.toFixed(2) || '-' }}
            </n-descriptions-item>
            <n-descriptions-item label="直接表达比例">
              {{ formatPercent(styleData.emotion?.direct_expression_ratio) }}
            </n-descriptions-item>
          </n-descriptions>
        </div>
      </n-collapse-item>

      <!-- 节奏层 -->
      <n-collapse-item title="节奏层特征" name="pacing">
        <div class="feature-content">
          <n-descriptions bordered :column="2">
            <n-descriptions-item label="平均章节长度">
              {{ styleData.pacing?.avg_chapter_length?.toFixed(0) || '-' }} 字
            </n-descriptions-item>
            <n-descriptions-item label="章节长度方差">
              {{ styleData.pacing?.chapter_length_variance?.toFixed(0) || '-' }}
            </n-descriptions-item>
            <n-descriptions-item label="场景切换频率">
              {{ styleData.pacing?.scene_transition_frequency?.toFixed(1) || '-' }} 次/章
            </n-descriptions-item>
            <n-descriptions-item label="悬念结尾比例">
              {{ formatPercent(styleData.pacing?.cliffhanger_ratio) }}
            </n-descriptions-item>
          </n-descriptions>
        </div>
      </n-collapse-item>

      <!-- 对话层 -->
      <n-collapse-item title="对话层特征" name="dialogue">
        <div class="feature-content">
          <n-descriptions bordered :column="2">
            <n-descriptions-item label="对话比例">
              {{ formatPercent(styleData.dialogue?.dialogue_ratio) }}
            </n-descriptions-item>
            <n-descriptions-item label="角色声音区分度">
              {{ formatPercent(styleData.dialogue?.character_voice_distinction) }}
            </n-descriptions-item>
            <n-descriptions-item label="对话标签频率">
              {{ styleData.dialogue?.dialogue_tag_frequency?.toFixed(1) || '-' }} 次/百句
            </n-descriptions-item>
            <n-descriptions-item label="平均对话长度">
              {{ styleData.dialogue?.avg_dialogue_length?.toFixed(0) || '-' }} 字
            </n-descriptions-item>
          </n-descriptions>
        </div>
      </n-collapse-item>

      <!-- 描写层 -->
      <n-collapse-item title="描写层特征" name="description">
        <div class="feature-content">
          <n-descriptions bordered :column="2">
            <n-descriptions-item label="描写比例">
              {{ formatPercent(styleData.description?.description_ratio) }}
            </n-descriptions-item>
            <n-descriptions-item label="详细程度">
              {{ formatPercent(styleData.description?.detail_granularity) }}
            </n-descriptions-item>
            <n-descriptions-item label="动作描写比例">
              {{ formatPercent(styleData.description?.action_description_ratio) }}
            </n-descriptions-item>
            <n-descriptions-item label="环境描写比例">
              {{ formatPercent(styleData.description?.environment_description_ratio) }}
            </n-descriptions-item>
            <n-descriptions-item label="心理描写比例">
              {{ formatPercent(styleData.description?.psychological_description_ratio) }}
            </n-descriptions-item>
            <n-descriptions-item label="外貌描写比例">
              {{ formatPercent(styleData.description?.appearance_description_ratio) }}
            </n-descriptions-item>
          </n-descriptions>
        </div>
      </n-collapse-item>
    </n-collapse>
  </div>
</template>

<script setup lang="ts">
import { NCollapse, NCollapseItem, NDescriptions, NDescriptionsItem } from 'naive-ui';

interface StyleData {
  vocabulary?: any;
  sentence?: any;
  rhetoric?: any;
  narrative?: any;
  emotion?: any;
  pacing?: any;
  dialogue?: any;
  description?: any;
}

defineProps<{
  styleData: StyleData;
}>();

function formatPercent(value: number | undefined): string {
  if (value === undefined || value === null) return '-';
  return `${(value * 100).toFixed(1)}%`;
}

function formatWordList(words: [string, number][] | undefined): string {
  if (!words || words.length === 0) return '-';
  return words.slice(0, 5).map(([word, freq]) => `${word}(${freq})`).join(', ');
}

function formatSensoryPreferences(prefs: Record<string, number> | undefined): string {
  if (!prefs) return '-';
  const entries = Object.entries(prefs);
  if (entries.length === 0) return '-';
  return entries.map(([key, value]) => `${key}: ${value.toFixed(0)}`).join(', ');
}
</script>

<style scoped>
.feature-detail-panel {
  width: 100%;
}

.feature-content {
  padding: 16px 0;
}
</style>