-- Reports for moderation
PRAGMA foreign_keys=ON;

CREATE TABLE IF NOT EXISTS reports (
  id           INTEGER PRIMARY KEY,
  reporter_id  INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  entity_type  TEXT    NOT NULL CHECK (entity_type IN ('post','comment','user')),
  entity_id    INTEGER NOT NULL,
  reason       TEXT    NOT NULL,
  created_at   INTEGER NOT NULL DEFAULT (unixepoch()),
  status       TEXT    NOT NULL DEFAULT 'open' CHECK (status IN ('open','reviewed','dismissed','actioned'))
);

CREATE INDEX IF NOT EXISTS idx_reports_entity ON reports(entity_type, entity_id);
CREATE INDEX IF NOT EXISTS idx_reports_status ON reports(status);