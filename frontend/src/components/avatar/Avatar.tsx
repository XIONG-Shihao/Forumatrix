import React, { useEffect, useState } from 'react';

type AvatarProps = {
  src?: string;
  fallbackInitial?: string;
  size?: number; // px
  onClick?: () => void;
  title?: string;
  className?: string;
};

export const Avatar: React.FC<AvatarProps> = ({
  src,
  fallbackInitial = '?',
  size = 40,
  onClick,
  title,
  className,
}) => {
  // If the image ever errors, show the fallback initial.
  const [errored, setErrored] = useState(false);

  // IMPORTANT: when src changes (e.g. after upload with ?v=123), allow retry.
  useEffect(() => {
    setErrored(false);
  }, [src]);

  return (
    <button
      onClick={onClick}
      title={title}
      className={className}
      style={{
        width: size,
        height: size,
        borderRadius: '50%',
        border: '1px solid #273059',
        background: 'transparent',
        padding: 0,
        position: 'relative',
        cursor: onClick ? 'pointer' : 'default',
        overflow: 'hidden',
        display: 'inline-grid',
        placeItems: 'center',
      }}
    >
      {src && !errored ? (
        <img
          key={src} // force a fresh fetch/render when cache-busting query changes
          src={src}
          alt=""
          style={{
            width: '100%',
            height: '100%',
            objectFit: 'cover',
            display: 'block',
            borderRadius: '50%',
          }}
          onError={() => setErrored(true)}
        />
      ) : (
        <span
          aria-hidden
          style={{
            width: '100%',
            height: '100%',
            display: 'grid',
            placeItems: 'center',
            background: '#4f46e5',
            color: '#fff',
            fontWeight: 800,
            fontSize: Math.round(size * 0.45),
            borderRadius: '50%',
            lineHeight: 1,
          }}
        >
          {fallbackInitial}
        </span>
      )}
    </button>
  );
};
