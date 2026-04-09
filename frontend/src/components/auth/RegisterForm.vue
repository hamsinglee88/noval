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
import { getPasswordStrength, validateUsername } from '@/utils/validation';

const router = useRouter();
const message = useMessage();
const authStore = useAuthStore();

const form = reactive({
  username: '',
  password: '',
  confirmPassword: '',
});

const loading = ref(false);
const usernameError = ref('');
const formError = ref('');
const passwordError = ref('');

const passwordStrength = computed(() => getPasswordStrength(form.password));
const canSubmit = computed(
  () =>
    Boolean(form.username && form.password && form.confirmPassword) &&
    !usernameError.value &&
    !passwordError.value &&
    passwordStrength.value.level >= 2,
);

function validateUsernameField() {
  usernameError.value = validateUsername(form.username);
}

function validatePasswordMatch() {
  if (!form.confirmPassword) {
    passwordError.value = '';
    return;
  }
  if (form.password !== form.confirmPassword) {
    passwordError.value = '两次输入的密码不一致';
  } else {
    passwordError.value = '';
  }
}

async function submit() {
  validateUsernameField();
  validatePasswordMatch();
  if (!canSubmit.value) {
    return;
  }

  loading.value = true;
  formError.value = '';

  try {
    // trim 输入后提交
    const result = await authStore.register({
      username: form.username.trim(),
      password: form.password,
    });
    message.success('注册成功，已自动登录');
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

        <n-form-item
          label="密码"
          :validation-status="passwordStrength.level === 1 ? 'error' : undefined"
          :feedback="passwordStrength.message"
        >
          <n-input
            v-model:value="form.password"
            type="password"
            show-password-on="click"
            placeholder="至少 8 位，包含字母和数字"
            maxlength="72"
          />
        </n-form-item>

        <n-form-item
          label="确认密码"
          :validation-status="passwordError ? 'error' : undefined"
          :feedback="passwordError"
        >
          <n-input
            v-model:value="form.confirmPassword"
            type="password"
            show-password-on="click"
            placeholder="再次输入密码"
            @blur="validatePasswordMatch"
          />
        </n-form-item>

        <div>
          <div class="password-strength">
            <span
              v-for="index in 3"
              :key="index"
              class="password-strength__bar"
              :class="{ 'is-active': index <= Math.max(passwordStrength.level, 0) }"
              :data-level="Math.max(passwordStrength.level, 1)"
            />
          </div>
          <small class="muted">密码强度：{{ passwordStrength.label }}</small>
        </div>

        <n-button type="primary" attr-type="submit" :loading="loading" :disabled="!canSubmit" block>
          注册
        </n-button>

        <router-link to="/login">已有账户？返回登录</router-link>
      </n-space>
    </n-form>
  </n-space>
</template>
