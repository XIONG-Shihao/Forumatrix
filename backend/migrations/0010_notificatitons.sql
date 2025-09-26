-- notifications.sql
PRAGMA foreign_keys=ON;

CREATE TABLE IF NOT EXISTS notifications (
  id          INTEGER PRIMARY KEY,
  user_id     INTEGER NOT NULL,         -- recipient
  actor_id    INTEGER NOT NULL,         -- who did the thing
  post_id     INTEGER NULL REFERENCES posts(id)    ON DELETE CASCADE,
  comment_id  INTEGER NULL REFERENCES comments(id) ON DELETE CASCADE,
  kind        INTEGER NOT NULL,         -- 1=post_like, 2=comment_like, 3=post_reply, 4=comment_reply
  created_at  INTEGER NOT NULL DEFAULT (unixepoch()),
  read_at     INTEGER NULL,
  CHECK (kind IN (1,2,3,4)),
  FOREIGN KEY (user_id)  REFERENCES users(id) ON DELETE CASCADE,
  FOREIGN KEY (actor_id) REFERENCES users(id) ON DELETE CASCADE
);

-- speed up queries
CREATE INDEX IF NOT EXISTS idx_notifications_user_created ON notifications(user_id, created_at DESC);

-- avoid spammy dup like-notifs from double clicks
CREATE UNIQUE INDEX IF NOT EXISTS uniq_notifs_like ON notifications(user_id, actor_id, post_id, comment_id, kind)
  WHERE kind IN (1,2) AND read_at IS NULL;