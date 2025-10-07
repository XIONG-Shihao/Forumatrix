import React from 'react';

export function NotificationBell({
  count,
  onClick,
  right = 90, // position near avatar; tweak in Home.tsx if needed
}: {
  count: number;
  onClick: () => void;
  right?: number;
}) {
  return (
    <button
      type="button"
      title="Notifications"
      onClick={onClick}
      style={{
        position: 'fixed',
        top: 11.5,
        right,
        display: 'inline-flex',
        alignItems: 'center',
        justifyContent: 'center',
        height: 38,
        width: 38,
        borderRadius: 10,
        background: '#1b2446',
        border: '1px solid #2a3760',
        color: '#e6eaff',
        cursor: 'pointer',
        zIndex: 20,
      }}
    >
      <span style={{ fontSize: 18, lineHeight: 1 }}>🔔</span>
      {count > 0 && (
        <span
          style={{
            position: 'absolute',
            top: -4,
            right: -4,
            minWidth: 18,
            height: 18,
            borderRadius: 9,
            background: '#ef4444',
            color: 'white',
            fontSize: 13,
            fontWeight: 700,
            display: 'grid',
            placeItems: 'center',
            padding: '0 4px',
            border: '1px solid #0b1020',
          }}
        >
          {count > 99 ? '99+' : count}
        </span>
      )}
    </button>
  );
}
