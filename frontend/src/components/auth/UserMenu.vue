<script setup lang="ts">
import { useRouter } from 'vue-router';
import { NButton, NSpace, useMessage } from 'naive-ui';

import { useAuthStore } from '@/stores/auth';

const authStore = useAuthStore();
const router = useRouter();
const message = useMessage();

async function logout() {
  await authStore.logout();
  message.success('已成功登出');
  await router.push('/login');
}
</script>

<template>
  <n-space align="center">
    <span class="muted">{{ authStore.user?.username }}</span>
    <n-button secondary type="default" @click="logout">
      登出
    </n-button>
  </n-space>
</template>
