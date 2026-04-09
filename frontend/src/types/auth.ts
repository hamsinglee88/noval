export interface AuthUser {
  id: string;
  username: string;
  created_at: string;
  last_login_at?: string | null;
}

export interface SessionInfo {
  token: string;
  expires_at: string;
}

export interface AuthPayload {
  user: AuthUser;
  session: SessionInfo;
}

export interface ApiSuccess<T> {
  success: boolean;
  data: T;
}
