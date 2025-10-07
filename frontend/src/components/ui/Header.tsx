// frontend/src/components/ui/Header.tsx
import { Link, useLocation, useNavigate } from 'react-router-dom';

//
// Tweak these numbers to resize the header controls.
//
const SIZES = {
  brandFont: 25, // “Arena-AI”
  avatar: 48, // avatar circle (px)
  usernameFont: 16, // if you ever render username text in header
  postBtnHeight: 48, // "+ Post" button height
  postBtnPad: '8px 12px', // "+ Post" button padding
  postBtnFont: 20, // "+ Post" button font
  bellSize: 48, // bell button width/height
  bellFont: 18, // bell icon font-size
};

type Props = {
  avatarUrl?: string | null;
  username?: string;
  showCompose?: boolean; // show "+ Post" (Home only)
  onCompose?: () => void;
  avatarClickable?: boolean; // false on /profile to avoid loop
  onBellClick?: () => void; // open notifications panel
};

export function Header({
  avatarUrl,
  username,
  showCompose = false,
  onCompose,
  avatarClickable = true,
  onBellClick,
}: Props) {
  const nav = useNavigate();
  const { pathname } = useLocation();

  const goHome = () => nav('/home');
  const isDocs = pathname.startsWith('/docs');

  return (
    <header
      style={{
        position: 'sticky',
        top: 0,
        zIndex: 40,
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'space-between',
        padding: '10px 18px',
        background: 'rgba(10,12,20,.6)',
        backdropFilter: 'blur(8px)',
        borderBottom: '1px solid rgba(255,255,255,.06)',
      }}
    >
      {/* Left: brand + Docs link */}
      <div style={{ display: 'flex', alignItems: 'center', gap: 12 }}>
        <button
          onClick={goHome}
          title="Go to Home"
          style={{
            fontWeight: 800,
            letterSpacing: 0.2,
            fontSize: SIZES.brandFont,
            color: 'white',
            background: 'transparent',
            border: 'none',
            cursor: 'pointer',
          }}
        >
          Arena-AI
        </button>

        <Link
          to="/docs"
          title="My Documents"
          style={{
            border: '1px solid #2a3760',
            background: isDocs ? '#21305f' : '#111a39',
            color: '#e6eaff',
            padding: '6px 10px',
            borderRadius: 8,
            textDecoration: 'none',
            fontWeight: 800,
            fontSize: 20,
          }}
        >
          Docs
        </Link>
      </div>

      {/* Right cluster */}
      <div
        style={{
          display: 'flex',
          alignItems: 'center',
          gap: 10,
        }}
      >
        {showCompose && (
          <button
            onClick={onCompose}
            title="Create a post"
            style={{
              display: 'inline-flex',
              alignItems: 'center',
              height: SIZES.postBtnHeight,
              padding: SIZES.postBtnPad,
              borderRadius: 10,
              background: '#3b82f6',
              color: 'white',
              border: '1px solid #3b82f6',
              fontWeight: 800,
              fontSize: SIZES.postBtnFont,
              cursor: 'pointer',
            }}
          >
            + Post
          </button>
        )}

        <button
          type="button"
          title="Notifications"
          onClick={onBellClick}
          style={{
            height: SIZES.bellSize,
            width: SIZES.bellSize,
            borderRadius: 10,
            background: '#1b2446',
            border: '1px solid #2a3760',
            color: '#e6eaff',
            cursor: 'pointer',
            display: 'inline-grid',
            placeItems: 'center',
            fontSize: SIZES.bellFont,
          }}
        >
          🔔
        </button>

        {avatarClickable ? (
          <Link to="/profile" style={{ display: 'inline-flex' }}>
            <AvatarThumb src={avatarUrl ?? undefined} name={username} />
          </Link>
        ) : (
          <span>
            <AvatarThumb src={avatarUrl ?? undefined} name={username} />
          </span>
        )}
      </div>
    </header>
  );
}

function AvatarThumb({ src, name }: { src?: string; name?: string }) {
  const initial = name?.trim()[0]?.toUpperCase() ?? '•';
  return src ? (
    <img
      src={src}
      alt={name ?? ''}
      style={{
        width: SIZES.avatar,
        height: SIZES.avatar,
        borderRadius: '50%',
        objectFit: 'cover',
        border: '1px solid #2a3760',
      }}
    />
  ) : (
    <div
      style={{
        width: SIZES.avatar,
        height: SIZES.avatar,
        borderRadius: '50%',
        display: 'grid',
        placeItems: 'center',
        background: '#3b82f6',
        color: 'white',
        fontWeight: 700,
        userSelect: 'none',
        fontSize: Math.max(12, SIZES.avatar * 0.45),
      }}
      title={name}
    >
      {initial}
    </div>
  );
}
