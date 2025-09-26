-- Sessions for auth (cookie-based)
PRAGMA foreign_keys=ON;

CREATE TABLE IF NOT EXISTS sessions (
  id          TEXT    PRIMARY KEY,  -- random opaque token; store only a hashed form in production
  user_id     INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  created_at  INTEGER NOT NULL DEFAULT (unixepoch()),
  expires_at  INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_sessions_user_id     ON sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_sessions_expires_at  ON sessions(expires_at);