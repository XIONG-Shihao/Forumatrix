// frontend/src/pages/docs/CreateDocModal.tsx
import React from 'react';
import { createDoc } from '../../features/docs/api';

type Props = {
  open: boolean;
  onClose: () => void;
  onCreated: (newId: number) => void;
  onError?: (msg: string) => void;
};

export default function CreateDocModal({
  open,
  onClose,
  onCreated,
  onError,
}: Props) {
  const [title, setTitle] = React.useState('');
  const [pages, setPages] = React.useState<string>('1'); // keep as string for the field
  const [busy, setBusy] = React.useState(false);
  const [err, setErr] = React.useState<string | null>(null);

  React.useEffect(() => {
    if (!open) {
      // reset when closing
      setTitle('');
      setPages('1');
      setBusy(false);
      setErr(null);
    }
  }, [open]);

  if (!open) return null;

  function clampPages(n: number): number {
    if (!Number.isFinite(n)) return 1;
    return Math.min(10, Math.max(1, Math.trunc(n)));
  }

  async function handleCreate(e: React.FormEvent) {
    e.preventDefault();
    if (busy) return;

    const trimmed = title.trim();
    const pageCount = clampPages(Number(pages));

    if (!trimmed) {
      setErr('Title cannot be empty.');
      onError?.('Title cannot be empty.');
      return;
    }

    setBusy(true);
    setErr(null);
    try {
      const res = await createDoc({ title: trimmed, page_count: pageCount });
      onCreated(res.id);
      onClose();
    } catch (e: any) {
      const msg =
        (e && typeof e.message === 'string' && e.message) ||
        'Failed to create document.';
      setErr(msg);
      onError?.(msg);
      // Also log for quick inspection
      // eslint-disable-next-line no-console
      console.error('CreateDoc failed:', e);
    } finally {
      setBusy(false);
    }
  }

  return (
    <div
      role="dialog"
      aria-modal="true"
      style={{
        position: 'fixed',
        inset: 0,
        background: 'rgba(0,0,0,.5)',
        display: 'grid',
        placeItems: 'center',
        zIndex: 100,
      }}
      onClick={onClose}
    >
      <form
        onClick={(e) => e.stopPropagation()}
        onSubmit={handleCreate}
        className="card"
        style={{
          width: 420,
          padding: 16,
          borderRadius: 12,
          background: '#0f1530',
          border: '1px solid #2a3760',
        }}
      >
        <h3 style={{ marginTop: 0, marginBottom: 12 }}>New Document</h3>

        {err && (
          <div
            className="error"
            style={{
              marginBottom: 10,
              padding: 10,
              borderRadius: 8,
              border: '1px solid #5a2a2a',
              background: '#2a0f10',
              color: '#e6eaff',
              minHeight: 20,
            }}
          >
            {err}
          </div>
        )}

        <label style={{ display: 'grid', gap: 6, marginBottom: 12 }}>
          <span style={{ color: '#cdd7ff' }}>Title</span>
          <input
            autoFocus
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            placeholder="Untitled"
            style={{
              background: '#101735',
              border: '1px solid #2a3760',
              color: '#e6eaff',
              padding: '10px 12px',
              borderRadius: 8,
              outline: 'none',
            }}
          />
        </label>

        <label style={{ display: 'grid', gap: 6, marginBottom: 16 }}>
          <span style={{ color: '#cdd7ff' }}>Pages (1–10)</span>
          <input
            type="number"
            min={1}
            max={10}
            value={pages}
            onChange={(e) => setPages(e.target.value)}
            style={{
              background: '#101735',
              border: '1px solid #2a3760',
              color: '#e6eaff',
              padding: '10px 12px',
              borderRadius: 8,
              outline: 'none',
              width: 120,
            }}
          />
          <small style={{ color: '#9aa3c6' }}>
            A4 pages. We cap collaborators at 10; page count is also limited to 10.
          </small>
        </label>

        <div style={{ display: 'flex', gap: 10, justifyContent: 'flex-end' }}>
          <button
            type="button"
            onClick={onClose}
            disabled={busy}
            style={{
              border: '1px solid #2a3760',
              background: 'transparent',
              color: '#e6eaff',
              padding: '10px 12px',
              borderRadius: 8,
              cursor: 'pointer',
            }}
          >
            Cancel
          </button>
          <button
            type="submit"
            disabled={busy}
            style={{
              border: '1px solid #2a3760',
              background: busy ? '#25305c' : '#3b82f6',
              color: 'white',
              padding: '10px 14px',
              borderRadius: 8,
              cursor: busy ? 'default' : 'pointer',
              fontWeight: 800,
            }}
          >
            {busy ? 'Creating…' : 'Create'}
          </button>
        </div>
      </form>
    </div>
  );
}