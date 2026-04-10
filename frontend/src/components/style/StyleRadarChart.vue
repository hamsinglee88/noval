<template>
  <div class="style-radar-chart">
    <Radar 
      :data="radarData" 
      :options="radarOptions"
      role="img"
      aria-label="风格雷达图：展示七层风格特征"
    />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { Radar } from 'vue-chartjs';
import { 
  Chart as ChartJS, 
  RadialLinearScale, 
  PointElement, 
  LineElement, 
  Filler, 
  Tooltip, 
  Legend 
} from 'chart.js';

ChartJS.register(RadialLinearScale, PointElement, LineElement, Filler, Tooltip, Legend);

interface LayerData {
  score: number;
  [key: string]: any;
}

interface StyleData {
  vocabulary: LayerData;
  sentence: LayerData;
  rhetoric: LayerData;
  narrative: LayerData;
  emotion: LayerData;
  pacing: LayerData;
  dialogue: LayerData;
  description: LayerData;
}

const props = defineProps<{
  styleData: StyleData;
}>();

const radarData = computed(() => ({
  labels: ['词汇层', '句式层', '修辞层', '叙事层', '情感层', '节奏层', '对话层', '描写层'],
  datasets: [{
    label: '风格特征强度',
    data: [
      calculateLayerScore(props.styleData.vocabulary),
      calculateLayerScore(props.styleData.sentence),
      calculateLayerScore(props.styleData.rhetoric),
      calculateLayerScore(props.styleData.narrative),
      calculateLayerScore(props.styleData.emotion),
      calculateLayerScore(props.styleData.pacing),
      calculateLayerScore(props.styleData.dialogue),
      calculateLayerScore(props.styleData.description),
    ],
    backgroundColor: 'rgba(78, 201, 176, 0.2)',
    borderColor: '#4EC9B0',
    pointBackgroundColor: '#4EC9B0',
    pointRadius: 4,
  }],
}));

const radarOptions = {
  responsive: true,
  maintainAspectRatio: true,
  scales: {
    r: {
      angleLines: { color: '#3C3C3C' },
      grid: { color: '#3C3C3C' },
      pointLabels: {
        color: '#D4D4D4',
        font: { size: 12 },
      },
      ticks: {
        color: '#858585',
        backdropColor: 'transparent',
        stepSize: 0.2,
      },
      min: 0,
      max: 1,
    },
  },
  plugins: {
    legend: { display: false },
    tooltip: {
      backgroundColor: '#2D2D30',
      titleColor: '#D4D4D4',
      bodyColor: '#858585',
      borderColor: '#4EC9B0',
      borderWidth: 1,
    },
  },
};

function calculateLayerScore(layerData: LayerData): number {
  // 如果有预计算的 score 字段，直接使用
  if (layerData.score !== undefined) {
    return layerData.score;
  }
  
  // 否则返回默认值
  return 0.5;
}
</script>

<style scoped>
.style-radar-chart {
  width: 100%;
  max-width: 500px;
  margin: 0 auto;
}
</style>