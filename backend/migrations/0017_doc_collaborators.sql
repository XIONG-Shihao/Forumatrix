-- 03_document_collaborators.sql
CREATE TABLE IF NOT EXISTS document_collaborators (
  doc_id   INTEGER NOT NULL REFERENCES documents(id) ON DELETE CASCADE,
  user_id  INTEGER NOT NULL REFERENCES users(id)     ON DELETE CASCADE,
  role     INTEGER NOT NULL DEFAULT 2,  -- always 2 = editor
  added_at INTEGER NOT NULL DEFAULT (unixepoch()),
  PRIMARY KEY (doc_id, user_id),
  CHECK (role = 2)
);

CREATE INDEX IF NOT EXISTS idx_doc_collab_doc ON document_collaborators(doc_id);
CREATE INDEX IF NOT EXISTS idx_doc_collab_user ON document_collaborators(user_id);