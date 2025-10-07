// frontend/src/pages/PostPage.tsx
import React, { useEffect, useMemo, useState } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { Header } from '../components/ui/Header';
import { getAuth } from '../features/auth/state';
import { getUser } from '../lib/api';
import type { UserPublic } from '../lib/types';

import {
  getPost,
  listComments,
  createComment,
  likePost,
  unlikePost,
  deletePost, // ⬅️ added
  formatUnix,
  type PostDetail,
  type CommentItem,
} from '../features/posts/api';
import { CommentCard } from '../components/comments/CommentCard';

// Chat imports
import { ChatFab } from '../components/chat/ChatFab';
import ChatPanel from '../components/chat/ChatPanel';
import { openChat, unreadCount } from '../features/chats/api';

export default function PostPage() {
  const nav = useNavigate();
  const { id } = useParams<{ id: string }>();
  const postId = Number(id);

  // header user
  const auth = getAuth();
  const [me, setMe] = useState<UserPublic | null>(null);
  const goProfile = (uid: number) =>
    nav(uid === me?.id ? '/profile' : `/users/${uid}`);

  useEffect(() => {
    (async () => {
      if (!auth?.userId) return;
      try {
        setMe(await getUser(auth.userId));
      } catch {}
    })();
  }, [auth?.userId]);

  const headerAvatar =
    me?.avatar_url ?? (me ? `/static/avatars/${me.id}.png` : undefined);

  // post + comments
  const [post, setPost] = useState<PostDetail | null>(null);
  const [comments, setComments] = useState<CommentItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [err, setErr] = useState<string | null>(null);

  // author (to know if they’re admin)
  const [author, setAuthor] = useState<UserPublic | null>(null);

  // like state for post
  const [liked, setLiked] = useState(false);
  const [score, setScore] = useState(0);
  const [busyLike, setBusyLike] = useState(false);

  // new comment composer (top-level)
  const [draft, setDraft] = useState('');

  // reply composer state
  const [replyTo, setReplyTo] = useState<CommentItem | null>(null);
  const [replyBody, setReplyBody] = useState('');

  // chat state
  const [chatOpen, setChatOpen] = useState(false);
  const [chatUnread, setChatUnread] = useState(0);
  const [initialChatId, setInitialChatId] = useState<number | null>(null);

  // delete UI state
  const [flash, setFlash] = useState<{
    kind: 'ok' | 'err';
    msg: string;
  } | null>(null);
  const [busyDelete, setBusyDelete] = useState(false);

  // load post + comments
  useEffect(() => {
    let cancelled = false;
    (async () => {
      setLoading(true);
      setErr(null);
      try {
        const p = await getPost(postId);
        const cs = await listComments(postId, 'created', 1, 200);
        if (cancelled) return;
        setPost(p);
        setComments(cs.items);
        setLiked(!!p.liked_by_me);
        setScore(p.score);
      } catch (e: any) {
        if (!cancelled) setErr(e?.message ?? 'Failed to load post');
      } finally {
        if (!cancelled) setLoading(false);
      }
    })();
    return () => {
      cancelled = true;
    };
  }, [postId]);

  // fetch author (so we can check if author is admin)
  useEffect(() => {
    if (!post?.user_id) return;
    let cancel = false;
    (async () => {
      try {
        const u = await getUser(post.user_id);
        if (!cancel) setAuthor(u);
      } catch {}
    })();
    return () => {
      cancel = true;
    };
  }, [post?.user_id]);

  // initial unread badge for the FAB
  useEffect(() => {
    unreadCount()
      .then(setChatUnread)
      .catch(() => {});
  }, []);

  // auto-hide flash
  useEffect(() => {
    if (!flash) return;
    const id = setTimeout(() => setFlash(null), 4000);
    return () => clearTimeout(id);
  }, [flash]);

  const when = useMemo(() => formatUnix(post?.created_at), [post?.created_at]);
  const authorAvatar = post
    ? post.author_avatar_url ?? `/static/avatars/${post.user_id}.png`
    : undefined;

  async function togglePostLike() {
    if (!post || busyLike) return;
    setBusyLike(true);
    try {
      if (!liked) {
        const res = await likePost(post.id);
        setLiked(true);
        setScore(res.score);
      } else {
        const res = await unlikePost(post.id);
        setLiked(false);
        setScore(res.score);
      }
    } finally {
      setBusyLike(false);
    }
  }

  async function submitComment() {
    const body = draft.trim();
    if (!body) return;
    await createComment(postId, body);
    const cs = await listComments(postId, 'created', 1, 200);
    setComments(cs.items);
    setDraft('');
  }

  async function submitReply() {
    if (!replyTo) return;
    const body = replyBody.trim();
    if (!body) return;
    await createComment(postId, body, replyTo.id);
    const cs = await listComments(postId, 'created', 1, 200);
    setComments(cs.items);
    setReplyTo(null);
    setReplyBody('');
  }

  // admin / visibility helpers
  const meIsAdmin = Boolean((me as any)?.is_admin);
  const authorIsAdmin = Boolean((author as any)?.is_admin);
  const isMine = !!me?.id && post?.user_id === me.id;

  // visibility rules:
  // - Normal user: visible only on own post
  // - Admin: visible on normal users (and own), not on other admins
  const showDeleteBtn = isMine || (meIsAdmin && !authorIsAdmin && !isMine);

  // delete click
  async function onDeleteClick() {
    if (!post) return;

    const moderating = meIsAdmin && !authorIsAdmin && !isMine;
    let reason: string | undefined;

    if (moderating) {
      reason = window
        .prompt('Enter a reason for removal (required, max 200 chars):')
        ?.trim();
      if (!reason) {
        setFlash({ kind: 'err', msg: 'Delete cancelled: reason is required.' });
        return;
      }
    }

    setBusyDelete(true);
    setFlash(null);
    try {
      await deletePost(post.id, reason);
      const fresh = await getPost(post.id); // body now becomes [Deleted By ...]
      setPost(fresh);
      setFlash({ kind: 'ok', msg: 'Post deleted successfully.' });
    } catch (e: any) {
      setFlash({
        kind: 'err',
        msg: e?.message || 'Delete failed. Please try again.',
      });
    } finally {
      setBusyDelete(false);
    }
  }

  // Chat helpers
  async function startChatWithAuthor() {
    if (!post) return;
    if (!me?.id) {
      nav('/auth');
      return;
    }
    // Don’t start a chat with yourself
    if (me.id === post.user_id) {
      setChatOpen(true);
      return;
    }
    try {
      const { chat_id } = await openChat(post.user_id);
      setInitialChatId(chat_id);
      setChatOpen(true);
    } catch (e: any) {
      alert(e?.message ?? 'Failed to start chat');
    }
  }

  if (loading) {
    return (
      <>
        <Header avatarUrl={headerAvatar} username={me?.username} />
        <main className="page">
          <div className="card">Loading…</div>
        </main>
      </>
    );
  }
  if (err || !post) {
    return (
      <>
        <Header avatarUrl={headerAvatar} username={me?.username} />
        <main className="page">
          <div className="card error">{err ?? 'Failed to load post'}</div>
        </main>
      </>
    );
  }

  return (
    <>
      <Header avatarUrl={headerAvatar} username={me?.username} />
      <main className="page" style={{ alignItems: 'start' }}>
        <div style={{ width: 'min(900px, 100%)' }}>
          {/* Post card */}
          <div
            style={{
              border: '1px solid #243156',
              borderRadius: 16,
              padding: 16,
              background: '#121831',
            }}
          >
            {/* Flash (success / error) */}
            {flash && (
              <div
                className={flash.kind === 'ok' ? 'card' : 'card error'}
                style={{
                  marginBottom: 10,
                  padding: 10,
                  borderRadius: 10,
                  border:
                    '1px solid ' +
                    (flash.kind === 'ok' ? '#2a3760' : '#5a2a2a'),
                  background: flash.kind === 'ok' ? '#0f1530' : '#2a0f10',
                  color: '#e6eaff',
                }}
              >
                {flash.msg}
              </div>
            )}

            {/* Header row (avatar, name) + right-side (time + delete) */}
            <div
              style={{
                display: 'flex',
                justifyContent: 'space-between',
                gap: 12,
                alignItems: 'center',
              }}
            >
              <div style={{ display: 'flex', alignItems: 'center', gap: 12 }}>
                <img
                  src={authorAvatar}
                  alt=""
                  onError={(e) =>
                    ((e.target as HTMLImageElement).style.display = 'none')
                  }
                  style={{
                    width: 42,
                    height: 42,
                    borderRadius: '50%',
                    objectFit: 'cover',
                    border: '1px solid #273059',
                  }}
                />
                <div style={{ display: 'grid' }}>
                  <button
                    onClick={() => goProfile(post.user_id)}
                    style={{
                      background: 'transparent',
                      border: 'none',
                      color: '#cdd7ff',
                      fontWeight: 700,
                      cursor: 'pointer',
                      padding: 0,
                      fontSize: 20,
                    }}
                    title="View profile"
                  >
                    {post.author_username}
                  </button>
                </div>
              </div>

              <div
                style={{
                  display: 'flex',
                  flexDirection: 'column',
                  alignItems: 'flex-end',
                  gap: 8,
                  minWidth: 200, // keep things from wrapping awkwardly
                  textAlign: 'right',
                }}
              >
                <div style={{ fontSize: 15, color: '#9aa3c6' }}>{when}</div>

                {showDeleteBtn && (
                  <button
                    onClick={onDeleteClick}
                    disabled={busyDelete}
                    title="Soft-delete this post"
                    style={{
                      border: '1px solid #5b2643',
                      background: busyDelete ? '#361a2a' : 'transparent',
                      color: '#ff6b81',
                      padding: '8px 12px',
                      borderRadius: 10,
                      cursor: busyDelete ? 'default' : 'pointer',
                      fontWeight: 700,
                    }}
                  >
                    {busyDelete ? 'Deleting…' : 'Delete Post'}
                  </button>
                )}
              </div>
            </div>

            <h2 style={{ margin: '12px 0 8px' }}>{post.title}</h2>
            <div style={{ whiteSpace: 'pre-wrap', lineHeight: 1.5 }}>
              {post.body}
            </div>

            {/* Chips + “Message author” button */}
            <div
              style={{
                marginTop: 12,
                display: 'flex',
                alignItems: 'center',
                gap: 18,
                color: '#97a3c7',
                flexWrap: 'wrap',
              }}
            >
              <button
                onClick={togglePostLike}
                disabled={busyLike}
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
                <span style={{ fontSize: 18 }}>❤️</span>
                <span>{score}</span>
              </button>

              <span>💬 {post.comment_count}</span>

              {/* Message author button (kept from your UI) */}
              <button
                onClick={startChatWithAuthor}
                disabled={!me?.id || me.id === post.user_id}
                title={
                  !me?.id
                    ? 'Sign in to chat'
                    : me.id === post.user_id
                    ? 'This is your post'
                    : 'Message the author'
                }
                style={{
                  marginLeft: 'auto',
                  border: '1px solid #2a3760',
                  background: 'transparent',
                  color: '#e6eaff',
                  padding: '12px 16px',
                  borderRadius: 10,
                  cursor:
                    !me?.id || me.id === post.user_id ? 'default' : 'pointer',
                  fontSize: 15,
                }}
              >
                ✉️ Message author
              </button>
            </div>
          </div>

          {/* New comment composer */}
          <div
            style={{
              marginTop: 14,
              border: '1px solid #243156',
              borderRadius: 12,
              padding: 12,
              background: '#121831',
            }}
          >
            <div className="label">Write a comment</div>
            <textarea
              className="textarea"
              rows={3}
              value={draft}
              onChange={(e) => setDraft(e.target.value)}
              placeholder="Share your thoughts…"
            />
            <button
              className="btn"
              onClick={submitComment}
              disabled={!draft.trim()}
            >
              Comment
            </button>
          </div>

          {/* Comments list */}
          <div style={{ display: 'grid', gap: 10, marginTop: 14 }}>
            {comments.length === 0 && (
              <div style={{ opacity: 0.8 }}>No comments yet.</div>
            )}
            {comments.map((c) => (
              <CommentCard
                key={c.id}
                c={c}
                onReplyClick={setReplyTo}
                viewerId={me?.id ?? null}
                viewerIsAdmin={!!me?.is_admin}
                onDeleted={(id) => {
                  // remove locally; or call listComments(...) to refresh
                  setComments((prev) => prev.filter((x) => x.id !== id));
                }}
              />
            ))}
          </div>

          {/* Reply composer */}
          {replyTo && (
            <div
              style={{
                marginTop: 14,
                border: '1px solid #2a3760',
                borderRadius: 12,
                padding: 12,
                background: '#0f1530',
              }}
            >
              <div className="label">
                Replying to <strong>@{replyTo.author_username}</strong>
              </div>
              <textarea
                className="textarea"
                rows={3}
                value={replyBody}
                onChange={(e) => setReplyBody(e.target.value)}
                placeholder="Write your reply…"
              />
              <div style={{ display: 'flex', gap: 10 }}>
                <button
                  className="btn"
                  onClick={submitReply}
                  disabled={!replyBody.trim()}
                  style={{ width: 'auto' }}
                >
                  Reply
                </button>
                <button
                  onClick={() => {
                    setReplyTo(null);
                    setReplyBody('');
                  }}
                  style={{
                    border: '1px solid #2a3760',
                    background: 'transparent',
                    color: '#e6eaff',
                    padding: '10px 14px',
                    borderRadius: 10,
                    cursor: 'pointer',
                  }}
                >
                  Cancel
                </button>
              </div>
            </div>
          )}
        </div>
      </main>

      {/* Chat FAB and panel */}
      <ChatFab count={chatUnread} onClick={() => setChatOpen(true)} />
      <ChatPanel
        open={chatOpen}
        onClose={() => setChatOpen(false)}
        initialChatId={initialChatId}
        onUnreadChange={setChatUnread}
        fallbackPeer={
          post
            ? {
                username: post.author_username,
                avatar_url:
                  post.author_avatar_url ??
                  `/static/avatars/${post.user_id}.png`,
              }
            : undefined
        }
      />
    </>
  );
}
