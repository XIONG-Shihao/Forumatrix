// frontend/src/features/docs/api.ts
import { api } from '../../lib/api';

/* =======================
   Types
   ======================= */

export type DocumentRow = {
  id: number;
  owner_id: number;
  title: string;
  page_count: number;
  created_at: number; // unix seconds
  updated_at: number; // unix seconds
};

export type DocumentListResponse = {
  items: DocumentRow[];
  page: number;
  total_pages: number;
  total: number;
};

export type DocumentMeta = {
  id: number;
  owner_id: number;
  title: string;
  page_count: number;
  created_at: number;
  updated_at: number;
  pages: Array<{ page_index: number; style: number; updated_at: number }>;
};

export type PageOpenPayload = {
  doc_id: number;
  page_index: number;
  style: number;                 // 1=Title, 2=Heading, 3=Body
  y_update: number[];            // backend returns Vec<u8> -> JSON array of numbers
};

export type UpsertPageResponse = {
  updated: number;
  updated_at: number;            // unix seconds
};

export type MembersListItem = {
  user_id: number;
  role: number;                  // 3=owner, 2=editor
  added_at: number;              // unix seconds
};

export type MembersListResponse = {
  items: MembersListItem[];
};

/* =======================
   Documents: list / create / meta
   ======================= */

// GET /api/docs?page=&limit=
// (Lists docs where the caller is owner or editor)
export async function listMyDocs(
  page = 1,
  limit = 10
): Promise<DocumentListResponse> {
  const qs = new URLSearchParams({
    page: String(page),
    limit: String(limit),
  }).toString();
  // Backend route: GET /api/docs
  return api.get<DocumentListResponse>(`/api/docs?${qs}`);
}

// POST /api/docs
export async function createDoc(input: {
  title: string;
  page_count?: number; // defaults to 1 on the server if omitted
}): Promise<{ id: number }> {
  return api.post<{ id: number }>(`/api/docs`, {
    title: input.title,
    page_count: input.page_count ?? 1,
  });
}

// GET /api/docs/:doc_id
export async function getDocMeta(docId: number): Promise<DocumentMeta> {
  return api.get<DocumentMeta>(`/api/docs/${docId}`);
}

/* =======================
   Pages: open / upsert
   ======================= */

// GET /api/docs/:doc_id/pages/:page_index
export async function openPage(
  docId: number,
  pageIndex: number
): Promise<PageOpenPayload> {
  return api.get<PageOpenPayload>(`/api/docs/${docId}/pages/${pageIndex}`);
}

// PUT /api/docs/:doc_id/pages/:page_index
// Body requires a base64-encoded merged update (and optional style)
export async function upsertPage(
  docId: number,
  pageIndex: number,
  body: { style?: number; y_update_base64: string }
): Promise<UpsertPageResponse> {
  return api.put<UpsertPageResponse>(
    `/api/docs/${docId}/pages/${pageIndex}`,
    body
  );
}

/* =======================
   Membership (join requests & members)
   ======================= */

// POST /api/docs/:doc_id/join_requests
export async function createJoinRequest(
  docId: number,
  message?: string
): Promise<{ request_id: number }> {
  const payload = message ? { message } : {};
  return api.post<{ request_id: number }>(
    `/api/docs/${docId}/join_requests`,
    payload
  );
}

// GET /api/docs/:doc_id/members  (owner only)
export async function listMembers(
  docId: number
): Promise<MembersListResponse> {
  return api.get<MembersListResponse>(`/api/docs/${docId}/members`);
}

// POST /api/docs/requests/:req_id/approve  (owner only)
export async function approveJoinRequest(
  reqId: number
): Promise<{ ok: boolean }> {
  return api.post<{ ok: boolean }>(`/api/docs/requests/${reqId}/approve`, {});
}

// POST /api/docs/requests/:req_id/deny  (owner only)
export async function denyJoinRequest(
  reqId: number
): Promise<{ updated: number }> {
  return api.post<{ updated: number }>(`/api/docs/requests/${reqId}/deny`, {});
}

// DELETE /api/docs/:doc_id/members/:member_user_id  (owner only)
export async function removeMember(
  docId: number,
  userId: number
): Promise<{ removed: number }> {
  return api.del<{ removed: number }>(
    `/api/docs/${docId}/members/${userId}`
  );
}

/* =======================
   (Optional) helpers
   ======================= */

/**
 * Encode a UTF-8 string to base64 for upsertPage().
 * If you already have a Uint8Array, use:
 *   btoa(String.fromCharCode(...bytes))
 */
export function utf8ToBase64(s: string): string {
  // In modern browsers:
  return btoa(unescape(encodeURIComponent(s)));
}