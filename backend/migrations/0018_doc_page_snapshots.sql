-- 04_document_page_snapshots.sql (optional, rolling safety/history; keep small)
CREATE TABLE IF NOT EXISTS document_page_snapshots (
  id           INTEGER PRIMARY KEY,
  doc_id       INTEGER NOT NULL REFERENCES documents(id) ON DELETE CASCADE,
  page_index   INTEGER NOT NULL,
  snapshot     BLOB    NOT NULL,                       -- full encoded yrs state (or snapshot)
  state_vec    BLOB    NULL,                           -- optional yrs state vector
  created_at   INTEGER NOT NULL DEFAULT (unixepoch()),
  CHECK (page_index BETWEEN 0 AND 9)
);

CREATE INDEX IF NOT EXISTS idx_snap_doc_idx_time ON document_page_snapshots(doc_id, page_index, created_at DESC);