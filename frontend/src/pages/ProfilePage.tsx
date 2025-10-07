// frontend/src/pages/ProfilePage.tsx
import React, { useEffect, useMemo, useState } from 'react';
import { useNavigate } from 'react-router-dom';

import { Header } from '../components/ui/Header';
import { getAuth, clearAuth } from '../features/auth/state';
import { logout as apiLogout } from '../lib/api';

import { getUser, uploadAvatar, updateUser } from '../features/users/api';
import type { UserPublic } from '../features/users/types';

import { getAvatarVer, bumpAvatarVer } from '../lib/avatarCache';
import { Avatar } from '../components/avatar/Avatar';
import { AvatarLightbox } from '../components/avatar/AvatarLightbox';
import { AvatarUploader } from '../components/avatar/AvatarUploader';

import { MyPostsPanel } from '../components/profile/MyPostsPanel';
import { MyCommentsPanel } from '../components/profile/MyCommentsPanel';
import { MyLikedPostsPanel } from '../components/profile/MyLikedPostsPanel';
import { ChatFab } from '../components/chat/ChatFab';
import ChatPanel from '../components/chat/ChatPanel';
import { unreadCount } from '../features/chats/api';

import { NotificationsPanel } from '../components/notifications/NotificationsPanel';

import '../styles/profile.css';

type Section = 'profile' | 'posts' | 'comments' | 'liked';

export default function ProfilePage() {
  const nav = useNavigate();
  const auth = getAuth();
  const userId = auth?.userId ?? null;

  const [user, setUser] = useState<UserPublic | null>(null);
  const [loading, setLoading] = useState(true);
  const [err, setErr] = useState<string | null>(null);

  // edit mode
  const [editing, setEditing] = useState(false);
  const [saving, setSaving] = useState(false);
  const [editErr, setEditErr] = useState<string | null>(null);
  const [draftUsername, setDraftUsername] = useState('');
  const [draftBio, setDraftBio] = useState<string>('');

  // avatar
  const [openLarge, setOpenLarge] = useState(false);
  const [ver, setVer] = useState<string>(() => getAvatarVer(userId));

  // left-nav section
  const [section, setSection] = useState<Section>('profile');
  const [bellOpen, setBellOpen] = useState(false);

  // chat
  const [chatOpen, setChatOpen] = useState(false);
  const [chatUnread, setChatUnread] = useState(0);
  // load user
  useEffect(() => {
    if (!userId) {
      setErr('Not signed in');
      setLoading(false);
      return;
    }
    let cancelled = false;
    (async () => {
      try {
        setErr(null);
        setLoading(true);
        const u = await getUser(userId);
        if (!cancelled) {
          setUser(u);
          setDraftUsername(u.username ?? '');
          setDraftBio(u.bio ?? '');
        }
      } catch (e: any) {
        if (!cancelled) setErr(e?.message ?? 'Failed to load profile');
      } finally {
        if (!cancelled) setLoading(false);
      }
    })();
    return () => {
      cancelled = true;
    };
  }, [userId]);

  useEffect(() => {
    // fetch an initial unread total so the FAB has a badge even before opening
    unreadCount()
      .then(setChatUnread)
      .catch(() => {});
  }, []);

  // keep cache-buster in sync if id changes
  useEffect(() => {
    setVer(getAvatarVer(userId));
  }, [userId]);

  const avatarSrc = useMemo(() => {
    if (!user) return '';
    const base = user.avatar_url?.trim()
      ? user.avatar_url!
      : `/static/avatars/${user.id}.png`;
    return `${base}?v=${ver}`;
  }, [user, ver]);

  async function handleUpload(file: File) {
    if (!userId) return;
    const { url } = await uploadAvatar(userId, file);
    setUser((prev) => (prev ? { ...prev, avatar_url: url } : prev));
    setVer(bumpAvatarVer(userId));
  }

  function onEditToggle() {
    if (!user) return;
    setEditErr(null);
    if (!editing) {
      setDraftUsername(user.username ?? '');
      setDraftBio(user.bio ?? '');
      setEditing(true);
    } else {
      setEditing(false);
    }
  }

  async function onLogout() {
    const sure = window.confirm('Log out of Arena-AI?');
    if (!sure) return;
    try {
      await apiLogout();
    } catch {
      // ignore
    } finally {
      clearAuth();
      nav('/auth', { replace: true });
    }
  }

  async function onSave() {
    if (!user || !userId) return;
    setEditErr(null);
    setSaving(true);
    try {
      await updateUser(userId, {
        username: draftUsername.trim(),
        dob: user.dob ?? null, // DOB is not editable from UI
        bio: draftBio.trim() === '' ? null : draftBio.trim(),
      });
      setUser((prev) =>
        prev
          ? {
              ...prev,
              username: draftUsername.trim(),
              bio: draftBio.trim() || null,
            }
          : prev
      );
      setEditing(false);
    } catch (e: any) {
      setEditErr(e?.message ?? 'Failed to save profile');
    } finally {
      setSaving(false);
    }
  }

  if (loading) {
    return (
      <>
        <Header avatarClickable={false} />
        <main className="page" style={{ alignItems: 'start' }}>
          <div className="card">Loading…</div>
        </main>
      </>
    );
  }
  if (err || !user) {
    return (
      <>
        <Header avatarClickable={false} />
        <main className="page" style={{ alignItems: 'start' }}>
          <div className="card error">{err ?? 'Failed to load profile'}</div>
        </main>
      </>
    );
  }

  // header avatar (non-clickable here)
  const headerAvatar = user?.avatar_url?.trim()
    ? `${user.avatar_url}?v=${ver}`
    : `/static/avatars/${user.id}.png?v=${ver}`;

  return (
    <>
      <Header
        avatarUrl={avatarSrc}
        username={user.username}
        avatarClickable={false}
        onBellClick={() => setBellOpen((v) => !v)}
      />

      <NotificationsPanel
        open={bellOpen}
        onClose={() => setBellOpen(false)}
        onNavigatePost={(pid) => {
          setBellOpen(false);
          nav(`/posts/${pid}`);
        }}
      />

      <main className="page">
        <div className="profile-layout">
          {/* ---- Left sidebar ---- */}
          <nav className="profile-sidebar">
            <button
              className="side-btn"
              onClick={() => setSection('profile')}
              aria-pressed={section === 'profile'}
            >
              My Profile
            </button>
            <button
              className="side-btn"
              onClick={() => setSection('posts')}
              aria-pressed={section === 'posts'}
            >
              My Posts
            </button>
            <button
              className="side-btn"
              onClick={() => setSection('comments')}
              aria-pressed={section === 'comments'}
            >
              My Comments
            </button>
            <button
              className="side-btn"
              onClick={() => setSection('liked')}
              aria-pressed={section === 'liked'}
            >
              My Liked Posts
            </button>

            <button className="side-btn danger" onClick={onLogout}>
              Log out
            </button>
          </nav>

          {/* ---- Right panel (switches by section) ---- */}
          {section === 'profile' ? (
            <div className="card profile-card">
              <div
                className="header-row"
                style={{ justifyContent: 'space-between' }}
              >
                <div style={{ display: 'flex', gap: 16, alignItems: 'center' }}>
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

                <button
                  className="btn"
                  style={{
                    width: 'auto',
                    background: editing ? '#16a34a' : '#374151',
                    borderColor: 'transparent',
                  }}
                  onClick={editing ? onSave : onEditToggle}
                  disabled={saving}
                  title={editing ? 'Save changes' : 'Edit profile'}
                >
                  {editing ? (saving ? 'Saving…' : 'Save') : 'Edit'}
                </button>
              </div>

              {editErr && (
                <div className="error" style={{ marginBottom: 12 }}>
                  {editErr}
                </div>
              )}

              {/* fields */}
              <div className="fields">
                <div className="field">
                  <label className="label">Username</label>
                  <input
                    className="input"
                    value={editing ? draftUsername : user.username}
                    onChange={(e) => setDraftUsername(e.target.value)}
                    disabled={!editing}
                  />
                </div>

                {/* Email field intentionally removed (email already shown under avatar) */}

                <div className="field">
                  <label className="label">Date of Birth</label>
                  <input
                    className="input"
                    type="date"
                    value={user.dob ?? ''}
                    onChange={() => {}}
                    disabled
                  />
                </div>

                <div className="field">
                  <label className="label">Bio</label>
                  <input
                    className="input"
                    value={editing ? draftBio : user.bio ?? ''}
                    onChange={(e) => setDraftBio(e.target.value)}
                    disabled={!editing}
                  />
                </div>
              </div>

              {/* Uploader only visible while editing */}
              {editing && (
                <div className="uploader-section">
                  <div className="label">Upload new avatar</div>
                  <AvatarUploader onUpload={handleUpload} />
                </div>
              )}
            </div>
          ) : section === 'posts' ? (
            <MyPostsPanel userId={user.id} />
          ) : section === 'comments' ? (
            <MyCommentsPanel userId={user.id} />
          ) : (
            <MyLikedPostsPanel userId={user.id} />
          )}
        </div>
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
          // (optional) initialChatId / fallbackPeer as needed on PublicProfile
          onUnreadChange={setChatUnread}
        />
        {openLarge && (
          <AvatarLightbox src={avatarSrc} onClose={() => setOpenLarge(false)} />
        )}
      </main>
    </>
  );
}
