// Unified posts+comments client helpers

export type Sort = 'latest' | 'popular' | 'controversial';

/* ---------- Posts (list & detail) ---------- */

export type PostListItem = {
  id: number;
  user_id: number;
  title: string;
  body: string;
  created_at: number; // unix seconds
  updated_at?: number | null; // unix seconds or null
  edited: number;
  score: number;
  comment_count: number;
  author_username: string;
  author_avatar_url?: string | null;
  liked_by_me?: boolean; // present on list endpoint
};

export type PostListResponse = {
  items: PostListItem[];
  page: number;
  total_pages: number;
  total: number;
};

export async function listPosts(
  sort: Sort,
  page = 1,
  limit = 7
): Promise<PostListResponse> {
  const u = new URL('/api/posts', window.location.origin);
  u.searchParams.set('sort', sort);
  u.searchParams.set('page', String(page));
  u.searchParams.set('limit', String(limit));
  const res = await fetch(u.toString(), { credentials: 'include' });
  if (!res.ok) throw new Error(await extractApiError(res));
  return res.json();
}

export type PostDetail = PostListItem & {
  liked_by_me: boolean; // detail should always give this
};

export async function getPost(id: number): Promise<PostDetail> {
  const res = await fetch(`/api/posts/${id}`, { credentials: 'include' });
  if (!res.ok) throw new Error(await extractApiError(res));
  return res.json();
}

/* ---------- Create post ---------- */
export async function createPost(input: {
  title: string;
  body: string;
}): Promise<{ id: number }> {
  const res = await fetch('/api/posts', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    credentials: 'include',
    body: JSON.stringify(input),
  });
  if (!res.ok) throw new Error(await extractApiError(res));
  return res.json(); // { id }
}

/* ---------- Post likes ---------- */

export async function likePost(
  postId: number
): Promise<{ liked: true; score: number }> {
  const res = await fetch(`/api/posts/${postId}/like`, {
    method: 'PUT',
    credentials: 'include',
  });
  if (!res.ok) throw new Error(await extractApiError(res));
  return res.json();
}

export async function unlikePost(
  postId: number
): Promise<{ liked: false; score: number }> {
  const res = await fetch(`/api/posts/${postId}/like`, {
    method: 'DELETE',
    credentials: 'include',
  });
  if (!res.ok) throw new Error(await extractApiError(res));
  return res.json();
}

/* ---------- Comments (list, create, like) ---------- */

export type CommentItem = {
  id: number;
  post_id: number;
  user_id: number;
  parent_id?: number | null;
  body: string;
  created_at: number; // unix seconds
  updated_at?: number | null;
  deleted_at?: number | null;
  edited: number;
  score: number;
  author_username: string;
  author_avatar_url?: string | null;
  parent_author_username?: string | null;
  liked_by_me: boolean;
  reply_count: number;
  author_is_admin?: boolean;
};

export type CommentListResponse = {
  items: CommentItem[];
  page: number;
  total_pages: number;
  total: number;
};

export async function listComments(
  postId: number,
  sort: 'created' | 'score' = 'created',
  page = 1,
  limit = 100
): Promise<CommentListResponse> {
  const u = new URL(`/api/posts/${postId}/comments`, window.location.origin);
  u.searchParams.set('sort', sort);
  u.searchParams.set('page', String(page));
  u.searchParams.set('limit', String(limit));
  const res = await fetch(u.toString(), { credentials: 'include' });
  if (!res.ok) throw new Error(await extractApiError(res));
  return res.json();
}

export async function createComment(
  postId: number,
  body: string,
  parent_id?: number
): Promise<{ id: number }> {
  const res = await fetch(`/api/posts/${postId}/comments`, {
    method: 'POST',
    credentials: 'include',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ body, parent_id }),
  });
  if (!res.ok) throw new Error(await extractApiError(res));
  return res.json();
}

export async function likeComment(
  commentId: number
): Promise<{ liked: true; score: number }> {
  const res = await fetch(`/api/comments/${commentId}/like`, {
    method: 'PUT',
    credentials: 'include',
  });
  if (!res.ok) throw new Error(await extractApiError(res));
  return res.json();
}

export async function unlikeComment(
  commentId: number
): Promise<{ liked: false; score: number }> {
  const res = await fetch(`/api/comments/${commentId}/like`, {
    method: 'DELETE',
    credentials: 'include',
  });
  if (!res.ok) throw new Error(await extractApiError(res));
  return res.json();
}

/** Soft-delete a comment. Admin may pass `reason`. */
export async function deleteComment(
  commentId: number,
  reason?: string
): Promise<{
  comment_id: number;
  deleted_at: number;
  deleted_by: number;
  deleted_reason?: string;
}> {
  const res = await fetch(`/api/comments/${commentId}/delete`, {
    method: 'POST',
    credentials: 'include',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(reason ? { reason } : {}),
  });
  if (!res.ok) {
    // Reuse your helper from this file
    throw new Error(await extractApiError(res));
  }
  return res.json();
}
/*---------- List My Comments ---------- */
export type MyCommentItem = {
  id: number;
  post_id: number;
  body: string;
  created_at: number;
  updated_at?: number | null;
  deleted_at?: number | null;
  edited: number;
  score: number;
  post_title: string;
};

export type MyCommentListResponse = {
  items: MyCommentItem[];
  page: number;
  total_pages: number;
  total: number;
};

export async function listCommentsByUser(
  userId: number,
  page = 1,
  limit = 5
): Promise<MyCommentListResponse> {
  const u = new URL(`/api/users/${userId}/comments`, window.location.origin);
  u.searchParams.set('page', String(page));
  u.searchParams.set('limit', String(limit));
  const res = await fetch(u.toString(), { credentials: 'include' });
  if (!res.ok) throw new Error(await extractApiError(res));
  return res.json();
}

/* ---------- shared ---------- */

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

/* ---------- List User Posts---------- */
export async function listPostsByUser(
  userId: number,
  page = 1,
  limit = 5
): Promise<PostListResponse> {
  const u = new URL(`/api/users/${userId}/posts`, window.location.origin);
  u.searchParams.set('page', String(page));
  u.searchParams.set('limit', String(limit)); // 5 per page as requested
  const res = await fetch(u.toString(), { credentials: 'include' });
  if (!res.ok) throw new Error(await extractApiError(res));
  return res.json();
}

/* ---------- List Liked Posts ---------- */
export async function listLikedPostsByUser(
  userId: number,
  page = 1,
  limit = 5
): Promise<PostListResponse> {
  const u = new URL(`/api/users/${userId}/liked_posts`, window.location.origin);
  u.searchParams.set('page', String(page));
  u.searchParams.set('limit', String(limit));
  const res = await fetch(u.toString(), { credentials: 'include' });
  if (!res.ok) throw new Error(await extractApiError(res));
  return res.json();
}

// DELETE (soft) a post. Admin may pass a reason.
export async function deletePost(
  postId: number,
  reason?: string
): Promise<{
  post_id: number;
  deleted_at: number;
  deleted_by: number;
  deleted_reason?: string;
}> {
  const res = await fetch(`/api/posts/${postId}/delete`, {
    method: 'POST',
    credentials: 'include',
    headers: reason ? { 'Content-Type': 'application/json' } : undefined,
    body: reason ? JSON.stringify({ reason }) : undefined,
  });
  if (!res.ok) throw new Error(await extractApiError(res));
  return res.json();
}
