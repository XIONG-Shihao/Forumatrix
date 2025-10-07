// frontend/src/pages/Home.tsx
import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';

import { Header } from '../components/ui/Header';
import { ComposePostModal } from '../components/posts/ComposePostModal';
import { FeedControls } from '../components/posts/FeedControls';
import { PostCard } from '../components/posts/PostCard';
import { NotificationsPanel } from '../components/notifications/NotificationsPanel';

import { getAuth } from '../features/auth/state';
import { getUser } from '../lib/api';
import type { UserPublic } from '../lib/types';

import { listPosts, type Sort, type PostListItem } from '../features/posts/api';
import { ChatFab } from '../components/chat/ChatFab';
import ChatPanel from '../components/chat/ChatPanel';

import { unreadCount as notifUnreadCount } from '../features/notifications/api';
import { unreadCount as chatUnreadCount } from '../features/chats/api';

const FEED_PAGE_SIZE = 5;

// inside HomePage()
const FEED = {
  cardPad: 16, // card padding
  cardMinH: 0, // min-height (0 = auto)
  avatar: 44, // avatar size
  username: 22, // username font
  time: 15, // time font
  title: 22, // title font
  body: 18, // body font
  chipFont: 14, // like/comment text
  icon: 18, // heart/bubble icon
  gap: 14, // gap between chips
  chipPadV: 0, // extra padding for like/comment row (vertical)
  chipPadH: 0, // extra padding for like/comment row (horizontal)
};

// CSS vars object (TS-friendly)
const feedVars: React.CSSProperties = {
  ['--feed-card-pad' as any]: `${FEED.cardPad}px`,
  ['--feed-card-minh' as any]: FEED.cardMinH ? `${FEED.cardMinH}px` : 'auto',
  ['--feed-avatar-size' as any]: `${FEED.avatar}px`,
  ['--feed-username-size' as any]: `${FEED.username}px`,
  ['--feed-time-size' as any]: `${FEED.time}px`,
  ['--feed-title-size' as any]: `${FEED.title}px`,
  ['--feed-body-size' as any]: `${FEED.body}px`,
  ['--feed-chip-font' as any]: `${FEED.chipFont}px`,
  ['--feed-icon-size' as any]: `${FEED.icon}px`,
  ['--feed-gap' as any]: `${FEED.gap}px`,
  ['--feed-chip-pad' as any]: `${FEED.chipPadV}px ${FEED.chipPadH}px`,
};

export default function HomePage() {
  const nav = useNavigate();
  // ---- Header user/avatar ----
  const auth = getAuth();
  const [me, setMe] = useState<UserPublic | null>(null);

  useEffect(() => {
    (async () => {
      if (!auth?.userId) return;
      try {
        setMe(await getUser(auth.userId));
      } catch {
        // ignore
      }
    })();
  }, [auth?.userId]);

  const avatarUrl =
    me?.avatar_url ?? (me ? `/static/avatars/${me.id}.png` : undefined);

  // ---- Feed state ----
  const [sort, setSort] = useState<Sort>('latest');
  const [page, setPage] = useState(1);
  const [totalPages, setTotalPages] = useState(1);
  const [items, setItems] = useState<PostListItem[]>([]);
  const [err, setErr] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [reloadTick, setReloadTick] = useState(0);
  const [chatOpen, setChatOpen] = useState(false);
  const [chatUnread, setChatUnread] = useState(0);

  // ---- Compose modal ----
  const [composeOpen, setComposeOpen] = useState(false);

  // ---- Notifications ----
  const [bellOpen, setBellOpen] = useState(false);
  const [notifCount, setNotifCount] = useState(0);

  // poll unread count (20s)
  useEffect(() => {
    let cancel = false;
    const load = async () => {
      try {
        const n = await notifUnreadCount();
        if (!cancel) setNotifCount(n);
      } catch {}
    };
    load();
    const id = setInterval(load, 20000);
    return () => {
      cancel = true;
      clearInterval(id);
    };
  }, []);

  // fetch posts whenever sort/page changes (or reload)
  useEffect(() => {
    let cancelled = false;
    (async () => {
      setLoading(true);
      setErr(null);
      try {
        const res = await listPosts(sort, page, FEED_PAGE_SIZE); // ← here
        if (!cancelled) {
          setItems(res.items);
          setTotalPages(res.total_pages || 1);
        }
      } catch (e: any) {
        if (!cancelled) setErr(e?.message ?? 'Failed to load posts');
      } finally {
        if (!cancelled) setLoading(false);
      }
    })();
    return () => {
      cancelled = true;
    };
  }, [sort, page, reloadTick]);

  useEffect(() => {
    chatUnreadCount()
      .then(setChatUnread)
      .catch(() => {});
  }, []);
  // pager handlers
  const first = () => setPage(1);
  const prev = () => setPage((p) => Math.max(1, p - 1));
  const next = () => setPage((p) => Math.min(totalPages, p + 1));
  const last = () => setPage(totalPages);
  const onSort = (s: Sort) => {
    setSort(s);
    setPage(1);
  };

  // after successful post: refresh first page of latest
  function handleCreated(_newId: number) {
    setComposeOpen(false);
    setSort('latest');
    if (page === 1) setReloadTick((t) => t + 1);
    else setPage(1);
    // Optionally navigate to the new post:
    // nav(`/posts/${_newId}`);
  }

  return (
    <>
      {/* Global header: show "+ Post" only on Home */}
      <Header
        showCompose
        onCompose={() => setComposeOpen(true)}
        avatarUrl={avatarUrl}
        username={me?.username}
        onBellClick={() => setBellOpen((v) => !v)}
      />

      {/* Notifications panel (toggles from Header bell) */}
      <NotificationsPanel
        open={bellOpen}
        onClose={() => setBellOpen(false)}
        onNavigatePost={(pid) => {
          setBellOpen(false);
          nav(`/posts/${pid}`);
        }}
        onAfterMarkAll={(updated) => {
          // keep a local count so future headers can show a badge
          setNotifCount((c) => Math.max(0, c - updated));
        }}
      />

      <main className="page" style={{ alignItems: 'start' }}>
        <div className="card" style={{ width: 'min(900px, 100%)' }}>
          <FeedControls
            sort={sort}
            onSort={onSort}
            page={page}
            totalPages={totalPages}
            onFirst={first}
            onPrev={prev}
            onNext={next}
            onLast={last}
          />

          {err && (
            <div className="error" style={{ marginTop: 12 }}>
              {err}
            </div>
          )}

          <div style={{ display: 'grid', gap: 12, marginTop: 12, ...feedVars }}>
            {loading && <div>Loading…</div>}
            {!loading && items.length === 0 && <div>No posts yet.</div>}
            {items.map((p) => (
              <PostCard
                key={p.id}
                post={p}
                onClick={(id) => nav(`/posts/${id}`)}
              />
            ))}
          </div>
        </div>
      </main>
      {/* Chat button + panel */}
      <ChatFab
        onClick={() => setChatOpen(true)}
        count={chatUnread}
        size={72}
        fontSize={36}
      />
      <ChatPanel
        open={chatOpen}
        onClose={() => setChatOpen(false)}
        onUnreadChange={setChatUnread}
      />
      {/* Compose modal */}
      <ComposePostModal
        open={composeOpen}
        onClose={() => setComposeOpen(false)}
        onCreated={handleCreated}
      />
    </>
  );
}
