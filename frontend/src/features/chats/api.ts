// frontend/src/features/chats/api.ts
import { api } from '../../lib/api';
import { getAuth } from '../auth/state';

/* ---------- Types ---------- */

export type ChatListItem = {
  id: number;
  peer_id: number;
  peer_username: string;
  peer_avatar_url?: string | null;
  last_message_at?: number | null;
  created_at: number;
  unread_count?: number; // <-- backend should include this per chat
};

export type Message = {
  id: number;
  chat_id: number;
  sender_id: number;
  body: string;
  created_at: number;
};

// Raw message row as returned by the backend (ciphertext bytes)
type RawMessageRow = {
  id: number;
  chat_id: number;
  sender_id: number;
  nonce: number[] | Uint8Array;
  ciphertext: number[] | Uint8Array;
  created_at: number;
  read_at?: number | null;
};

type ListChatsResponse = {
  items: ChatListItem[];
  page: number;
  has_more: boolean;
};

type ListMessagesRawResponse = {
  items: RawMessageRow[];
  page: number;
  has_more: boolean;
};

type SendMessageRawResponse = {
  message: RawMessageRow;
};

/* ---------- Helpers ---------- */

function decodeBody(bytes: number[] | Uint8Array): string {
  try {
    const arr =
      bytes instanceof Uint8Array ? bytes : Uint8Array.from(bytes ?? []);
    return new TextDecoder().decode(arr);
  } catch {
    return '[unsupported]';
  }
}

export function myUserId(): number | null {
  return getAuth()?.userId ?? null;
}

/* ---------- API Calls ---------- */

export async function listChats(
  page = 1,
  limit = 5
): Promise<ListChatsResponse> {
  const res = (await api.get(
    `/api/chats?page=${page}&limit=${limit}`
  )) as unknown as ListChatsResponse;
  return res;
}

export async function listMessages(
  chatId: number,
  page = 1,
  limit = 30
): Promise<{ items: Message[]; page: number; has_more: boolean }> {
  const raw = (await api.get(
    `/api/chats/${chatId}/messages?page=${page}&limit=${limit}`
  )) as unknown as ListMessagesRawResponse;

  const items: Message[] = (raw.items ?? []).map((r) => ({
    id: r.id,
    chat_id: r.chat_id,
    sender_id: r.sender_id,
    body: decodeBody(r.ciphertext),
    created_at: r.created_at,
  }));

  return { items, page: raw.page, has_more: raw.has_more };
}

export async function sendMessage(
  chatId: number,
  text: string
): Promise<Message> {
  const raw = (await api.post(`/api/chats/${chatId}/messages`, {
    text,
  })) as unknown as SendMessageRawResponse;

  const m = raw.message;
  return {
    id: m.id,
    chat_id: m.chat_id,
    sender_id: m.sender_id,
    body: decodeBody(m.ciphertext),
    created_at: m.created_at,
  };
}

export async function openChat(
  peerId: number
): Promise<{ chat_id: number; peer_id: number }> {
  const res = await api.post<{ chat_id: number; peer_id: number }>(
    '/api/chats/open',
    { peer_id: peerId }
  );
  return res;
}

/* ---------- Unread APIs ---------- */

export async function unreadCount(): Promise<number> {
  const res = (await api.get('/api/chats/unread_count')) as unknown as {
    unread: number;
  };
  return res.unread ?? 0;
}

export async function markChatRead(chatId: number): Promise<number> {
  const res = (await api.put(`/api/chats/${chatId}/read`, {})) as unknown as {
    updated: number;
  };
  return res.updated ?? 0;
}
