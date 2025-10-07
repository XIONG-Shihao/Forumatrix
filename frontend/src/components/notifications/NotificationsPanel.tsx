import React, { useEffect, useRef, useState } from 'react';
import {
  listNotifications,
  markAllRead,
  markRead,
  type NotificationItem,
  formatUnix,
} from '../../features/notifications/api';

type Props = {
  open: boolean;
  onClose: () => void;
  onNavigatePost: (postId: number) => void;
  onAfterMarkAll?: (updated: number) => void;
};

export function NotificationsPanel({
  open,
  onClose,
  onNavigatePost,
  onAfterMarkAll,
}: Props) {
  const [items, setItems] = useState<NotificationItem[]>([]);
  const [page, setPage] = useState(1);
  const [totalPages, setTotalPages] = useState(1);
  const [loading, setLoading] = useState(false);
  const [err, setErr] = useState<string | null>(null);
  const wrapRef = useRef<HTMLDivElement>(null);

  // close on outside click
  useEffect(() => {
    if (!open) return;
    const onDown = (e: MouseEvent) => {
      if (wrapRef.current && !wrapRef.current.contains(e.target as Node)) {
        onClose();
      }
    };
    document.addEventListener('mousedown', onDown);
    return () => document.removeEventListener('mousedown', onDown);
  }, [open, onClose]);

  // fetch when opened or page changes
  useEffect(() => {
    if (!open) return;
    let cancelled = false;
    (async () => {
      setLoading(true);
      setErr(null);
      try {
        const res = await listNotifications(page, 20);
        if (!cancelled) {
          setItems(res.items);
          setTotalPages(res.total_pages || 1);
        }
      } catch (e: any) {
        if (!cancelled) setErr(e?.message ?? 'Failed to load notifications');
      } finally {
        if (!cancelled) setLoading(false);
      }
    })();
    return () => {
      cancelled = true;
    };
  }, [open, page]);

  if (!open) return null;

  const first = () => setPage(1);
  const prev = () => setPage((p) => Math.max(1, p - 1));
  const next = () => setPage((p) => Math.min(totalPages, p + 1));
  const last = () => setPage(totalPages);

  async function onMarkAll() {
    try {
      const n = await markAllRead();
      onAfterMarkAll?.(n);
      // mark locally as read
      setItems((prev) =>
        prev.map((it) => ({ ...it, read_at: it.read_at ?? Date.now() / 1000 }))
      );
    } catch (e) {
      // ignore; error banner already visible if list failed
    }
  }

  return (
    <div
      style={{
        position: 'fixed',
        top: 56, // under the header buttons
        right: 16,
        zIndex: 40,
      }}
    >
      <div
        ref={wrapRef}
        className="card"
        style={{
          width: 380,
          maxHeight: '72vh',
          overflow: 'auto',
          padding: 12,
          borderRadius: 14,
          background: '#121831',
          border: '1px solid #2a3760',
          boxShadow: '0 18px 50px rgba(0,0,0,.45)',
        }}
      >
        <div
          style={{
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'space-between',
            marginBottom: 8,
          }}
        >
          <div style={{ fontWeight: 800 }}>Notifications</div>
          <div style={{ display: 'flex', gap: 8 }}>
            <button
              className="btn"
              onClick={onMarkAll}
              style={{
                width: 'auto',
                padding: '6px 10px',
                borderRadius: 10,
                background: 'transparent',
                border: '1px solid #2a3760',
                color: '#e6eaff',
              }}
            >
              Mark all read
            </button>
            <button
              className="btn"
              onClick={onClose}
              style={{
                width: 'auto',
                padding: '6px 10px',
                borderRadius: 10,
                background: '#5163ff',
                border: '1px solid #5163ff',
                color: 'white',
              }}
            >
              Close
            </button>
          </div>
        </div>

        {err && (
          <div className="error" style={{ marginBottom: 8 }}>
            {err}
          </div>
        )}
        {loading && <div style={{ padding: 8 }}>Loading…</div>}
        {!loading && items.length === 0 && (
          <div style={{ padding: 8, opacity: 0.8 }}>No notifications yet.</div>
        )}

        <div style={{ display: 'grid', gap: 8 }}>
          {items.map((n) => (
            <NotificationRow
              key={n.id}
              n={n}
              onClick={async () => {
                try {
                  await markRead(n.id);
                  setItems((prev) =>
                    prev.map((it) =>
                      it.id === n.id
                        ? {
                            ...it,
                            read_at:
                              it.read_at ?? Math.floor(Date.now() / 1000),
                          }
                        : it
                    )
                  );
                } catch {}
                if (n.post_id) onNavigatePost(n.post_id);
              }}
            />
          ))}
        </div>

        <div
          style={{
            marginTop: 10,
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'space-between',
            gap: 8,
          }}
        >
          <div style={{ fontSize: 12, opacity: 0.8 }}>
            Page {page} / {Math.max(1, totalPages)}
          </div>
          <div style={{ display: 'flex', gap: 6 }}>
            <PagerBtn onClick={first} disabled={page <= 1}>
              &laquo;
            </PagerBtn>
            <PagerBtn onClick={prev} disabled={page <= 1}>
              &lsaquo;
            </PagerBtn>
            <PagerBtn onClick={next} disabled={page >= totalPages}>
              &rsaquo;
            </PagerBtn>
            <PagerBtn onClick={last} disabled={page >= totalPages}>
              &raquo;
            </PagerBtn>
          </div>
        </div>
      </div>
    </div>
  );
}

function NotificationRow({
  n,
  onClick,
}: {
  n: NotificationItem;
  onClick: () => void;
}) {
  const when = formatUnix(n.created_at);
  const text = renderText(n);
  const seen = !!n.read_at;
  const avatar = n.actor_avatar_url ?? '';

  return (
    <button
      onClick={onClick}
      style={{
        textAlign: 'left',
        border: '1px solid ' + (seen ? '#223058' : '#3a4ca0'),
        background: seen ? '#101735' : '#16204a',
        color: '#e6eaff',
        padding: 10,
        borderRadius: 12,
        display: 'grid',
        gridTemplateColumns: '36px 1fr',
        gap: 10,
        cursor: 'pointer',
      }}
    >
      <div
        style={{
          width: 36,
          height: 36,
          borderRadius: '50%',
          overflow: 'hidden',
          border: '1px solid #2a3760',
          display: 'grid',
          placeItems: 'center',
          background: '#4f46e5',
          fontWeight: 800,
        }}
      >
        {avatar ? (
          <img
            src={avatar}
            alt=""
            onError={(e) =>
              ((e.target as HTMLImageElement).style.display = 'none')
            }
            style={{ width: '100%', height: '100%', objectFit: 'cover' }}
          />
        ) : (
          <span>{n.actor_username?.[0]?.toUpperCase() ?? '?'}</span>
        )}
      </div>

      <div style={{ display: 'grid' }}>
        <div style={{ lineHeight: 1.3 }}>{text}</div>
        <div style={{ fontSize: 12, color: '#97a3c7', marginTop: 4 }}>
          {when}
        </div>
      </div>
    </button>
  );
}

function renderText(n: NotificationItem): React.ReactNode {
  // Required strings:
  // 1) "$user liked your post $title"
  // 2) "$user liked your comment at post $title"
  // 3) "$user replied your post $title"
  // 4) "$user replied your comment at post $title"
  const strong = (t: string) => <span style={{ fontWeight: 700 }}>{t}</span>;
  const title = `“${n.post_title}”`;
  switch (n.kind) {
    case 'post_liked':
      return (
        <>
          {strong(n.actor_username)} liked your post {strong(title)}
        </>
      );
    case 'comment_liked':
      return (
        <>
          {strong(n.actor_username)} liked your comment at post {strong(title)}
        </>
      );
    case 'post_replied':
      return (
        <>
          {strong(n.actor_username)} replied your post {strong(title)}
        </>
      );
    case 'comment_replied':
      return (
        <>
          {strong(n.actor_username)} replied your comment at post{' '}
          {strong(title)}
        </>
      );
    default:
      return (
        <>
          {strong(n.actor_username)} did something on {strong(title)}
        </>
      );
  }
}

function PagerBtn(props: React.ButtonHTMLAttributes<HTMLButtonElement>) {
  return (
    <button
      {...props}
      style={{
        border: '1px solid #2a3760',
        background: 'transparent',
        color: '#e6eaff',
        padding: '4px 8px',
        borderRadius: 8,
        cursor: props.disabled ? 'default' : 'pointer',
        opacity: props.disabled ? 0.5 : 1,
      }}
    />
  );
}
