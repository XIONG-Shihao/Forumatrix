import React from 'react';
import {
  type CommentItem,
  likeComment,
  unlikeComment,
  formatUnix,
  deleteComment, // <-- add this API
} from '../../features/posts/api';
import { useNavigate } from 'react-router-dom';
import { getAuth } from '../../features/auth/state';

export function CommentCard({
  c,
  onReplyClick,
  // Optional overrides if parent wants to be explicit:
  viewerId,
  viewerIsAdmin,
  onDeleted,
}: {
  c: CommentItem;
  onReplyClick?: (c: CommentItem) => void;
  viewerId?: number | null;
  viewerIsAdmin?: boolean;
  onDeleted?: (id: number) => void; // parent can prune the list
}) {
  const when = formatUnix(c.created_at);
  const avatar = c.author_avatar_url ?? `/static/avatars/${c.user_id}.png`;

  const [liked, setLiked] = React.useState<boolean>(!!c.liked_by_me);
  const [score, setScore] = React.useState<number>(c.score);
  const [busy, setBusy] = React.useState(false);

  const [busyDelete, setBusyDelete] = React.useState(false);

  const nav = useNavigate();
  const me = getAuth();
  const _viewerId = viewerId ?? me?.userId ?? null;
  // Some auth states include is_admin; if not present, treat as false.
  const _viewerIsAdmin =
    viewerIsAdmin ??
    (typeof (me as any)?.is_admin === 'boolean' ? (me as any).is_admin : false);

  const goProfile = (uid: number) =>
    nav(uid === me?.userId ? '/profile' : `/users/${uid}`);

  // ---- ONE PLACE TO TUNE SIZES / SPACING ----
  const UI = {
    nameFont: 18, // author name
    metaFont: 13, // "replying to" + small meta
    bodyFont: 18, // comment text
    timeFont: 14, // time (top-right)
    nameGap: 10, // distance between name and "replying to ..."
    replyBtn: {
      font: 14, // "Reply" text size
      padV: 6,
      padH: 10, // "Reply" button padding
      radius: 8, // "Reply" button border radius
    },
    delBtn: {
      font: 12,
      padV: 6,
      padH: 10,
      radius: 8,
    },
  };

  // ----- Delete permissions aligned with backend -----
  const isOwner = _viewerId != null && _viewerId === c.user_id;
  const authorIsAdmin = !!c.author_is_admin;

  /**
   * Rules:
   * - Normal user can delete OWN comment
   * - Admin can delete normal users’ comments (not admin-authored)
   * - Admin cannot delete admin comments (including their own)
   */
  const canDelete = isOwner || (_viewerIsAdmin && !authorIsAdmin);
  const adminAction = _viewerIsAdmin && !authorIsAdmin && !isOwner;

  async function toggleLike(e: React.MouseEvent) {
    e.stopPropagation();
    if (busy) return;
    setBusy(true);
    try {
      if (!liked) {
        const res = await likeComment(c.id);
        setLiked(true);
        setScore(res.score);
      } else {
        const res = await unlikeComment(c.id);
        setLiked(false);
        setScore(res.score);
      }
    } finally {
      setBusy(false);
    }
  }

  async function onDeleteClick(e: React.MouseEvent) {
    e.stopPropagation();
    if (!canDelete || busyDelete) return;

    const adminAction = _viewerIsAdmin && !authorIsAdmin && !isOwner;
    let reason: string | undefined;

    if (adminAction) {
      const input =
        window.prompt('Reason for deletion (required, ≤200 chars):') ?? '';
      const trimmed = input.trim();
      if (!trimmed) return; // cancel or empty -> no action
      reason = trimmed;
    } else {
      const ok = window.confirm('Delete this comment?');
      if (!ok) return;
    }

    setBusyDelete(true);
    try {
      await deleteComment(c.id, reason);
      onDeleted?.(c.id); // let parent remove it or refetch
    } catch (err: any) {
      alert(err?.message ?? 'Delete failed');
    } finally {
      setBusyDelete(false);
    }
  }

  return (
    <div
      style={{
        border: '1px solid #243156',
        borderRadius: 12,
        padding: 12,
        background: '#0f1530',
      }}
    >
      {/* Header row: avatar + (name + replying-to) ... time + Delete */}
      <div style={{ display: 'flex', justifyContent: 'space-between' }}>
        <div style={{ display: 'flex', gap: 10, alignItems: 'center' }}>
          <img
            src={avatar}
            alt=""
            onClick={() => goProfile(c.user_id)}
            onError={(e) =>
              ((e.target as HTMLImageElement).style.display = 'none')
            }
            style={{
              width: 28,
              height: 28,
              borderRadius: '50%',
              objectFit: 'cover',
              border: '1px solid #273059',
              cursor: 'pointer',
            }}
            title="View profile"
          />
          {/* Name + 'replying to …' on ONE line */}
          <div
            style={{
              display: 'flex',
              alignItems: 'baseline',
              gap: UI.nameGap, // <— Adjust distance here
              flexWrap: 'wrap',
            }}
          >
            <button
              onClick={() => goProfile(c.user_id)}
              title="View profile"
              style={{
                background: 'transparent',
                border: 'none',
                color: '#cdd7ff',
                fontWeight: 700,
                fontSize: UI.nameFont,
                cursor: 'pointer',
                textAlign: 'left',
                padding: 0,
              }}
            >
              {c.author_username}
            </button>

            {c.parent_author_username && (
              <span style={{ fontSize: UI.metaFont, color: '#9aa3c6' }}>
                replying to @{c.parent_author_username}
              </span>
            )}
          </div>
        </div>

        {/* Right: time + Delete under it */}
        <div
          style={{
            display: 'flex',
            flexDirection: 'column',
            alignItems: 'flex-end',
            gap: 6,
          }}
        >
          <div style={{ fontSize: UI.timeFont, color: '#9aa3c6' }}>{when}</div>

          {canDelete && (
            <button
              onClick={onDeleteClick}
              disabled={busyDelete}
              title="Soft-delete this comment"
              style={{
                border: '1px solid #5b2643',
                background: busyDelete ? '#361a2a' : 'transparent',
                color: '#ff6b81',
                padding: `${UI.delBtn.padV}px ${UI.delBtn.padH}px`,
                borderRadius: UI.delBtn.radius,
                cursor: busyDelete ? 'default' : 'pointer',
                fontWeight: 700,
                fontSize: UI.delBtn.font,
              }}
            >
              {busyDelete ? 'Deleting…' : 'Delete'}
            </button>
          )}
        </div>
      </div>

      {/* Comment body */}
      <div style={{ marginTop: 8, lineHeight: 1.45, fontSize: UI.bodyFont }}>
        {c.body}
      </div>

      {/* Actions */}
      <div
        style={{
          marginTop: 10,
          display: 'flex',
          alignItems: 'center',
          gap: 16,
          color: '#97a3c7',
          fontSize: 13,
        }}
      >
        <button
          onClick={toggleLike}
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
          }}
        >
          <span style={{ fontSize: 16 }}>❤️</span>
          <span>{score}</span>
        </button>

        <span>💬 {c.reply_count}</span>

        {onReplyClick && (
          <button
            onClick={() => onReplyClick(c)}
            style={{
              border: '1px solid #2a3760',
              background: 'transparent',
              color: '#e6eaff',
              padding: `${UI.replyBtn.padV}px ${UI.replyBtn.padH}px`,
              borderRadius: UI.replyBtn.radius,
              cursor: 'pointer',
              fontSize: UI.replyBtn.font,
            }}
          >
            Reply
          </button>
        )}
      </div>
    </div>
  );
}
