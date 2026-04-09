import { defineStore } from 'pinia';
import CryptoJS from 'crypto-js';
import type { AxiosError } from 'axios';

import { authApi, setSessionToken } from '@/services/api';
import type { AuthPayload, AuthUser } from '@/types/auth';

const storageKey = 'noval-auth-session';
const SECRET_KEY = import.meta.env.VITE_SESSION_SECRET || 'noval-dev-secret';

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
      // 使用 AES 加密存储 session
      const encrypted = CryptoJS.AES.encrypt(JSON.stringify(session), SECRET_KEY).toString();
      localStorage.setItem(storageKey, encrypted);
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

      let session: PersistedSession;
      try {
        // 解密 session 数据
        const bytes = CryptoJS.AES.decrypt(raw, SECRET_KEY);
        const decrypted = bytes.toString(CryptoJS.enc.Utf8);
        if (!decrypted) {
          this.clearSession();
          return;
        }
        session = JSON.parse(decrypted) as PersistedSession;
      } catch (e) {
        // 解密或解析失败，清除无效数据
        this.clearSession();
        return;
      }

      this.token = session.token;
      this.expiresAt = session.expiresAt;
      this.landingRoute = session.route ?? '/projects';
      setSessionToken(session.token);

      try {
        const payload = await authApi.me();
        this.user = payload.user;
        this.expiresAt = payload.session.expires_at;
        this.persistSession();
      } catch (error) {
        // 仅在 401/403 时清除 session，网络错误保留当前状态
        const axiosError = error as AxiosError<{ error?: { code?: string } }>;
        const status = axiosError.response?.status;
        if (status === 401 || status === 403) {
          this.clearSession();
        }
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
