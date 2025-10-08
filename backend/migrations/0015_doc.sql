-- 01_documents.sql
PRAGMA foreign_keys=ON;

CREATE TABLE IF NOT EXISTS documents (
  id           INTEGER PRIMARY KEY,
  owner_id     INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  title        TEXT    NOT NULL DEFAULT '',
  page_count   INTEGER NOT NULL DEFAULT 1,
  created_at   INTEGER NOT NULL DEFAULT (unixepoch()),
  updated_at   INTEGER NOT NULL DEFAULT (unixepoch()),
  CHECK (page_count >= 1 AND page_count <= 10)
);

CREATE INDEX IF NOT EXISTS idx_document_owner   ON documents(owner_id);
CREATE INDEX IF NOT EXISTS idx_document_updated ON documents(updated_at DESC);