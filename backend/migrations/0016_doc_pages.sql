-- 02_document_pages.sql
-- One row per page. We store the merged Yjs/yrs update in y_update (binary).
-- Fixed styles: 1 = Title, 2 = Heading, 3 = Body
CREATE TABLE IF NOT EXISTS document_pages (
  id           INTEGER PRIMARY KEY,
  doc_id       INTEGER NOT NULL REFERENCES documents(id) ON DELETE CASCADE,
  page_index   INTEGER NOT NULL,                       -- 0..9
  style        INTEGER NOT NULL DEFAULT 3,             -- 1=Title,2=Heading,3=Body
  y_update     BLOB    NOT NULL DEFAULT x'',           -- merged CRDT state (yrs update)
  created_at   INTEGER NOT NULL DEFAULT (unixepoch()),
  updated_at   INTEGER NOT NULL DEFAULT (unixepoch()),
  CHECK (page_index BETWEEN 0 AND 9),
  CHECK (style IN (1,2,3)),
  UNIQUE(doc_id, page_index)
);

CREATE INDEX IF NOT EXISTS idx_doc_pages_doc_idx ON document_pages(doc_id, page_index);
CREATE INDEX IF NOT EXISTS idx_doc_pages_updated ON document_pages(updated_at DESC);