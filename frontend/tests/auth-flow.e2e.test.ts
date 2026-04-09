import { setActivePinia, createPinia } from 'pinia';

import { useAuthStore } from '@/stores/auth';
import * as api from '@/services/api';

describe('auth flow e2e', () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    localStorage.clear();
  });

  it('should handle login success', async () => {
    vi.spyOn(api.authApi, 'login').mockResolvedValue({
      user: {
        id: 'user-1',
        username: 'writer123',
        created_at: '2026-04-09T10:00:00Z',
        last_login_at: '2026-04-09T10:00:00Z',
      },
      session: {
        token: 'token-1',
        expires_at: '2026-04-16T10:00:00Z',
      },
    });

    const store = useAuthStore();
    const route = await store.login({
      username: 'writer123',
      password: 'SecurePass123',
    });

    expect(route).toBe('/projects');
    expect(store.isAuthenticated).toBe(true);
    expect(store.user?.username).toBe('writer123');
    expect(localStorage.getItem('noval-auth-session')).toContain('token-1');
  });

  it('should handle login failure', async () => {
    vi.spyOn(api.authApi, 'login').mockRejectedValue(new Error('用户名或密码错误。'));

    const store = useAuthStore();

    await expect(
      store.login({
        username: 'writer123',
        password: 'Wrong123',
      }),
    ).rejects.toThrow('用户名或密码错误。');
  });
});
