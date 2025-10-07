// frontend/src/lib/avatarCache.ts
const KEY = 'arena:avatar:ver';

function k(userId: number | null) {
  return `${KEY}:${userId ?? 'anon'}`;
}

/**
 * Returns a stable string used as ?v=... on the avatar URL.
 * If none exists yet for this user, seed it with Date.now() so
 * the first render forces a cache miss.
 */
export function getAvatarVer(userId: number | null): string {
  try {
    const key = k(userId);
    let v = localStorage.getItem(key);
    if (!v) {
      v = String(Date.now());
      localStorage.setItem(key, v);
    }
    return v;
  } catch {
    // localStorage not available? fall back to timestamp
    return String(Date.now());
  }
}

/** Bumps version to a new timestamp and returns it. */
export function bumpAvatarVer(userId: number | null): string {
  const v = String(Date.now());
  try {
    localStorage.setItem(k(userId), v);
  } catch {}
  return v;
}
