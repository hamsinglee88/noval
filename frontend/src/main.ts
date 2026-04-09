import { createApp } from 'vue';
import { createPinia } from 'pinia';
import { NConfigProvider, NMessageProvider, darkTheme } from 'naive-ui';

import App from './App.vue';
import router from './router';
import { useAuthStore } from './stores/auth';
import './styles.css';

const app = createApp({
  components: {
    App,
    NConfigProvider,
    NMessageProvider,
  },
  template: `
    <n-config-provider
      :theme="darkTheme"
      :theme-overrides="themeOverrides"
    >
      <n-message-provider placement="top">
        <App />
      </n-message-provider>
    </n-config-provider>
  `,
  setup() {
    const themeOverrides = {
      common: {
        bodyColor: '#1E1E1E',
        cardColor: '#252526',
        modalColor: '#252526',
        primaryColor: '#4EC9B0',
        primaryColorHover: '#66d3bd',
        primaryColorPressed: '#39b19a',
        textColorBase: '#D4D4D4',
        borderColor: 'rgba(212, 212, 212, 0.12)',
        errorColor: '#F48771',
      },
    };

    return {
      darkTheme,
      themeOverrides,
    };
  },
});

const pinia = createPinia();
app.use(pinia);

const authStore = useAuthStore(pinia);
await authStore.restoreSession();

app.use(router);
app.mount('#app');
