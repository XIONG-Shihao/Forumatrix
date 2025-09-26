-- Posts
PRAGMA foreign_keys=ON;

CREATE TABLE IF NOT EXISTS posts (
  id             INTEGER PRIMARY KEY,
  user_id        INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  title          TEXT    NOT NULL,
  body           TEXT    NOT NULL,
  created_at     INTEGER NOT NULL DEFAULT (unixepoch()),
  updated_at     INTEGER NULL,
  edited         INTEGER NOT NULL DEFAULT 0,
  score          INTEGER NOT NULL DEFAULT 0,
  comment_count  INTEGER NOT NULL DEFAULT 0,
  CHECK (edited IN (0,1))
);

-- Helpful listing indexes
CREATE INDEX IF NOT EXISTS idx_posts_user_id          ON posts(user_id);
CREATE INDEX IF NOT EXISTS idx_posts_created_at_desc  ON posts(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_posts_score_desc       ON posts(score DESC);