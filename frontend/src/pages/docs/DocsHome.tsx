// frontend/src/pages/docs/DocsHome.tsx
import React from 'react';
import { useNavigate } from 'react-router-dom';
import { Header } from '../../components/ui/Header';
import { getAuth } from '../../features/auth/state';
import { getUser } from '../../lib/api';
import type { UserPublic } from '../../lib/types';

import {
  listMyDocs,
  type DocumentRow,
  type DocumentListResponse,
} from '../../features/docs/api';

import CreateDocModal from './CreateDocModal';

export default function DocsHome() {
  const nav = useNavigate();

  // ---- header avatar ----
  const auth = getAuth();
  const [me, setMe] = React.useState<UserPublic | null>(null);
  React.useEffect(() => {
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

  // ---- list state ----
  const [items, setItems] = React.useState<DocumentRow[]>([]);
  const [page, setPage] = React.useState(1);
  const [totalPages, setTotalPages] = React.useState(1);
  const [loading, setLoading] = React.useState(false);
  const [err, setErr] = React.useState<string | null>(null);

  // ---- modal / flash ----
  const [createOpen, setCreateOpen] = React.useState(false);
  const [flash, setFlash] = React.useState<
    { kind: 'ok' | 'err'; msg: string } | null
  >(null);

  // fetch docs
  React.useEffect(() => {
    let cancel = false;
    (async () => {
      setLoading(true);
      setErr(null);
      try {
        const res: DocumentListResponse = await listMyDocs(page, 20);
        if (cancel) return;
        setItems(res?.items ?? []); // safe when empty
        setTotalPages(res?.total_pages ?? 1);
      } catch (e: any) {
        if (!cancel) {
          setItems([]); // clear list on error
          setTotalPages(1);
          setErr(e?.message ?? 'Failed to load documents');
        }
      } finally {
        if (!cancel) setLoading(false);
      }
    })();
    return () => {
      cancel = true;
    };
  }, [page]);

  return (
    <>
      <Header avatarUrl={avatarUrl} username={me?.username} />

      <main className="page" style={{ alignItems: 'start' }}>
        <div className="card" style={{ width: 'min(1000px, 100%)' }}>
          <div
            style={{
              display: 'flex',
              justifyContent: 'space-between',
              alignItems: 'center',
            }}
          >
            <h2 style={{ margin: 0 }}>My Documents</h2>
            <button
              onClick={() => setCreateOpen(true)}
              style={{
                border: '1px solid #2a3760',
                background: '#1b2446',
                color: 'white',
                padding: '10px 14px',
                borderRadius: 10,
                cursor: 'pointer',
                fontWeight: 800,
              }}
            >
              + New Doc
            </button>
          </div>

          {/* flash */}
          {flash && (
            <div
              className={flash.kind === 'ok' ? 'card' : 'card error'}
              style={{
                marginTop: 10,
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

          {/* error */}
          {err && (
            <div className="error" style={{ marginTop: 12 }}>
              {err}
            </div>
          )}

          {/* list */}
          <div style={{ display: 'grid', gap: 12, marginTop: 12 }}>
            {loading && <div>Loading…</div>}
            {!loading && items.length === 0 && !err && (
              <div>No documents yet.</div>
            )}
            {!loading &&
              items.length > 0 &&
              items.map((d) => (
                <DocCard
                  key={d.id}
                  doc={d}
                  onOpen={() => nav(`/docs/${d.id}`)}
                />
              ))}
          </div>

          {/* pager */}
          <div
            style={{
              display: 'flex',
              gap: 8,
              marginTop: 12,
              alignItems: 'center',
            }}
          >
            <button
              onClick={() => setPage((p) => Math.max(1, p - 1))}
              disabled={page <= 1}
              style={btnMini}
            >
              Prev
            </button>
            <div style={{ opacity: 0.8 }}>
              Page {page} / {totalPages}
            </div>
            <button
              onClick={() => setPage((p) => Math.min(totalPages, p + 1))}
              disabled={page >= totalPages}
              style={btnMini}
            >
              Next
            </button>
          </div>
        </div>
      </main>

      <CreateDocModal
        open={createOpen}
        onClose={() => setCreateOpen(false)}
        onCreated={(_newId) => {
          setCreateOpen(false);
          setFlash({ kind: 'ok', msg: 'Document created.' });
          // refresh first page; listMyDocs uses current page
          setPage(1);
          // optional: navigate directly:
          // nav(`/docs/${_newId}`);
        }}
        onError={(m) => setFlash({ kind: 'err', msg: m })}
      />
    </>
  );
}

function DocCard({
  doc,
  onOpen,
}: {
  doc: DocumentRow;
  onOpen: () => void;
}) {
  const when = formatUnix(doc.updated_at);
  return (
    <article
      className="card"
      onClick={onOpen}
      style={{ padding: 14, cursor: 'pointer' }}
      title="Open document"
    >
      <div
        style={{
          display: 'flex',
          justifyContent: 'space-between',
          gap: 12,
          alignItems: 'center',
        }}
      >
        <div>
          <div style={{ fontWeight: 800, fontSize: 18 }}>
            {doc.title || '(Untitled)'}
          </div>
          <div style={{ opacity: 0.8, fontSize: 14 }}>
            Pages: {doc.page_count}
          </div>
        </div>
        <div style={{ color: '#97a3c7', fontSize: 13 }}>
          Updated {when}
        </div>
      </div>
    </article>
  );
}

function formatUnix(sec?: number | null): string {
  if (!Number.isFinite(sec ?? NaN)) return '';
  const d = new Date((sec as number) * 1000);
  const dd = String(d.getDate()).padStart(2, '0');
  const mm = String(d.getMonth() + 1).padStart(2, '0');
  const yyyy = d.getFullYear();
  const hh = String(d.getHours()).padStart(2, '0');
  const min = String(d.getMinutes()).padStart(2, '0');
  return `${dd}/${mm}/${yyyy}, ${hh}:${min}`;
}

const btnMini: React.CSSProperties = {
  border: '1px solid #2a3760',
  background: 'transparent',
  color: '#e6eaff',
  padding: '6px 10px',
  borderRadius: 8,
  cursor: 'pointer',
};