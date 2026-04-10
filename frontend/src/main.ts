import { createApp, h } from 'vue';
import { createPinia } from 'pinia';
import { NConfigProvider, NMessageProvider, darkTheme, lightTheme } from 'naive-ui';

import App from './App.vue';
import router from './router';
import './styles.css';

const pinia = createPinia();

const app = createApp({
  render() {
    return h(
      NConfigProvider,
      {
        theme: lightTheme,
        themeOverrides: {
          common: {
            bodyColor: '#f5f5f5',
            cardColor: '#ffffff',
            modalColor: '#ffffff',
            primaryColor: '#18a058',
            primaryColorHover: '#36ad6a',
            primaryColorPressed: '#0c7a43',
            textColorBase: '#333333',
            borderColor: '#e0e0e0',
            errorColor: '#d03050',
          },
        },
      },
      () =>
        h(NMessageProvider, { placement: 'top' }, () => h(App))
    );
  },
});

app.use(pinia);
app.use(router);

await router.isReady();
app.mount('#app');
