<script setup lang="ts">
import UserMenu from '@/components/auth/UserMenu.vue';
import { useAuthStore } from '@/stores/auth';

const authStore = useAuthStore();
</script>

<template>
  <main class="page-shell">
    <section class="page-card">
      <header class="page-header">
        <div>
          <h1>项目列表</h1>
          <p class="muted">欢迎回来，{{ authStore.user?.username }}。你的项目和风格档案会在这里加载。</p>
        </div>
        <UserMenu />
      </header>

      <div class="placeholder-grid">
        <section class="placeholder-block">
          <h3>暂无项目</h3>
          <p class="muted">认证流程已完成。后续 Story 会在这里接入项目创建、章节列表和风格档案数据。</p>
        </section>
      </div>
    </section>
  </main>
</template>
