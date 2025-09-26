-- Users
PRAGMA foreign_keys=ON;

CREATE TABLE IF NOT EXISTS users (
  id             INTEGER PRIMARY KEY,
  email          TEXT    NOT NULL UNIQUE,
  password_hash  TEXT    NOT NULL,
  username       TEXT    NOT NULL UNIQUE,
  dob            DATE    NULL,
  bio            TEXT    NULL,
  avatar_url     TEXT    NULL,
  is_active      INTEGER NOT NULL DEFAULT 1,  -- 1=true, 0=false
  is_admin       INTEGER NOT NULL DEFAULT 0,  -- 1=true, 0=false
  -- created/updated optional now; add later if you need
  CHECK (is_active IN (0,1)),
  CHECK (is_admin  IN (0,1))
);

-- Case-insensitive uniqueness for email (optional in SQLite; enforce in app)
-- You can normalize to lowercase in application code before insert.