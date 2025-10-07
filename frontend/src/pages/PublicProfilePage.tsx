// frontend/src/pages/PublicProfilePage.tsx
import React, { useEffect, useMemo, useState } from 'react';
import { useParams, useNavigate } from 'react-router-dom';

import { Header } from '../components/ui/Header';
import { ChatFab } from '../components/chat/ChatFab';

import { getAuth } from '../features/auth/state';
import { getUser } from '../features/users/api';
import type { UserPublic } from '../features/users/types';

import { MyPostsPanel } from '../components/profile/MyPostsPanel';
import { MyCommentsPanel } from '../components/profile/MyCommentsPanel';
import { MyLikedPostsPanel } from '../components/profile/MyLikedPostsPanel';

import { Avatar } from '../components/avatar/Avatar';
import { AvatarLightbox } from '../components/avatar/AvatarLightbox';
import { getAvatarVer } from '../lib/avatarCache';

import ChatPanel from '../components/chat/ChatPanel';
import { openChat } from '../features/chats/api';
import { unreadCount } from '../features/chats/api';

import { suspendUser } from '../lib/api';

import '../styles/profile.css';

type Section = 'profile' | 'posts' | 'comments' | 'liked';

export default function PublicProfilePage() {
  const { id } = useParams<{ id: string }>();
  const userId = Number(id);

  const nav = useNavigate();
  const me = getAuth(); // nullable
  const myAvatarUrl = useMemo(() => {
    if (!me?.userId) return '';
    const v = getAvatarVer(me.userId);
    return `/static/avatars/${me.userId}.png?v=${v}`;
  }, [me?.userId]);

  const [user, setUser] = useState<UserPublic | null>(null);
  const [loading, setLoading] = useState(true);
  const [err, setErr] = useState<string | null>(null);

  // read-only profile UI helpers
  const [openLarge, setOpenLarge] = useState(false);
  const [section, setSection] = useState<Section>('profile');

  // chat
  const [chatOpen, setChatOpen] = useState(false);
  const [chatUnread, setChatUnread] = useState(0);

  const [initialChatId, setInitialChatId] = useState<number | null>(null);
  // flash message (success/error)
  const [flash, setFlash] = useState<{
    kind: 'ok' | 'err';
    msg: string;
  } | null>(null);

  // keep a “meFull” so we know if the viewer is admin
  const [meFull, setMeFull] = useState<UserPublic | null>(null);
  useEffect(() => {
    (async () => {
      if (!me?.userId) return;
      try {
        const u = await getUser(me.userId);
        setMeFull(u);
      } catch {}
    })();
  }, [me?.userId]);

  const viewerIsAdmin = (meFull?.is_admin ?? 0) === 1;
  const targetIsAdmin = (user?.is_admin ?? 0) === 1;
  const isSelf = !!me?.userId && user?.id === me.userId;

  const showSuspendBtn = !!user && viewerIsAdmin && !targetIsAdmin && !isSelf;
  // Suspend user button handler
  async function onSuspend() {
    if (!user) return;
    if (!confirm(`Suspend @${user.username}? This will disable login.`)) return;
    try {
      await suspendUser(user.id);
      setFlash({ kind: 'ok', msg: `@${user.username} has been suspended.` });
      // reflect state in UI (optional)
      setUser((u) => (u ? { ...u, is_active: 0 } : u));
    } catch (e: any) {
      setFlash({ kind: 'err', msg: e?.message ?? 'Failed to suspend user' });
    } finally {
      // auto-hide after a few seconds
      setTimeout(() => setFlash(null), 3000);
    }
  }
  // Start chat handler
  async function onStartChat() {
    if (!me?.userId) {
      nav('/auth');
      return;
    }
    if (!user) return;

    try {
      const { chat_id } = await openChat(user.id);
      setInitialChatId(chat_id); // ensure this chat is selected when opening
      setChatOpen(true);
    } catch (e: any) {
      alert(e?.message ?? 'Failed to start chat');
    }
  }

  useEffect(() => {
    // fetch an initial unread total so the FAB has a badge even before opening
    unreadCount()
      .then(setChatUnread)
      .catch(() => {});
  }, []);

  // load the target user
  useEffect(() => {
    let cancel = false;
    (async () => {
      try {
        setErr(null);
        setLoading(true);
        const u = await getUser(userId);
        if (!cancel) setUser(u);
      } catch (e: any) {
        if (!cancel) setErr(e?.message ?? 'Failed to load user');
      } finally {
        if (!cancel) setLoading(false);
      }
    })();
    return () => {
      cancel = true;
    };
  }, [userId]);

  const avatarSrc = useMemo(() => {
    if (!user) return '';
    const base = user.avatar_url?.trim()
      ? user.avatar_url!
      : `/static/avatars/${user.id}.png`;
    const v = getAvatarVer(user.id);
    return `${base}?v=${v}`;
  }, [user]);

  // Safe fallbackPeer (only when user is loaded)
  const fallbackPeer = user
    ? {
        username: user.username ?? 'User',
        avatar_url: user.avatar_url ?? undefined,
      }
    : undefined;

  return (
    <>
      {/* Global header — compose button hidden on this page */}
      <Header
        showCompose={false}
        avatarUrl={myAvatarUrl}
        username={me?.username}
      />

      <main className="page profile-page">
        {loading ? (
          <div className="card">Loading…</div>
        ) : err || !user ? (
          <div className="card error">{err ?? 'User not found'}</div>
        ) : (
          <div className="profile-layout">
            {/* ---- Left sidebar ---- */}
            <nav className="profile-sidebar">
              <button
                className="side-btn"
                onClick={() => setSection('profile')}
                aria-pressed={section === 'profile'}
              >
                Profile
              </button>
              <button
                className="side-btn"
                onClick={() => setSection('posts')}
                aria-pressed={section === 'posts'}
              >
                Posts
              </button>
              <button
                className="side-btn"
                onClick={() => setSection('comments')}
                aria-pressed={section === 'comments'}
              >
                Comments
              </button>
              <button
                className="side-btn"
                onClick={() => setSection('liked')}
                aria-pressed={section === 'liked'}
              >
                Liked Posts
              </button>

              <button
                className="side-btn danger"
                onClick={onStartChat}
                disabled={!user || !me?.userId}
                title={
                  !me?.userId
                    ? 'Sign in to start a chat'
                    : !user
                    ? 'Loading…'
                    : 'Start chat'
                }
              >
                Start chat
              </button>
            </nav>

            {/* ---- Right panel (read-only) ---- */}
            {section === 'profile' ? (
              <div className="card profile-card">
                {/* Flash message (success / error) */}
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

                <div
                  className="header-row"
                  style={{ justifyContent: 'space-between' }}
                >
                  <div
                    style={{ display: 'flex', gap: 16, alignItems: 'center' }}
                  >
                    <Avatar
                      src={avatarSrc}
                      fallbackInitial={user.username?.[0]?.toUpperCase() ?? '?'}
                      size={56}
                      onClick={() => setOpenLarge(true)}
                      title="Click to enlarge"
                    />
                    <div>
                      <div className="title">{user.username}</div>
                      <div className="subtle">{user.email}</div>
                    </div>
                  </div>
                  {/* right: Suspend (admins on non-admin targets only) */}
                  {showSuspendBtn && (
                    <button
                      onClick={onSuspend}
                      title="Suspend this account"
                      style={{
                        border: '1px solid #5b2643',
                        background: 'transparent',
                        color: '#ff6b81',
                        padding: '10px 14px',
                        borderRadius: 10,
                        cursor: 'pointer',
                        fontWeight: 800,
                      }}
                    >
                      Suspend
                    </button>
                  )}
                </div>

                <div className="fields" style={{ marginTop: 8 }}>
                  <div className="field">
                    <label className="label">Username</label>
                    <input
                      className="input"
                      value={user.username ?? ''}
                      disabled
                    />
                  </div>

                  <div className="field">
                    <label className="label">Date of Birth</label>
                    <input className="input" value={user.dob ?? ''} disabled />
                  </div>

                  <div className="field">
                    <label className="label">Bio</label>
                    <input className="input" value={user.bio ?? ''} disabled />
                  </div>

                  <div className="field">
                    <label className="label">Account Status</label>
                    <input
                      className="input"
                      value={user.is_active === 1 ? 'Active' : 'Suspended'}
                      disabled
                    />
                  </div>
                </div>
              </div>
            ) : section === 'posts' ? (
              <MyPostsPanel userId={user.id} />
            ) : section === 'comments' ? (
              <MyCommentsPanel userId={user.id} />
            ) : (
              <MyLikedPostsPanel userId={user.id} />
            )}
          </div>
        )}
      </main>

      {openLarge && (
        <AvatarLightbox src={avatarSrc} onClose={() => setOpenLarge(false)} />
      )}

      {/* Single ChatPanel instance with safe fallbackPeer */}
      <ChatPanel
        open={chatOpen}
        onClose={() => setChatOpen(false)}
        initialChatId={initialChatId}
        // fallbackPeer={fallbackPeer}
        onUnreadChange={setChatUnread}
      />

      {/* Global chat FAB in the bottom-right */}
      <ChatFab
        onClick={() => setChatOpen(true)}
        count={chatUnread}
        size={72}
        fontSize={36}
      />
    </>
  );
}
