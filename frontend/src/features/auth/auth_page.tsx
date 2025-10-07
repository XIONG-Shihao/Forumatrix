// frontend/src/features/auth/auth_page.tsx
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { authApi } from './api';
import { saveAuth } from './state';

type Tab = 'login' | 'register';

export default function AuthPage() {
  const [tab, setTab] = useState<Tab>('login');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const nav = useNavigate();

  // login form
  const [identifier, setIdentifier] = useState('');
  const [password, setPassword] = useState('');

  // register form
  const [email, setEmail] = useState('');
  const [username, setUsername] = useState('');
  const [regPassword, setRegPassword] = useState('');
  const [dob, setDob] = useState('');
  const [bio, setBio] = useState('');

  function switchTab(next: Tab) {
    setTab(next);
    setError(null);
  }

  async function doLogin() {
    setError(null);
    setLoading(true);
    try {
      const res = await authApi.login({
        identifier,
        password,
      });
      // persist lightweight client auth state for the UI
      saveAuth({
        userId: res.user_id,
        username: res.username,
        email: res.email,
      });
      nav('/', { replace: true });
    } catch (e: any) {
      setError(e?.message ?? 'Login failed');
    } finally {
      setLoading(false);
    }
  }

  async function doRegister() {
    setError(null);
    setLoading(true);
    try {
      // 1) create account
      await authApi.register({
        email,
        username,
        password: regPassword,
        dob: dob || undefined,
        bio: bio || undefined,
      });

      // 2) immediately sign in (server sets session cookie)
      const res = await authApi.login({
        identifier: email, // you can also allow username here
        password: regPassword,
      });

      // 3) persist client auth for UI
      saveAuth({
        userId: res.user_id,
        username: res.username,
        email: res.email,
      });
      nav('/', { replace: true });
    } catch (e: any) {
      setError(e?.message ?? 'Register failed');
    } finally {
      setLoading(false);
    }
  }

  return (
    <div style={styles.wrap}>
      <div style={styles.card}>
        <h1 style={{ margin: 0 }}>Arena-AI</h1>
        <p style={{ marginTop: 8, opacity: 0.75 }}>
          minimal auth to talk to your Rust backend
        </p>

        <div style={styles.tabs}>
          <button
            style={{
              ...styles.tabBtn,
              ...(tab === 'login' ? styles.tabActive : {}),
            }}
            onClick={() => switchTab('login')}
            aria-pressed={tab === 'login'}
          >
            Login
          </button>
          <button
            style={{
              ...styles.tabBtn,
              ...(tab === 'register' ? styles.tabActive : {}),
            }}
            onClick={() => switchTab('register')}
            aria-pressed={tab === 'register'}
          >
            Register
          </button>
        </div>

        {error && <div style={styles.error}>{error}</div>}

        {tab === 'login' ? (
          <form
            onSubmit={(e) => {
              e.preventDefault();
              if (!loading) void doLogin();
            }}
            style={styles.form}
          >
            <label style={styles.label}>Email or Username</label>
            <input
              style={styles.input}
              value={identifier}
              onChange={(e) => setIdentifier(e.target.value)}
              placeholder="you@example.com or yourname"
              autoComplete="username"
              required
            />

            <label style={styles.label}>Password</label>
            <input
              style={styles.input}
              type="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              autoComplete="current-password"
              required
            />

            <button style={styles.primaryBtn} disabled={loading}>
              {loading ? 'Signing in…' : 'Sign in'}
            </button>
          </form>
        ) : (
          <form
            onSubmit={(e) => {
              e.preventDefault();
              if (!loading) void doRegister();
            }}
            style={styles.form}
          >
            <label style={styles.label}>Email</label>
            <input
              style={styles.input}
              type="email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              autoComplete="email"
              required
            />

            <label style={styles.label}>Username</label>
            <input
              style={styles.input}
              value={username}
              onChange={(e) => setUsername(e.target.value)}
              autoComplete="username"
              required
            />

            <label style={styles.label}>Password</label>
            <input
              style={styles.input}
              type="password"
              value={regPassword}
              onChange={(e) => setRegPassword(e.target.value)}
              autoComplete="new-password"
              minLength={8}
              required
            />

            <label style={styles.label}>Date of Birth (optional)</label>
            <input
              style={styles.input}
              type="date"
              lang="en-CA" // 👈 forces YYYY-MM-DD UI on most browsers
              placeholder="YYYY-MM-DD" // visual hint if the browser shows a textbox
              value={dob}
              onChange={(e) => setDob(e.target.value)}
              min="1900-01-01"
              max="2100-12-31"
            />

            <label style={styles.label}>Bio (optional)</label>
            <textarea
              style={styles.textarea}
              rows={3}
              value={bio}
              onChange={(e) => setBio(e.target.value)}
            />

            <button style={styles.primaryBtn} disabled={loading}>
              {loading ? 'Creating…' : 'Create account'}
            </button>
          </form>
        )}
      </div>
    </div>
  );
}

const styles: Record<string, React.CSSProperties> = {
  wrap: {
    minHeight: '100dvh',
    display: 'grid',
    placeItems: 'center',
    background: '#0b1020',
    color: 'white',
    padding: 16,
  },
  card: {
    width: '100%',
    maxWidth: 520,
    background: '#121831',
    border: '1px solid #1f2a4d',
    borderRadius: 16,
    padding: 24,
    boxShadow: '0 10px 30px rgba(0,0,0,0.25)',
  },
  tabs: {
    display: 'flex',
    gap: 8,
    marginTop: 16,
    marginBottom: 12,
  },
  tabBtn: {
    flex: 1,
    background: 'transparent',
    border: '1px solid #2a3663',
    color: '#c9d1ff',
    padding: '10px 12px',
    borderRadius: 10,
    cursor: 'pointer',
  },
  tabActive: {
    background: '#1b2446',
    borderColor: '#5163ff',
  },
  error: {
    background: '#3a0d0d',
    color: '#ffb4b4',
    border: '1px solid #5b1919',
    borderRadius: 10,
    padding: 10,
    marginBottom: 10,
    whiteSpace: 'pre-wrap',
  },
  form: {
    display: 'grid',
    gap: 8,
  },
  label: { fontSize: 12, opacity: 0.75 },
  input: {
    background: '#0f1530',
    border: '1px solid #273057',
    color: '#e6eaff',
    padding: '10px 12px',
    borderRadius: 10,
    outline: 'none',
  },
  textarea: {
    background: '#0f1530',
    border: '1px solid #273057',
    color: '#e6eaff',
    padding: '10px 12px',
    borderRadius: 10,
    outline: 'none',
    resize: 'vertical',
  },
  primaryBtn: {
    marginTop: 6,
    background: '#5163ff',
    border: '1px solid #5163ff',
    color: 'white',
    padding: '10px 12px',
    borderRadius: 12,
    fontWeight: 600,
    cursor: 'pointer',
  },
};
