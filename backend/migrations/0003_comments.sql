-- Comments (nested with parent_id)
PRAGMA foreign_keys=ON;

CREATE TABLE IF NOT EXISTS comments (
  id          INTEGER PRIMARY KEY,
  post_id     INTEGER NOT NULL REFERENCES posts(id)   ON DELETE CASCADE,
  user_id     INTEGER NOT NULL REFERENCES users(id)   ON DELETE CASCADE,
  parent_id   INTEGER NULL    REFERENCES comments(id) ON DELETE CASCADE,
  body        TEXT    NOT NULL,
  created_at  INTEGER NOT NULL DEFAULT (unixepoch()),
  updated_at  INTEGER NULL,
  deleted_at  INTEGER NULL,
  edited      INTEGER NOT NULL DEFAULT 0,
  score       INTEGER NOT NULL DEFAULT 0,
  CHECK (edited IN (0,1))
);

-- Threaded retrieval & sort helpers
CREATE INDEX IF NOT EXISTS idx_comments_post_parent_created ON comments(post_id, parent_id, created_at);
CREATE INDEX IF NOT EXISTS idx_comments_post_score           ON comments(post_id, score DESC);
