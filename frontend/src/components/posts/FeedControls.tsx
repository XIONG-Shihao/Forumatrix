// frontend/src/components/posts/FeedControls.tsx
import type { Sort } from '../../features/posts/api';

type Props = {
  sort: Sort;
  onSort: (s: Sort) => void;
  page: number;
  totalPages: number;
  onFirst: () => void;
  onPrev: () => void;
  onNext: () => void;
  onLast: () => void;
};

export function FeedControls({
  sort,
  onSort,
  page,
  totalPages,
  onFirst,
  onPrev,
  onNext,
  onLast,
}: Props) {
  return (
    <div style={wrap}>
      <div style={tabs}>
        <Tab active={sort === 'latest'} onClick={() => onSort('latest')}>
          Latest
        </Tab>
        <Sep />
        <Tab active={sort === 'popular'} onClick={() => onSort('popular')}>
          Popular
        </Tab>
        <Sep />
        <Tab
          active={sort === 'controversial'}
          onClick={() => onSort('controversial')}
        >
          Controversial
        </Tab>
      </div>

      <div style={pager}>
        <Btn onClick={onFirst} disabled={page <= 1}>
          &laquo;
        </Btn>
        <Btn onClick={onPrev} disabled={page <= 1}>
          &lsaquo;
        </Btn>
        <span style={{ opacity: 0.8 }}>
          Page {page} / {Math.max(1, totalPages)}
        </span>
        <Btn onClick={onNext} disabled={page >= totalPages}>
          &rsaquo;
        </Btn>
        <Btn onClick={onLast} disabled={page >= totalPages}>
          &raquo;
        </Btn>
      </div>
    </div>
  );
}

function Tab({ active, children, onClick }: any) {
  return (
    <button
      onClick={onClick}
      style={{
        border: 'none',
        background: active ? 'rgba(79,70,229,.18)' : 'transparent',
        color: '#e6eaff',
        padding: '8px 10px',
        borderRadius: 8,
        cursor: 'pointer',
        fontSize: 'var(--feed-tab-size)', // 👈 added
        fontWeight: 600, // (optional) bolder tabs
      }}
    >
      {children}
    </button>
  );
}
const Sep = () => <span style={{ opacity: 0.35, margin: '0 6px' }}>|</span>;
const Btn = (p: any) => (
  <button
    {...p}
    style={{
      border: '1px solid #2a3760',
      background: 'transparent',
      color: '#e6eaff',
      padding: '6px 10px',
      borderRadius: 8,
      cursor: p.disabled ? 'default' : 'pointer',
      opacity: p.disabled ? 0.5 : 1,
    }}
  />
);

const wrap = {
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'space-between',
  gap: 12,
} as const;
const tabs = { display: 'flex', alignItems: 'center' } as const;
const pager = { display: 'flex', alignItems: 'center', gap: 8 } as const;
