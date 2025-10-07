import React, { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import {
  listLikedPostsByUser,
  type PostListItem,
  formatUnix,
} from '../../features/posts/api';
import { PostMiniCard } from '../posts/PostMiniCard';

export function MyLikedPostsPanel({
  userId,
  title = 'My Liked Posts',
}: {
  userId: number;
  title?: string;
}) {
  const nav = useNavigate();
  const [items, setItems] = useState<PostListItem[]>([]);
  const [page, setPage] = useState(1);
  const [totalPages, setTotalPages] = useState(1);
  const [loading, setLoading] = useState(false);
  const [err, setErr] = useState<string | null>(null);

  useEffect(() => {
    let cancel = false;
    (async () => {
      setLoading(true);
      setErr(null);
      try {
        const res = await listLikedPostsByUser(userId, page, 5);
        if (!cancel) {
          setItems(res.items);
          setTotalPages(res.total_pages || 1);
        }
      } catch (e: any) {
        if (!cancel) setErr(e?.message ?? 'Failed to load liked posts');
      } finally {
        if (!cancel) setLoading(false);
      }
    })();
    return () => {
      cancel = true;
    };
  }, [userId, page]);

  const first = () => setPage(1);
  const prev = () => setPage((p) => Math.max(1, p - 1));
  const next = () => setPage((p) => Math.min(totalPages, p + 1));
  const last = () => setPage(totalPages);

  return (
    <div className="card" style={{ padding: 12 }}>
      {/* header with right-aligned pager (same as MyPosts) */}
      <div
        style={{
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'space-between',
          marginBottom: 8,
        }}
      >
        <h2 style={{ margin: 6 }}>{title}</h2>
        <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
          <PagerBtn onClick={first} disabled={page <= 1}>
            &laquo;
          </PagerBtn>
          <PagerBtn onClick={prev} disabled={page <= 1}>
            &lsaquo;
          </PagerBtn>
          <span style={{ opacity: 0.85, fontWeight: 700 }}>
            Page {page} / {Math.max(1, totalPages)}
          </span>
          <PagerBtn onClick={next} disabled={page >= totalPages}>
            &rsaquo;
          </PagerBtn>
          <PagerBtn onClick={last} disabled={page >= totalPages}>
            &raquo;
          </PagerBtn>
        </div>
      </div>

      {err && (
        <div className="error" style={{ marginBottom: 8 }}>
          {err}
        </div>
      )}
      {loading && <div style={{ padding: 8 }}>Loading…</div>}
      {!loading && items.length === 0 && (
        <div style={{ padding: 8, opacity: 0.8 }}>No liked posts yet.</div>
      )}

      <div style={{ display: 'grid', gap: 10 }}>
        {items.map((p) => (
          <PostMiniCard
            key={p.id}
            post={p}
            onClick={(id) => nav(`/posts/${id}`)}
          />
        ))}
      </div>
    </div>
  );
}

/*function MiniCard({
  title,
  body,
  when,
  onClick,
}: {
  title: string;
  body: string;
  when: string;
  onClick?: () => void;
}) {
  return (
    <button
      onClick={onClick}
      style={{
        textAlign: 'left',
        border: '1px solid #2a3760',
        background: '#121831',
        color: '#e6eaff',
        padding: 14,
        borderRadius: 14,
        cursor: 'pointer',
        display: 'block',
        width: '100%',
      }}
      title={title}
    >
      <div
        style={{
          display: 'flex',
          justifyContent: 'space-between',
          gap: 12,
          alignItems: 'center',
        }}
      >
        <h3 style={{ margin: '0 0 6px', fontSize: 20, fontWeight: 800 }}>
          {title}
        </h3>
        <div style={{ fontSize: 12, color: '#97a3c7', whiteSpace: 'nowrap' }}>
          {when}
        </div>
      </div>
      <div style={{ opacity: 0.9, lineHeight: 1.5 }}>{body}</div>
    </button>
  );
}*/

function PagerBtn(props: React.ButtonHTMLAttributes<HTMLButtonElement>) {
  return (
    <button
      {...props}
      style={{
        border: '1px solid #2a3760',
        background: 'transparent',
        color: '#e6eaff',
        padding: '6px 10px',
        borderRadius: 8,
        cursor: props.disabled ? 'default' : 'pointer',
        opacity: props.disabled ? 0.5 : 1,
      }}
    />
  );
}

function truncate(s: string, n = 200) {
  return s.length > n ? s.slice(0, n) + '…' : s;
}
