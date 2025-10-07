import { useState } from 'react';

type Props = {
  src?: string | null;
  name?: string;
  size?: number; // px
};

export function Avatar({ src, name = '', size = 36 }: Props) {
  const [error, setError] = useState(false);
  const initial = name.trim()[0]?.toUpperCase() ?? '?';

  if (!src || error) {
    // fallback circle with initial
    return (
      <div
        title={name}
        style={{
          width: size,
          height: size,
          borderRadius: '50%',
          display: 'grid',
          placeItems: 'center',
          background: '#3b82f6',
          color: 'white',
          fontWeight: 700,
          userSelect: 'none',
        }}
      >
        {initial}
      </div>
    );
  }

  return (
    <img
      src={src}
      title={name}
      alt={name}
      width={size}
      height={size}
      onError={() => setError(true)}
      style={{
        width: size,
        height: size,
        borderRadius: '50%',
        objectFit: 'cover',
      }}
    />
  );
}
