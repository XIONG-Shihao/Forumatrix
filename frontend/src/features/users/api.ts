import { api } from '../../lib/api'; // you can keep this for getUser
import type { UserPublic } from './types';

// Reuse existing helper for GET
export async function getUser(id: number): Promise<UserPublic> {
  return api.get<UserPublic>(`/api/users/${id}`);
}

export async function uploadAvatar(
  userId: number,
  file: File
): Promise<{ url: string }> {
  const fd = new FormData();
  fd.append('file', file);

  const res = await fetch(`/api/users/${userId}/avatar`, {
    method: 'PUT',
    body: fd,
    credentials: 'include', // 👈 send sid cookie
  });
  if (!res.ok) {
    const msg = await extractApiError(res);
    throw new Error(msg);
  }
  return res.json();
}

// ---- FIX: real PUT with credentials + JSON ----
export async function updateUser(
  id: number,
  body: { username: string; dob: string | null; bio: string | null }
): Promise<void> {
  const res = await fetch(`/api/users/${id}`, {
    method: 'PUT', // 👈 must be PUT (backend route)
    headers: { 'Content-Type': 'application/json' },
    credentials: 'include', // 👈 send sid cookie
    body: JSON.stringify(body),
  });
  if (!res.ok) {
    const msg = await extractApiError(res);
    throw new Error(msg);
  }
  // backend may return 200 with JSON or empty; we don't require the body
}

async function extractApiError(res: Response): Promise<string> {
  const ct = res.headers.get('content-type') || '';
  try {
    if (ct.includes('application/json')) {
      const data = await res.json();
      return data?.error?.message ?? JSON.stringify(data);
    }
    return await res.text();
  } catch {
    return `${res.status} ${res.statusText}`;
  }
}
