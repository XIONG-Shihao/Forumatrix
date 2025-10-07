import React from 'react';

type Props = {
  src: string;
  onClose: () => void;
  size?: number;
};

export const AvatarLightbox: React.FC<Props> = ({
  src,
  onClose,
  size = 512,
}) => {
  return (
    <div className="lb-backdrop" onClick={onClose}>
      <div className="lb-card" onClick={(e) => e.stopPropagation()}>
        <img
          src={src}
          alt="Avatar large"
          className="lb-img"
          style={{ width: size, height: size }}
        />
        <button className="btn" onClick={onClose} style={{ marginTop: 14 }}>
          Close
        </button>
      </div>
    </div>
  );
};
