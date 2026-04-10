import { createRouter, createWebHistory } from 'vue-router';

import LoginView from '@/views/LoginView.vue';
import ProjectsView from '@/views/ProjectsView.vue';
import RegisterView from '@/views/RegisterView.vue';
import StyleOnboardingView from '@/views/StyleOnboardingView.vue';
import StyleLibraryView from '@/views/StyleLibraryView.vue';
import StyleReportView from '@/views/StyleReportView.vue';
import SaveStyleProfileView from '@/views/SaveStyleProfileView.vue';
import CreateProjectView from '@/views/CreateProjectView.vue';

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      redirect: '/login',
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
      path: '/projects/create',
      name: 'create-project',
      component: CreateProjectView,
      meta: { requiresAuth: true },
    },
    {
      path: '/projects/:id',
      name: 'project-detail',
      component: () => import('@/views/ProjectDetailView.vue'),
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
    {
      path: '/styles/:id/report',
      name: 'style-report',
      component: StyleReportView,
      meta: { requiresAuth: true },
    },
    {
      path: '/styles/:taskId/save',
      name: 'save-style-profile',
      component: SaveStyleProfileView,
      meta: { requiresAuth: true },
    },
  ],
});

export default router;