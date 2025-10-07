import { api } from '../../lib/api';

export type RegisterRequest = {
  email: string;
  username: string;
  password: string;
  dob?: string | null;
  bio?: string | null;
};

export type RegisterResponse = {
  id: number;
  email: string;
  username: string;
};

export type LoginRequest = {
  identifier: string; // email or username
  password: string;
};

export type LoginResponse = {
  user_id: number;
  username: string;
  email: string;
};

export const authApi = {
  register: (body: RegisterRequest) =>
    api.post<RegisterResponse>('/api/auth/register', body),
  login: (body: LoginRequest) =>
    api.post<LoginResponse>('/api/auth/login', body),
  logout: () => api.post<void>('/api/auth/logout'),
  status: () => api.get<string>('/api/status'),
};
