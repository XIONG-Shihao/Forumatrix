import React, { useEffect, useRef, useState } from 'react';
import { createPost } from '../../features/posts/api';

type Props = {
  open: boolean;
  onClose: () => void;
  onCreated?: (id: number) => void;
};

export function ComposePostModal({ open, onClose, onCreated }: Props) {
  const [title, setTitle] = useState('');
  const [body, setBody] = useState('');
  const [busy, setBusy] = useState(false);
  const [err, setErr] = useState<string | null>(null);
  const panelRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!open) return;
    const onKey = (e: KeyboardEvent) => {
      if (e.key === 'Escape' || e.key === 'Esc') onClose();
      if ((e.metaKey || e.ctrlKey) && e.key === 'Enter') submit();
    };
    document.addEventListener('keydown', onKey);
    return () => document.removeEventListener('keydown', onKey);
  }, [open]);

  async function submit() {
    if (!title.trim() || !body.trim() || busy) return;
    setBusy(true);
    setErr(null);
    try {
      const res = await createPost({ title: title.trim(), body: body.trim() });
      onCreated?.(res.id);
    } catch (e: any) {
      setErr(e?.message ?? 'Failed to post');
    } finally {
      setBusy(false);
    }
  }

  if (!open) return null;

  return (
    <div
      onMouseDown={(e) => {
        // close on outside click
        if (e.target === e.currentTarget) onClose();
      }}
      style={overlay}
    >
      <div ref={panelRef} className="card" style={panel}>
        <div
          style={{
            display: 'flex',
            alignItems: 'baseline',
            gap: 8,
            marginBottom: 12,
          }}
        >
          <h2 style={{ margin: 0 }}>New post</h2>
          <span style={{ opacity: 0.65, fontSize: 12 }}>
            (Ctrl/⌘ + Enter to post)
          </span>
        </div>

        {err && (
          <div className="error" style={{ marginBottom: 10 }}>
            {err}
          </div>
        )}

        <label style={label}>Title</label>
        <input
          className="input"
          value={title}
          onChange={(e) => setTitle(e.target.value)}
          placeholder="Title"
          autoFocus
        />

        <label style={{ ...label, marginTop: 10 }}>Body</label>
        <textarea
          className="textarea"
          rows={8}
          value={body}
          onChange={(e) => setBody(e.target.value)}
          placeholder="Write your post..."
        />

        <div
          style={{
            display: 'grid',
            gridTemplateColumns: '1fr 1fr',
            gap: 12,
            marginTop: 16,
          }}
        >
          <button
            className="btn"
            onClick={onClose}
            disabled={busy}
            style={btnSecondary}
          >
            Cancel
          </button>
          <button
            className="btn"
            onClick={submit}
            disabled={busy || !title.trim() || !body.trim()}
            style={btnPrimary}
          >
            {busy ? 'Posting…' : 'Post'}
          </button>
        </div>
      </div>
    </div>
  );
}

const overlay: React.CSSProperties = {
  position: 'fixed',
  inset: 0,
  background: 'rgba(0,0,0,.45)',
  display: 'grid',
  placeItems: 'center',
  zIndex: 50,
};

const panel: React.CSSProperties = {
  width: 'min(720px, 92vw)',
  padding: 18,
  borderRadius: 16,
  background: '#121831',
  border: '1px solid #1f2a4d',
  boxShadow: '0 20px 60px rgba(0,0,0,.45)',
};

const label: React.CSSProperties = {
  fontSize: 12,
  opacity: 0.75,
  marginBottom: 6,
};

const btnPrimary: React.CSSProperties = {
  background: '#5163ff',
  border: '1px solid #5163ff',
  color: 'white',
  fontWeight: 600,
  borderRadius: 12,
  padding: '10px 12px',
};

const btnSecondary: React.CSSProperties = {
  background: 'transparent',
  border: '1px solid #2a3760',
  color: '#e6eaff',
  fontWeight: 600,
  borderRadius: 12,
  padding: '10px 12px',
};
