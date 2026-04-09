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
          <h1>风格档案创建引导</h1>
          <p class="muted">新账户已自动登录。下一步会引导你上传代表作，生成个人风格档案。</p>
        </div>
        <UserMenu />
      </header>

      <div class="placeholder-grid">
        <section class="placeholder-block">
          <h3>欢迎，{{ authStore.user?.username }}</h3>
          <p class="muted">Story 1.1 已把注册完成后的跳转入口打通，后续 Story 1.2 将在这里接入上传与分析任务。</p>
        </section>
      </div>
    </section>
  </main>
</template>
