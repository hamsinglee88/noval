import axios from 'axios';

import type { ApiSuccess, AuthPayload } from '@/types/auth';

const baseURL = import.meta.env.VITE_API_BASE_URL ?? 'http://127.0.0.1:3000';

let currentSessionToken = '';

const http = axios.create({
  baseURL,
});

http.interceptors.request.use((config) => {
  if (currentSessionToken) {
    config.headers.Authorization = `Bearer ${currentSessionToken}`;
  }

  return config;
});

export function setSessionToken(token: string) {
  currentSessionToken = token;
}

export const authApi = {
  async register(payload: { username: string; password: string }) {
    const response = await http.post<ApiSuccess<AuthPayload>>('/api/auth/register', payload);
    return response.data.data;
  },
  async login(payload: { username: string; password: string }) {
    const response = await http.post<ApiSuccess<AuthPayload>>('/api/auth/login', payload);
    return response.data.data;
  },
  async logout() {
    await http.post('/api/auth/logout');
  },
  async me() {
    const response = await http.get<ApiSuccess<AuthPayload>>('/api/auth/me');
    return response.data.data;
  },
};
