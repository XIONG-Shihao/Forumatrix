-- 05_document_join_requests.sql
CREATE TABLE IF NOT EXISTS document_join_requests (
  id         INTEGER PRIMARY KEY,
  doc_id     INTEGER NOT NULL REFERENCES documents(id) ON DELETE CASCADE,
  user_id    INTEGER NOT NULL REFERENCES users(id)     ON DELETE CASCADE,
  status     INTEGER NOT NULL DEFAULT 0,  -- 0=pending, 1=approved, 2=denied
  message    TEXT     NULL,               -- optional requester note
  created_at INTEGER NOT NULL DEFAULT (unixepoch()),
  decided_at INTEGER NULL,
  decided_by INTEGER NULL REFERENCES users(id) ON DELETE SET NULL,
  UNIQUE(doc_id, user_id)
);

CREATE INDEX IF NOT EXISTS idx_join_req_doc_status ON document_join_requests(doc_id, status);
CREATE INDEX IF NOT EXISTS idx_join_req_user       ON document_join_requests(user_id);