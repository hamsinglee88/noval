import { defineStore } from 'pinia';

import { authApi, setSessionToken } from '@/services/api';
import type { AuthPayload, AuthUser } from '@/types/auth';

const storageKey = 'noval-auth-session';

interface PersistedSession {
  token: string;
  expiresAt: string;
  route: '/projects' | '/style-profiles/onboarding';
}

interface AuthState {
  user: AuthUser | null;
  token: string;
  expiresAt: string;
  initialized: boolean;
  landingRoute: '/projects' | '/style-profiles/onboarding';
}

export const useAuthStore = defineStore('auth', {
  state: (): AuthState => ({
    user: null,
    token: '',
    expiresAt: '',
    initialized: false,
    landingRoute: '/projects',
  }),
  getters: {
    isAuthenticated: (state) => Boolean(state.user && state.token),
  },
  actions: {
    applyAuth(payload: AuthPayload, route: '/projects' | '/style-profiles/onboarding') {
      this.user = payload.user;
      this.token = payload.session.token;
      this.expiresAt = payload.session.expires_at;
      this.landingRoute = route;
      setSessionToken(payload.session.token);
      this.persistSession();
    },
    persistSession() {
      if (!this.token) {
        localStorage.removeItem(storageKey);
        return;
      }

      const session: PersistedSession = {
        token: this.token,
        expiresAt: this.expiresAt,
        route: this.landingRoute,
      };
      localStorage.setItem(storageKey, JSON.stringify(session));
    },
    clearSession() {
      this.user = null;
      this.token = '';
      this.expiresAt = '';
      this.landingRoute = '/projects';
      setSessionToken('');
      localStorage.removeItem(storageKey);
    },
    async restoreSession() {
      if (this.initialized) {
        return;
      }

      this.initialized = true;
      const raw = localStorage.getItem(storageKey);
      if (!raw) {
        return;
      }

      const session = JSON.parse(raw) as PersistedSession;
      this.token = session.token;
      this.expiresAt = session.expiresAt;
      this.landingRoute = session.route ?? '/projects';
      setSessionToken(session.token);

      try {
        const payload = await authApi.me();
        this.user = payload.user;
        this.expiresAt = payload.session.expires_at;
        this.persistSession();
      } catch {
        this.clearSession();
      }
    },
    async register(payload: { username: string; password: string }) {
      const result = await authApi.register(payload);
      this.applyAuth(result, '/style-profiles/onboarding');
      return '/style-profiles/onboarding' as const;
    },
    async login(payload: { username: string; password: string }) {
      const result = await authApi.login(payload);
      this.applyAuth(result, '/projects');
      return '/projects' as const;
    },
    async logout() {
      try {
        await authApi.logout();
      } finally {
        this.clearSession();
      }
    },
  },
});
