// src/pages/docs/DocPage.tsx
import React from 'react';
import { useParams } from 'react-router-dom';
import { Header } from '../../components/ui/Header';
import { getAuth } from '../../features/auth/state';
import { getUser } from '../../lib/api';
import type { UserPublic } from '../../lib/types';
import {
  getDocMeta,
  openPage,
  upsertPage,
  createJoinRequest,
  type DocumentMeta,
  type PageOpenPayload,
} from '../../features/docs/api';

type Style = 1 | 2 | 3;

type PageVM = {
  index: number;
  style: Style;
  text: string;
  loading: boolean;
  dirty: boolean;
  saving: boolean;
  error?: string | null;
};

const A4_MAX_W = 900; // px max width when centered
const PAGE_GAP = 40;

export default function DocPage() {
  const { docId: docIdParam } = useParams();
  const docId = Number(docIdParam);

  // header avatar
  const auth = getAuth();
  const [me, setMe] = React.useState<UserPublic | null>(null);
  React.useEffect(() => {
    (async () => {
      if (!auth?.userId) return;
      try {
        setMe(await getUser(auth.userId));
      } catch {}
    })();
  }, [auth?.userId]);
  const avatarUrl =
    me?.avatar_url ?? (me ? `/static/avatars/${me.id}.png` : undefined);

  const [meta, setMeta] = React.useState<DocumentMeta | null>(null);
  const [pages, setPages] = React.useState<PageVM[]>([]);
  const [loadingMeta, setLoadingMeta] = React.useState(true);
  const [banner, setBanner] = React.useState<string | null>(null);
  const [notMember, setNotMember] = React.useState(false);
  const [savingAll, setSavingAll] = React.useState(false);

  // Load meta, then all page contents (<=10; cheap)
  React.useEffect(() => {
    let cancelled = false;
    (async () => {
      setLoadingMeta(true);
      setBanner(null);
      setNotMember(false);
      try {
        const m = await getDocMeta(docId);
        if (cancelled) return;
        setMeta(m);
        // Seed VMs from meta (style comes from meta.pages; text lazy -> load below)
        const seed: PageVM[] = (m.pages ?? []).map((p) => ({
          index: p.page_index,
          style: (p.style as Style) ?? 3,
          text: '',
          loading: true,
          dirty: false,
          saving: false,
        }));
        setPages(seed);

        // Load all pages
        const loaded = await Promise.all(
          seed.map(async (vm) => {
            try {
              const payload: PageOpenPayload = await openPage(docId, vm.index);
              const bytes = new Uint8Array(payload.y_update as any);
              const txt = bytes.length ? new TextDecoder().decode(bytes) : '';
              return {
                ...vm,
                text: txt,
                style: (payload.style as Style) ?? vm.style,
                loading: false,
              };
            } catch (e: any) {
              return {
                ...vm,
                loading: false,
                error: String(e?.message || e || 'Load failed'),
              };
            }
          })
        );
        if (!cancelled) setPages(loaded);
      } catch (e: any) {
        const msg = String(e?.message || e || 'Failed to load document');
        if (/not an editor/i.test(msg)) {
          setNotMember(true);
          setBanner(
            'You are not a member of this document. Ask the owner to add you, or request access.'
          );
        } else {
          setBanner(msg);
        }
      } finally {
        if (!cancelled) setLoadingMeta(false);
      }
    })();
    return () => {
      cancelled = true;
    };
  }, [docId]);

  function updatePage(idx: number, patch: Partial<PageVM>) {
    setPages((prev) =>
      prev.map((p) => (p.index === idx ? { ...p, ...patch } : p))
    );
  }

  async function saveOne(idx: number) {
    const p = pages.find((x) => x.index === idx);
    if (!p || p.saving || !p.dirty) return;
    updatePage(idx, { saving: true, error: null });
    try {
      const bytes = new TextEncoder().encode(p.text);
      const y_update_base64 = btoa(String.fromCharCode(...bytes));
      await upsertPage(docId, idx, { style: p.style, y_update_base64 });
      updatePage(idx, { dirty: false, saving: false });
    } catch (e: any) {
      updatePage(idx, {
        saving: false,
        error: String(e?.message || e || 'Save failed'),
      });
    }
  }

  async function saveAll() {
    setSavingAll(true);
    setBanner(null);
    try {
      for (const p of pages) {
        if (p.dirty) {
          const bytes = new TextEncoder().encode(p.text);
          const y_update_base64 = btoa(String.fromCharCode(...bytes));
          await upsertPage(docId, p.index, { style: p.style, y_update_base64 });
          updatePage(p.index, { dirty: false });
        }
      }
    } catch (e: any) {
      setBanner(String(e?.message || e || 'Save failed'));
    } finally {
      setSavingAll(false);
    }
  }

  async function requestAccess() {
    try {
      await createJoinRequest(docId, 'Please grant me edit access.');
      setBanner('Access request sent to the owner.');
    } catch (e: any) {
      setBanner(String(e?.message || e || 'Failed to request access'));
    }
  }

  return (
    <>
      <Header avatarUrl={avatarUrl} username={me?.username} />

      <div style={{ padding: '16px 24px 40px' }}>
        {/* Banner */}
        {banner && (
          <div
            role="alert"
            style={{
              margin: '0 auto 16px',
              maxWidth: A4_MAX_W,
              padding: '10px 12px',
              borderRadius: 10,
              border: '1px solid #5a2a2a',
              background: '#2a0f10',
              color: '#ffd8d8',
              fontSize: 14,
            }}
          >
            {banner}
            {notMember && (
              <button
                onClick={requestAccess}
                style={{
                  marginLeft: 10,
                  border: '1px solid #2a3760',
                  background: '#1b2446',
                  color: '#fff',
                  padding: '6px 10px',
                  borderRadius: 8,
                  cursor: 'pointer',
                }}
              >
                Request Access
              </button>
            )}
          </div>
        )}

        {/* Top toolbar */}
        <div
          style={{
            margin: '0 auto 12px',
            maxWidth: A4_MAX_W,
            display: 'flex',
            alignItems: 'center',
            gap: 12,
            background: '#0f1530',
            border: '1px solid #243156',
            borderRadius: 12,
            padding: 10,
          }}
        >
          <div style={{ fontWeight: 800 }}>
            {loadingMeta ? 'Loading…' : meta?.title || '(Untitled)'}
          </div>
          <div style={{ flex: 1 }} />
          <button
            onClick={saveAll}
            disabled={savingAll || notMember || !pages.some((p) => p.dirty)}
            style={{
              border: '1px solid #2a3760',
              background: savingAll ? '#1a2140' : '#1b2446',
              color: '#fff',
              padding: '8px 12px',
              borderRadius: 10,
              cursor:
                savingAll || notMember || !pages.some((p) => p.dirty)
                  ? 'default'
                  : 'pointer',
              fontWeight: 700,
            }}
            title={notMember ? 'Join this doc to save' : 'Save all dirty pages'}
          >
            {savingAll
              ? 'Saving…'
              : pages.some((p) => p.dirty)
              ? 'Save All'
              : 'Saved'}
          </button>
        </div>

        {/* Continuous pages */}
        <div
          style={{
            display: 'grid',
            gap: PAGE_GAP,
            justifyItems: 'center',
          }}
        >
          {loadingMeta && (
            <div style={{ color: '#9aa3c6' }}>Loading document…</div>
          )}

          {!loadingMeta &&
            pages.map((p) => (
              <PageBlock
                key={p.index}
                vm={p}
                disabled={notMember}
                onChangeText={(t) =>
                  updatePage(p.index, { text: t, dirty: true })
                }
                onChangeStyle={(s) =>
                  updatePage(p.index, { style: s, dirty: true })
                }
                onSave={() => saveOne(p.index)}
              />
            ))}
        </div>
      </div>
    </>
  );
}

function PageBlock({
  vm,
  disabled,
  onChangeText,
  onChangeStyle,
  onSave,
}: {
  vm: PageVM;
  disabled: boolean;
  onChangeText: (t: string) => void;
  onChangeStyle: (s: Style) => void;
  onSave: () => void;
}) {
  return (
    <div style={{ width: '100%', maxWidth: A4_MAX_W }}>
      {/* Page header (style + save for this page) */}
      <div
        style={{
          display: 'flex',
          alignItems: 'center',
          gap: 8,
          marginBottom: 8,
          color: '#9aa3c6',
          fontSize: 13,
        }}
      >
        <span>Page {vm.index + 1}</span>
        <span style={{ marginLeft: 10 }}>Style:</span>
        <Seg
          label="Title"
          on={() => onChangeStyle(1)}
          active={vm.style === 1}
        />
        <Seg
          label="Heading"
          on={() => onChangeStyle(2)}
          active={vm.style === 2}
        />
        <Seg label="Body" on={() => onChangeStyle(3)} active={vm.style === 3} />
        <div style={{ flex: 1 }} />
        <button
          onClick={onSave}
          disabled={disabled || vm.saving || !vm.dirty}
          style={{
            border: '1px solid #2a3760',
            background: vm.saving ? '#1a2140' : '#1b2446',
            color: '#fff',
            padding: '6px 10px',
            borderRadius: 8,
            cursor: disabled || vm.saving || !vm.dirty ? 'default' : 'pointer',
            fontWeight: 700,
          }}
        >
          {vm.saving ? 'Saving…' : vm.dirty ? 'Save' : 'Saved'}
        </button>
      </div>

      {/* A4 page */}
      <div
        style={{
          background: '#fff',
          color: '#000',
          borderRadius: 6,
          boxShadow: '0 8px 30px rgba(0,0,0,.35)',
          border: '1px solid #e7e7e7',
          width: '100%',
          aspectRatio: '794 / 1123',
          padding: 24,
          overflow: 'hidden',
        }}
      >
        {vm.loading ? (
          <div style={{ color: '#444' }}>Loading…</div>
        ) : (
          <textarea
            value={vm.text}
            onChange={(e) => onChangeText(e.target.value)}
            disabled={disabled}
            placeholder="Start typing…"
            style={{
              width: '100%',
              height: '100%',
              border: 'none',
              outline: 'none',
              resize: 'none',
              background: 'transparent',
              color: '#000',
              overflow: 'hidden',
              fontFamily:
                vm.style === 1
                  ? 'ui-serif, Georgia, serif'
                  : 'ui-sans-serif, system-ui, -apple-system, Segoe UI, Roboto, Arial',
              fontWeight: vm.style === 1 ? 800 : vm.style === 2 ? 700 : 400,
              fontSize: vm.style === 1 ? 28 : vm.style === 2 ? 22 : 16,
              lineHeight: vm.style === 3 ? 1.55 : 1.35,
            }}
          />
        )}
      </div>

      {vm.error && (
        <div
          role="alert"
          style={{
            marginTop: 8,
            padding: '8px 10px',
            borderRadius: 8,
            border: '1px solid #5a2a2a',
            background: '#2a0f10',
            color: '#ffd8d8',
            fontSize: 13,
          }}
        >
          {vm.error}
        </div>
      )}
    </div>
  );
}

function Seg({
  label,
  on,
  active,
}: {
  label: string;
  on: () => void;
  active: boolean;
}) {
  return (
    <button
      onClick={on}
      style={{
        border: '1px solid #2a3760',
        background: active ? '#1b2446' : 'transparent',
        color: '#e6eaff',
        padding: '4px 10px',
        borderRadius: 8,
        cursor: 'pointer',
        fontWeight: 700,
        fontSize: 13,
      }}
    >
      {label}
    </button>
  );
}
