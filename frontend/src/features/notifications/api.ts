// Frontend notifications API

export type NotificationItem = {
  id: number;
  kind: 'post_liked' | 'comment_liked' | 'post_replied' | 'comment_replied';
  created_at: number;
  read_at?: number | null;
  actor_username: string;
  actor_avatar_url?: string | null;
  post_id?: number | null; // <- nullable
  post_title?: string | null; // <- nullable
  comment_id?: number | null;
};

export type NotificationListResp = {
  items: NotificationItem[];
  page: number;
  total_pages: number;
  total: number;
};

export async function listNotifications(
  page = 1,
  limit = 20
): Promise<NotificationListResp> {
  const u = new URL('/api/notifications', window.location.origin);
  u.searchParams.set('page', String(page));
  u.searchParams.set('limit', String(limit));
  const res = await fetch(u.toString(), { credentials: 'include' });
  if (!res.ok) throw new Error(await extractApiError(res));
  return res.json();
}

export async function unreadCount(): Promise<number> {
  // backend route is /api/notifications/unread
  const res = await fetch('/api/notifications/unread', {
    credentials: 'include',
  });
  if (!res.ok) throw new Error(await extractApiError(res));
  const data = await res.json();
  return Number(data?.unread ?? 0);
}

export async function markRead(id: number): Promise<void> {
  // backend expects PUT /api/notifications/read with { ids: [...] }
  const res = await fetch('/api/notifications/read', {
    method: 'PUT',
    credentials: 'include',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ ids: [id] }),
  });
  if (!res.ok) throw new Error(await extractApiError(res));
}

export async function markAllRead(): Promise<number> {
  const res = await fetch('/api/notifications/read_all', {
    method: 'PUT',
    credentials: 'include',
  });
  if (!res.ok) throw new Error(await extractApiError(res));
  const data = await res.json();
  return Number(data?.updated ?? 0);
}

/* shared small formatter (dd/mm/yyyy, hh:mm) */
export function formatUnix(sec?: number | null): string {
  if (!Number.isFinite(sec ?? NaN)) return '';
  const d = new Date((sec as number) * 1000);
  const dd = String(d.getDate()).padStart(2, '0');
  const mm = String(d.getMonth() + 1).padStart(2, '0');
  const yyyy = d.getFullYear();
  const hh = String(d.getHours()).padStart(2, '0');
  const min = String(d.getMinutes()).padStart(2, '0');
  return `${dd}/${mm}/${yyyy}, ${hh}:${min}`;
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
