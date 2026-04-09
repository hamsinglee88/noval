<script setup lang="ts">
import LoginForm from '@/components/auth/LoginForm.vue';
</script>

<template>
  <main class="auth-page">
    <section class="auth-panel">
      <div class="auth-brand">
        <h1>Novel Agent</h1>
        <p>本地优先的长篇小说创作工作台</p>
      </div>
      <LoginForm />
    </section>
  </main>
</template>
