-- Chats hold a unique, ordered pair of participants and an encrypted
-- per-chat symmetric key (wrapped with CHAT_MASTER_KEY).
CREATE TABLE IF NOT EXISTS chats (
  id            INTEGER PRIMARY KEY AUTOINCREMENT,
  user_lo       INTEGER NOT NULL,
  user_hi       INTEGER NOT NULL,
  enc_key       BLOB    NOT NULL, -- AEAD(cipher_key) using CHAT_MASTER_KEY
  key_nonce     BLOB    NOT NULL, -- AEAD nonce used to wrap cipher_key
  created_at    INTEGER NOT NULL,
  last_msg_at   INTEGER NOT NULL,

  -- Keep the pair canonical so we can enforce uniqueness easily.
  CHECK (user_lo < user_hi),
  UNIQUE(user_lo, user_hi)
);

-- Helpful indexes
CREATE INDEX IF NOT EXISTS idx_chats_last_msg_at ON chats(last_msg_at DESC);
CREATE INDEX IF NOT EXISTS idx_chats_user_lo ON chats(user_lo);
CREATE INDEX IF NOT EXISTS idx_chats_user_hi ON chats(user_hi);