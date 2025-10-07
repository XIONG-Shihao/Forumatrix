// frontend/src/components/posts/PostCard.tsx
import React from 'react';
import type { PostListItem } from '../../features/posts/api';
import { likePost, unlikePost, formatUnix } from '../../features/posts/api';

export function PostCard({
  post,
  onClick,
}: {
  post: PostListItem;
  onClick?: (id: number) => void;
}) {
  const when = formatUnix(post.created_at);
  const avatar =
    post.author_avatar_url ?? `/static/avatars/${post.user_id}.png`;

  return (
    <article
      className="card"
      style={{
        width: 'var(--feed-card-width, 100%)', // ⬅ fill the parent by default
        maxWidth: 'var(--feed-card-maxw, none)', // ⬅ cancel any global max-width
        padding: 'var(--feed-card-pad, 14px)', // ← card padding
        cursor: onClick ? 'pointer' : 'default',
        minHeight: 'var(--feed-card-minh, auto)', // ← optional card min-height
      }}
      onClick={() => onClick?.(post.id)}
    >
      {/* Header row */}
      <div
        style={{
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'center',
          gap: 12,
        }}
      >
        <div style={{ display: 'flex', gap: 12, alignItems: 'center' }}>
          <img
            src={avatar}
            onError={(e) =>
              ((e.target as HTMLImageElement).style.display = 'none')
            }
            alt=""
            style={{
              width: 'var(--feed-avatar-size, 42px)', // ← avatar size
              height: 'var(--feed-avatar-size, 42px)',
              borderRadius: '50%',
              objectFit: 'cover',
              border: '1px solid #273059',
            }}
          />
          <div style={{ display: 'grid' }}>
            <div
              style={{
                fontWeight: 700,
                fontSize: 'var(--feed-username-size, 15px)', // ← username font
              }}
            >
              {post.author_username}
            </div>
          </div>
        </div>

        <div
          style={{
            fontSize: 'var(--feed-time-size, 13px)', // ← time font
            color: '#97a3c7',
            whiteSpace: 'nowrap',
          }}
        >
          {when}
        </div>
      </div>

      {/* Title + body */}
      <h3
        style={{
          margin: '10px 0 6px',
          fontSize: 'var(--feed-title-size, 20px)', // ← title font
        }}
      >
        {post.title}
      </h3>

      <p
        style={{
          margin: 0,
          opacity: 0.9,
          lineHeight: 1.45,
          fontSize: 'var(--feed-body-size, 14px)', // ← body font
        }}
      >
        {post.body.length > 240 ? post.body.slice(0, 240) + '…' : post.body}
      </p>

      {/* Footer: like + comments */}
      <div
        style={{
          marginTop: 10,
          color: '#97a3c7',
          display: 'flex',
          gap: 'var(--feed-gap, 14px)',
          alignItems: 'center',
          fontSize: 'var(--feed-chip-font, 14px)', // ← chip text size
        }}
      >
        <LikeButton id={post.id} initialScore={post.score} />

        <span style={{ display: 'inline-flex', alignItems: 'center', gap: 6 }}>
          <span style={{ fontSize: 'var(--feed-icon-size, 18px)' }}>💬</span>
          {post.comment_count}
        </span>
      </div>
    </article>
  );
}

/* Like button that doesn't propagate clicks (so the card doesn't navigate) */
function LikeButton({
  id,
  initialScore,
}: {
  id: number;
  initialScore: number;
}) {
  const [liked, setLiked] = React.useState(false);
  const [score, setScore] = React.useState(initialScore);
  const [busy, setBusy] = React.useState(false);

  const toggle = async (e: React.MouseEvent) => {
    e.stopPropagation();
    if (busy) return;
    setBusy(true);
    try {
      if (!liked) {
        const res = await likePost(id);
        setLiked(true);
        setScore(res.score);
      } else {
        const res = await unlikePost(id);
        setLiked(false);
        setScore(res.score);
      }
    } finally {
      setBusy(false);
    }
  };

  return (
    <button
      onClick={toggle}
      disabled={busy}
      title={liked ? 'Unlike' : 'Like'}
      style={{
        border: 'none',
        background: 'transparent',
        color: liked ? '#ff6b81' : '#97a3c7',
        cursor: 'pointer',
        display: 'inline-flex',
        alignItems: 'center',
        gap: 6,
        fontSize: 'var(--feed-chip-font, 14px)', // ← like text size
        padding: 'var(--feed-chip-pad, 0)', // ← optional area size
      }}
    >
      <span style={{ fontSize: 'var(--feed-icon-size, 18px)' }}>❤️</span>
      <span>{score}</span>
    </button>
  );
}
