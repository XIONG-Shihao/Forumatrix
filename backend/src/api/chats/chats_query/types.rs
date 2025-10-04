use serde::Serialize;
use sqlx::FromRow;

/// Raw chat row (matches migrations: user_lo/user_hi + enc_key/key_nonce/last_msg_at)
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct ChatRow {
    pub id: i64,
    pub user_lo: i64,
    pub user_hi: i64,
    pub enc_key: Vec<u8>,
    pub key_nonce: Vec<u8>,
    pub created_at: i64,
    pub last_msg_at: i64,
}

/// Item for listing a user’s chats (peer + last activity)
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct ChatListItem {
    pub id: i64,
    pub peer_id: i64,
    pub peer_username: String,
    pub peer_avatar_url: Option<String>,
    pub last_message_at: Option<i64>, // we’ll expose last_msg_at here
    pub created_at: i64,
    pub unread_count: i64,
}

/// Raw message row (ciphertext + nonce)
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct MessageRow {
    pub id: i64,
    pub chat_id: i64,
    pub sender_id: i64,
    pub nonce: Vec<u8>,
    pub ciphertext: Vec<u8>,
    pub created_at: i64,
    pub read_at: Option<i64>,
}

/// Public message shape we send back to the API layer
#[derive(Debug, Clone, Serialize)]
pub struct MessageItem {
    pub id: i64,
    pub chat_id: i64,
    pub sender_id: i64,
    pub body: String,
    pub created_at: i64,
}
