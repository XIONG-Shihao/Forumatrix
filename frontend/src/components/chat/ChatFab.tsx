// frontend/src/components/chat/ChatFab.tsx
import React from 'react';

export function ChatFab({
  onClick,
  count = 0,
  size = 66, // ← control button diameter
  fontSize = 30, // ← control emoji size
  right = 36, // ← distance from right edge
  bottom = 36, // ← distance from bottom
}: {
  onClick: () => void;
  count?: number;
  size?: number;
  fontSize?: number;
  right?: number;
  bottom?: number;
}) {
  const showBadge = count > 0;

  return (
    <button
      type="button"
      onClick={onClick}
      title="Open chat"
      aria-label="Open chat"
      style={{
        position: 'fixed',
        right,
        bottom,
        height: size,
        width: size,
        borderRadius: size / 2,
        background: '#4f46e5',
        border: '1px solid rgba(255,255,255,.15)',
        color: 'white',
        fontSize,
        display: 'grid',
        placeItems: 'center',
        cursor: 'pointer',
        zIndex: 60,
        boxShadow: '0 10px 25px rgba(0,0,0,.35)',
      }}
    >
      💬
      {showBadge && (
        <span
          style={{
            position: 'absolute',
            top: -6,
            right: -6,
            minWidth: 18,
            height: 18,
            padding: '0 4px',
            borderRadius: 9,
            background: '#ef4444',
            color: '#fff',
            fontSize: 12,
            lineHeight: '18px',
            textAlign: 'center',
            border: '1px solid rgba(0,0,0,.35)',
          }}
        >
          {count > 99 ? '99+' : count}
        </span>
      )}
    </button>
  );
}
