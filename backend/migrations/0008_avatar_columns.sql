-- Minimal avatar metadata on users (no color/seed persisted)

ALTER TABLE users
  ADD COLUMN avatar_kind TEXT NOT NULL DEFAULT 'generated'
  CHECK (avatar_kind IN ('generated', 'upload'));

ALTER TABLE users
  ADD COLUMN avatar_rev INTEGER NOT NULL DEFAULT 0;

-- Optional (nice to have if you’ll ever vary the output format):
ALTER TABLE users
  ADD COLUMN avatar_mime TEXT NULL; -- e.g., 'image/webp'