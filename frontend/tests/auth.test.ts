import { mount } from '@vue/test-utils';
import { NMessageProvider } from 'naive-ui';
import { createPinia } from 'pinia';
import { createMemoryHistory, createRouter } from 'vue-router';
import { defineComponent } from 'vue';

import RegisterForm from '@/components/auth/RegisterForm.vue';
import { getPasswordStrength, validateUsername } from '@/utils/validation';

describe('auth validation', () => {
  it('should validate username format', () => {
    expect(validateUsername('ab')).toContain('3-20');
    expect(validateUsername('writer_01')).toBe('');
  });

  it('should validate password strength', () => {
    expect(getPasswordStrength('weak').level).toBe(1);
    expect(getPasswordStrength('Strong123').level).toBe(2);
    expect(getPasswordStrength('Strong#123456').level).toBe(3);
  });

  it('should render password strength feedback', async () => {
    const router = createRouter({
      history: createMemoryHistory(),
      routes: [
        {
          path: '/',
          component: { template: '<div />' },
        },
        {
          path: '/login',
          component: { template: '<div />' },
        },
      ],
    });

    await router.push('/');
    await router.isReady();

    const Host = defineComponent({
      components: {
        NMessageProvider,
        RegisterForm,
      },
      template: `
        <n-message-provider>
          <RegisterForm />
        </n-message-provider>
      `,
    });

    const wrapper = mount(Host, {
      global: {
        plugins: [createPinia(), router],
      },
    });

    const input = wrapper.find('input[type="password"]');
    await input.setValue('Strong123');

    expect(wrapper.text()).toContain('密码强度：中');
  });
});
