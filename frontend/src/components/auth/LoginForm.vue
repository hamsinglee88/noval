<script setup lang="ts">
import { computed, reactive, ref } from 'vue';
import { useRouter } from 'vue-router';
import {
  NAlert,
  NButton,
  NForm,
  NFormItem,
  NInput,
  NSpace,
  useMessage,
} from 'naive-ui';

import { useAuthStore } from '@/stores/auth';
import { getErrorMessage } from '@/utils/http';
import { validateUsername } from '@/utils/validation';

const router = useRouter();
const message = useMessage();
const authStore = useAuthStore();

const form = reactive({
  username: '',
  password: '',
});

const loading = ref(false);
const usernameError = ref('');
const formError = ref('');

const canSubmit = computed(() => Boolean(form.username && form.password && !usernameError.value));

function validateUsernameField() {
  usernameError.value = validateUsername(form.username);
}

async function submit() {
  validateUsernameField();
  if (!canSubmit.value) {
    return;
  }

  loading.value = true;
  formError.value = '';

  try {
    // trim 输入后提交
    const result = await authStore.login({
      username: form.username.trim(),
      password: form.password,
    });
    message.success('登录成功');
    await router.push(result);
  } catch (error) {
    formError.value = getErrorMessage(error);
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <n-space vertical size="large">
    <n-alert v-if="formError" type="error" :bordered="false">
      {{ formError }}
    </n-alert>

    <n-form @submit.prevent="submit">
      <n-space vertical size="large">
        <n-form-item label="用户名" :validation-status="usernameError ? 'error' : undefined" :feedback="usernameError">
          <n-input
            v-model:value="form.username"
            placeholder="writer_01"
            @blur="validateUsernameField"
            maxlength="20"
          />
        </n-form-item>

        <n-form-item label="密码">
          <n-input
            v-model:value="form.password"
            type="password"
            show-password-on="click"
            placeholder="请输入密码"
            maxlength="72"
          />
        </n-form-item>

        <n-button type="primary" attr-type="submit" :loading="loading" :disabled="!canSubmit" block>
          登录
        </n-button>

        <n-space justify="space-between" align="center">
          <router-link to="/register">还没有账户？注册</router-link>
          <n-button text disabled>忘记密码（预留）</n-button>
        </n-space>
      </n-space>
    </n-form>
  </n-space>
</template>
