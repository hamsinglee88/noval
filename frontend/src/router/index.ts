import { createRouter, createWebHistory } from 'vue-router';

import { useAuthStore } from '@/stores/auth';
import LoginView from '@/views/LoginView.vue';
import ProjectsView from '@/views/ProjectsView.vue';
import RegisterView from '@/views/RegisterView.vue';
import StyleOnboardingView from '@/views/StyleOnboardingView.vue';
import StyleLibraryView from '@/views/StyleLibraryView.vue';

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      redirect: () => {
        const authStore = useAuthStore();
        return authStore.isAuthenticated ? authStore.landingRoute : '/login';
      },
    },
    {
      path: '/login',
      name: 'login',
      component: LoginView,
      meta: { guestOnly: true },
    },
    {
      path: '/register',
      name: 'register',
      component: RegisterView,
      meta: { guestOnly: true },
    },
    {
      path: '/projects',
      name: 'projects',
      component: ProjectsView,
      meta: { requiresAuth: true },
    },
    {
      path: '/style-profiles/onboarding',
      name: 'style-onboarding',
      component: StyleOnboardingView,
      meta: { requiresAuth: true },
    },
    {
      path: '/style-library',
      name: 'style-library',
      component: StyleLibraryView,
      meta: { requiresAuth: true },
    },
  ],
});

router.beforeEach(async (to) => {
  const authStore = useAuthStore();
  await authStore.restoreSession();

  if (to.meta.requiresAuth && !authStore.isAuthenticated) {
    return '/login';
  }

  if (to.meta.guestOnly && authStore.isAuthenticated) {
    return authStore.landingRoute;
  }

  return true;
});

export default router;
