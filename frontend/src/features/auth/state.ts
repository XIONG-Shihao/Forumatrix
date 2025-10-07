// frontend/src/features/auth/state.ts
export type AuthSession = {
  userId: number;
  username: string;
  email: string;
} | null;

const KEY = 'arena-ai:auth';
let current: AuthSession = loadAuth();

export function loadAuth(): AuthSession {
  try {
    const raw = localStorage.getItem(KEY);
    return raw ? (JSON.parse(raw) as AuthSession) : null;
  } catch {
    return null;
  }
}

export function saveAuth(a: AuthSession) {
  current = a;
  if (a) localStorage.setItem(KEY, JSON.stringify(a));
  else localStorage.removeItem(KEY);
}

export function clearAuth() {
  saveAuth(null);
}

export function getAuth(): AuthSession {
  return current;
}
