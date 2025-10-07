// frontend/src/components/chat/ChatPanel.tsx
import React, { useEffect, useMemo, useRef, useState } from 'react';
import { getAuth } from '../../features/auth/state';
import {
  listChats,
  listMessages,
  sendMessage,
  markChatRead,
  unreadCount,
  type ChatListItem,
  type Message,
} from '../../features/chats/api';
import { Avatar } from '../ui/Avatar';
import { formatUnix } from '../../features/posts/api';
import { useNavigate } from 'react-router-dom';

const SIZES = {
  pager: { text: 16, btnSize: 28, btnFont: 14 }, // Page n/m & «‹›»
  headerName: 18, // username above messages
  messageFont: 18, // bubble text
  bubblePadY: 10,
  bubblePadX: 14,
  sendBtn: { font: 16, padY: 10, padX: 16, minW: 70 },
  input: { font: 16, padY: 10, padX: 12 }, // composer input
};

type Props = {
  open: boolean;
  onClose: () => void;
  /** If provided (e.g. from “Start chat”), we’ll show that chat even if it’s not on page 1. */
  initialChatId?: number | null;
  /** Title fallback while we’re still paging to find the chat. */
  fallbackPeer?: { username: string; avatar_url?: string };
  /** Bubble up total unread so the 💬 FAB badge can update. */
  onUnreadChange?: (n: number) => void;
};

export default function ChatPanel({
  open,
  onClose,
  initialChatId = null,
  fallbackPeer,
  onUnreadChange,
}: Props) {
  const nav = useNavigate();
  const auth = getAuth();
  const myId = auth?.userId ?? 0;
  const goProfile = (uid: number) =>
    nav(uid === auth?.userId ? '/profile' : `/users/${uid}`);

  // left list
  const [chats, setChats] = useState<ChatListItem[]>([]);
  const [chatPage, setChatPage] = useState(1);
  const [chatHasMore, setChatHasMore] = useState(false);
  const [listLoading, setListLoading] = useState(false);
  const [listErr, setListErr] = useState<string | null>(null);

  // unread total
  const [totalUnread, setTotalUnread] = useState(0);

  // active chat + messages
  const [activeChatId, setActiveChatId] = useState<number | null>(null);
  const activeChat = useMemo(
    () => chats.find((c) => c.id === activeChatId) ?? null,
    [chats, activeChatId]
  );

  const [messages, setMessages] = useState<Message[]>([]);
  const [msgLoading, setMsgLoading] = useState(false);
  const [msgErr, setMsgErr] = useState<string | null>(null);

  // composer
  const [input, setInput] = useState('');
  const [sending, setSending] = useState(false);

  const hasChats = chats.length > 0;
  const canType = hasChats && activeChatId != null;

  const scrollRef = useRef<HTMLDivElement>(null);

  // derived page display
  const totalPageDisplay = chatHasMore ? chatPage + 1 : chatPage;

  /* ---------------- helpers ---------------- */

  async function refreshUnread() {
    try {
      const n = await unreadCount();
      setTotalUnread(n);
      onUnreadChange?.(n);
    } catch {
      // ignore
    }
  }

  async function loadChatsPage(page = 1) {
    setListLoading(true);
    setListErr(null);
    try {
      const res = await listChats(page, 5);
      setChats(res.items);
      setChatHasMore(res.has_more);
      setChatPage(page);
      return res;
    } catch (e: any) {
      setListErr(e?.message ?? 'Failed to load chats');
      return { items: [] as ChatListItem[], page, has_more: false };
    } finally {
      setListLoading(false);
    }
  }

  /** Ensure a given chat appears in the left list, paging forward if needed. */
  async function ensureChatVisible(targetId: number) {
    // start from page 1
    let page = 1;
    let res = await loadChatsPage(page);
    let found = res.items.some((c) => c.id === targetId);

    while (!found && res.has_more) {
      page += 1;
      res = await loadChatsPage(page);
      found = res.items.some((c) => c.id === targetId);
    }

    setActiveChatId(targetId);
  }

  // fetch messages for active chat
  async function loadMessages(chatId: number) {
    setMsgLoading(true);
    setMsgErr(null);
    try {
      const res = await listMessages(chatId, 1, 50);
      setMessages(res.items.slice().reverse()); // oldest → newest
      setTimeout(() => scrollRef.current?.scrollTo({ top: 9e9 }), 0);
    } catch (e: any) {
      setMsgErr(e?.message ?? 'Failed to load messages');
    } finally {
      setMsgLoading(false);
    }
  }

  async function selectChat(id: number) {
    setActiveChatId(id);
    // mark as read & refresh badges
    try {
      const updated = await markChatRead(id);
      if (updated > 0) {
        setChats((prev) =>
          prev.map((c) => (c.id === id ? { ...c, unread_count: 0 } : c))
        );
        await refreshUnread();
      }
    } catch {
      // ignore
    }
  }

  /* ---------------- effects ---------------- */

  // open/close/init
  useEffect(() => {
    if (!open) return;
    refreshUnread();

    (async () => {
      if (initialChatId) {
        await ensureChatVisible(initialChatId);
        setActiveChatId(initialChatId); // messages load via effect
      } else {
        const res = await loadChatsPage(1);
        const firstId = res.items[0]?.id ?? null;
        setActiveChatId(firstId); // messages load via effect
      }
    })();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [open, initialChatId]);

  // when active chat changes and we didn’t load explicitly yet
  useEffect(() => {
    if (!activeChatId) {
      setMessages([]);
      return;
    }
    let cancelled = false;

    setMsgLoading(true);
    setMsgErr(null);

    listMessages(activeChatId, 1, 50)
      .then((res) => {
        if (cancelled) return;
        setMessages(res.items.slice().reverse()); // oldest -> newest
        // scroll to bottom after render
        setTimeout(() => scrollRef.current?.scrollTo({ top: 9e9 }), 0);
      })
      .catch((e: any) => {
        if (cancelled) return;
        setMsgErr(e?.message ?? 'Failed to load messages');
      })
      .finally(() => {
        if (cancelled) return;
        setMsgLoading(false);
      });

    return () => {
      cancelled = true;
    };
  }, [activeChatId]);

  async function onSend(e: React.FormEvent) {
    e.preventDefault();
    if (!canType) return;
    const body = input.trim();
    if (!body || sending) return;

    setSending(true);
    try {
      await sendMessage(activeChatId!, body);
      setInput('');
      await loadMessages(activeChatId!);
      // refresh list + unread totals so ordering & badges update
      await loadChatsPage(chatPage);
      await refreshUnread();
    } catch {
      // ignore; surfaced by API
    } finally {
      setSending(false);
    }
  }

  if (!open) return null;

  return (
    <div style={overlay}>
      <div style={panel}>
        {/* Header */}
        <div style={hdr}>
          <div style={{ fontWeight: 800 }}>Chats ({totalUnread})</div>
          <button title="Close" onClick={onClose} style={closeBtn}>
            ×
          </button>
        </div>

        {/* Body grid */}
        <div style={grid}>
          {/* Left: chat list */}
          <div style={leftCol}>
            {listErr && (
              <div className="error" style={{ marginBottom: 8 }}>
                {listErr}
              </div>
            )}
            {listLoading ? (
              <div style={{ opacity: 0.8 }}>Loading…</div>
            ) : chats.length === 0 ? (
              <div style={{ opacity: 0.8 }}>You do not have any message.</div>
            ) : (
              <div style={listGrid}>
                {chats.map((c) => (
                  <ChatCard
                    key={c.id}
                    item={c}
                    active={c.id === activeChatId}
                    onClick={() => selectChat(c.id)}
                  />
                ))}
              </div>
            )}

            {/* Pager */}
            <div style={pagerRow}>
              <div style={{ fontSize: SIZES.pager.text, opacity: 0.9 }}>
                Page {chatPage} / {totalPageDisplay}
              </div>
              <div style={{ display: 'flex', gap: 6 }}>
                <PagerBtn
                  onClick={() => loadChatsPage(1)}
                  disabled={chatPage <= 1}
                >
                  &laquo;
                </PagerBtn>
                <PagerBtn
                  onClick={() => loadChatsPage(Math.max(1, chatPage - 1))}
                  disabled={chatPage <= 1}
                >
                  &lsaquo;
                </PagerBtn>
                <PagerBtn
                  onClick={() =>
                    loadChatsPage(chatHasMore ? chatPage + 1 : chatPage)
                  }
                  disabled={!chatHasMore}
                >
                  &rsaquo;
                </PagerBtn>
                <PagerBtn
                  onClick={() =>
                    chatHasMore ? loadChatsPage(chatPage + 1) : undefined
                  }
                  disabled={!chatHasMore}
                >
                  &raquo;
                </PagerBtn>
              </div>
            </div>
          </div>

          {/* Right: message area */}
          <div style={rightCol}>
            <div style={convHdr}>
              {activeChat ? (
                <button
                  onClick={() => goProfile(activeChat.peer_id)}
                  title="View profile"
                  style={{
                    background: 'transparent',
                    border: 'none',
                    color: 'var(--text)',
                    fontWeight: 700,
                    cursor: 'pointer',
                    padding: 0,
                    fontSize: SIZES.headerName,
                  }}
                >
                  {activeChat.peer_username}
                </button>
              ) : (
                <div style={{ fontWeight: 700 }}>—</div>
              )}
            </div>

            <div ref={scrollRef} style={msgsWrap}>
              {msgErr && (
                <div className="error" style={{ marginBottom: 8 }}>
                  {msgErr}
                </div>
              )}
              {!hasChats ? (
                <div style={{ opacity: 0.7, padding: 20 }}>
                  Start a conversation to see messages here.
                </div>
              ) : msgLoading ? (
                <div style={{ opacity: 0.8 }}>Loading…</div>
              ) : messages.length === 0 ? (
                <div style={{ opacity: 0.7, padding: 20 }}>
                  No messages yet.
                </div>
              ) : (
                <div style={{ display: 'grid', gap: 10 }}>
                  {messages.map((m) => (
                    <Bubble key={m.id} self={m.sender_id === myId}>
                      {m.body}
                    </Bubble>
                  ))}
                </div>
              )}
            </div>

            {/* Composer */}
            <form onSubmit={onSend} style={composerRow}>
              <input
                className="input"
                value={input}
                onChange={(e) => setInput(e.target.value)}
                placeholder={
                  hasChats ? 'Type a message (max 200 chars)…' : 'No chats yet'
                }
                maxLength={200}
                disabled={!canType}
                style={{
                  flex: 1,
                  opacity: canType ? 1 : 0.6,
                  background: '#0b1020',
                  fontSize: SIZES.input.font,
                  padding: `${SIZES.input.padY}px ${SIZES.input.padX}px`,
                  lineHeight: 1.25, // optional; helps vertical rhythm
                }}
              />
              <button
                className="btn"
                type="submit"
                disabled={!canType || sending || !input.trim()}
                style={{
                  width: 'auto',
                  padding: `${SIZES.sendBtn.padY}px ${SIZES.sendBtn.padX}px`,
                  marginTop: 0,
                  fontSize: SIZES.sendBtn.font,
                  minWidth: SIZES.sendBtn.minW,
                }}
              >
                Send
              </button>
            </form>
          </div>
        </div>
      </div>
    </div>
  );
}

/* ---------------- subcomponents ---------------- */

function ChatCard({
  item,
  active,
  onClick,
}: {
  item: ChatListItem;
  active: boolean;
  onClick: () => void;
}) {
  const when = item.last_message_at ? formatUnix(item.last_message_at) : '';

  return (
    <button
      onClick={onClick}
      style={{
        textAlign: 'left',
        border: '1px solid ' + (active ? '#3a4ca0' : 'var(--border)'),
        background: active ? 'rgba(79,70,229,.12)' : 'transparent',
        color: 'var(--text)',
        padding: 14,
        borderRadius: 12,
        cursor: 'pointer',
        display: 'grid',
        gridTemplateColumns: '36px 1fr',
        gap: 10,
        width: '100%',
        height: 120, // fixed height so rows don’t stretch
        position: 'relative',
      }}
      title={item.peer_username}
    >
      <Avatar
        src={item.peer_avatar_url ?? undefined}
        name={item.peer_username}
        size={40}
      />
      <div style={{ display: 'grid', alignContent: 'center' }}>
        <div
          style={{
            fontWeight: 700,
            paddingRight: 72, // ← room for the time
            whiteSpace: 'nowrap',
            overflow: 'hidden',
            textOverflow: 'ellipsis',
            fontSize: 18,
          }}
        >
          {item.peer_username}
        </div>
        <div
          style={{
            position: 'absolute',
            top: 8, // ← tweak to taste
            right: 12, // ← "
            fontSize: 13,
            color: '#97a3c7',
            pointerEvents: 'none', // click stays on the card
          }}
        >
          {when}
        </div>
      </div>

      {/* unread badge */}
      {!!item.unread_count && item.unread_count > 0 && (
        <div
          style={{
            position: 'absolute',
            right: 12,
            bottom: 8,
            background: '#e11d48',
            color: 'white',
            borderRadius: 10,
            padding: '2px 6px',
            fontSize: 11,
            fontWeight: 700,
          }}
        >
          {item.unread_count}
        </div>
      )}
    </button>
  );
}

function Bubble({
  children,
  self,
}: {
  children: React.ReactNode;
  self?: boolean;
}) {
  return (
    <div
      style={{
        display: 'flex',
        justifyContent: self ? 'flex-end' : 'flex-start',
      }}
    >
      <div
        style={{
          maxWidth: '70%',
          padding: `${SIZES.bubblePadY}px ${SIZES.bubblePadX}px`,
          borderRadius: 999,
          border: '1px solid var(--border)',
          background: self ? '#313d7a' : '#1a2242',
          color: 'var(--text)',
          lineHeight: 1.35,
          fontSize: SIZES.messageFont,
        }}
      >
        {children}
      </div>
    </div>
  );
}

/* ---------------- styles ---------------- */

const overlay: React.CSSProperties = {
  position: 'fixed',
  inset: 0,
  background: 'rgba(0,0,0,.35)',
  display: 'grid',
  placeItems: 'center',
  zIndex: 60,
  padding: 12,
};

const panel: React.CSSProperties = {
  width: 'min(1000px, 96vw)',
  height: 'min(72vh, 80rem)',
  background: 'var(--bg-2)',
  border: '1px solid var(--border)',
  borderRadius: 14,
  boxShadow: '0 18px 50px rgba(0,0,0,.45)',
  display: 'grid',
  gridTemplateRows: 'auto 1fr',
};

const hdr: React.CSSProperties = {
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'space-between',
  padding: '10px 12px',
  borderBottom: '1px solid var(--border)',
};

const closeBtn: React.CSSProperties = {
  border: '1px solid var(--border)',
  background: 'transparent',
  color: 'var(--text)',
  borderRadius: 8,
  padding: '4px 8px',
  cursor: 'pointer',
};

const grid: React.CSSProperties = {
  display: 'grid',
  gridTemplateColumns: '340px 1fr',
  gap: 12,
  padding: 12,
  minHeight: 0,
};

const leftCol: React.CSSProperties = {
  display: 'grid',
  gridTemplateRows: '1fr auto',
  gap: 10,
  minHeight: 0,
  borderRight: '1px solid var(--border)',
  paddingRight: 10,
};

const listGrid: React.CSSProperties = {
  display: 'grid',
  gap: 8,
  alignContent: 'start', // <- prevent rows from stretching
  gridAutoRows: 'min-content', // <- each row sized by its content
  position: 'relative',
};

const pagerRow: React.CSSProperties = {
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'space-between',
  gap: 8,
  paddingTop: 6,
};

const rightCol: React.CSSProperties = {
  display: 'grid',
  gridTemplateRows: 'auto 1fr auto',
  gap: 10,
  minHeight: 0,
};

const convHdr: React.CSSProperties = {
  padding: '6px 4px',
  borderBottom: '1px solid var(--border)',
  minHeight: 34,
  display: 'flex',
  alignItems: 'center',
};

const msgsWrap: React.CSSProperties = {
  overflow: 'auto',
  padding: 8,
  background: '#0e1430',
  border: '1px solid var(--border)',
  borderRadius: 8,
};

const composerRow: React.CSSProperties = {
  display: 'flex',
  alignItems: 'center',
  gap: 8,
};

function PagerBtn(props: React.ButtonHTMLAttributes<HTMLButtonElement>) {
  return (
    <button
      {...props}
      style={{
        border: '1px solid var(--border)',
        background: 'transparent',
        color: 'var(--text)',
        borderRadius: 8,
        cursor: props.disabled ? 'default' : 'pointer',
        opacity: props.disabled ? 0.5 : 1,
        width: SIZES.pager.btnSize,
        height: SIZES.pager.btnSize,
        fontSize: SIZES.pager.btnFont,
        lineHeight: 1,
        display: 'grid',
        placeItems: 'center',
        padding: 0, // keep the square
      }}
    />
  );
}
