-- Encrypted message rows. We store only ciphertext + nonce.
CREATE TABLE IF NOT EXISTS chat_messages (
  id          INTEGER PRIMARY KEY AUTOINCREMENT,
  chat_id     INTEGER NOT NULL REFERENCES chats(id) ON DELETE CASCADE,
  sender_id   INTEGER NOT NULL,
  nonce       BLOB    NOT NULL, -- AEAD nonce used for this message
  ciphertext  BLOB    NOT NULL, -- AEAD(body) using the per-chat cipher key
  created_at  INTEGER NOT NULL,
  read_at     INTEGER NULL,     -- when the *recipient* read it (server timestamp)

  -- ciphertext is expected to be non-empty
  CHECK (length(ciphertext) > 0)
);

-- Query patterns
CREATE INDEX IF NOT EXISTS idx_msg_chat_id_id          ON chat_messages(chat_id, id);
CREATE INDEX IF NOT EXISTS idx_msg_chat_id_created_at  ON chat_messages(chat_id, created_at);
CREATE INDEX IF NOT EXISTS idx_msg_unread              ON chat_messages(chat_id, read_at);