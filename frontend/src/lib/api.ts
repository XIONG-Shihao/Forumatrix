// frontend/src/lib/api.ts
export type Json = Record<string, unknown> | Array<unknown>;

import { UserPublic } from './types';

async function request<T>(
  path: string,
  opts: RequestInit & { json?: Json } = {}
): Promise<T> {
  const init: RequestInit = {
    method: opts.method ?? 'GET',
    credentials: 'include',
    headers: {
      ...(opts.json ? { 'Content-Type': 'application/json' } : {}),
      ...(opts.headers ?? {}),
    },
    body: opts.json ? JSON.stringify(opts.json) : opts.body,
  };

  const res = await fetch(path.startsWith('/') ? path : `/api${path}`, init);
  if (!res.ok) {
    const ct = res.headers.get('content-type') || '';
    let msg = `${res.status} ${res.statusText}`;
    if (ct.includes('application/json')) {
      try {
        const data = await res.json();
        msg = data?.error?.message ?? JSON.stringify(data);
      } catch {}
    } else {
      msg = await res.text();
    }
    throw new Error(msg);
  }

  const isJson = (res.headers.get('content-type') || '').includes(
    'application/json'
  );
  return (isJson ? res.json() : (undefined as unknown)) as T;
}

export const api = {
  get: <T>(path: string) => request<T>(path),
  post: <T>(path: string, json?: Json) =>
    request<T>(path, { method: 'POST', json }),
  put: <T>(path: string, json?: Json) =>
    request<T>(path, { method: 'PUT', json }), // 👈 add this
  del: <T>(path: string) => request<T>(path, { method: 'DELETE' }), // 👈 optional but useful
};

// Existing helpers below are fine:
export async function logout(): Promise<void> {
  const res = await fetch('/api/auth/logout', {
    method: 'POST',
    credentials: 'include',
  });
  if (!(res.status === 204 || res.ok)) {
    throw new Error(`Logout failed (status ${res.status})`);
  }
}

export async function getUser(id: number): Promise<UserPublic> {
  return request<UserPublic>(`/api/users/${id}`);
}

export async function suspendUser(
  userId: number
): Promise<{ user_id: number; is_active: number }> {
  // Note: pass a path starting with "/api" since request() handles it.
  return api.put<{ user_id: number; is_active: number }>(
    `/api/users/${userId}/suspend`
  );
}
