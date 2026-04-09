<script setup lang="ts">
import RegisterForm from '@/components/auth/RegisterForm.vue';
</script>

<template>
  <main class="auth-page">
    <section class="auth-panel">
      <div class="auth-brand">
        <h1>创建你的 Novel Agent</h1>
        <p>注册后会直接进入风格档案创建引导。</p>
      </div>
      <RegisterForm />
    </section>
  </main>
</template>
