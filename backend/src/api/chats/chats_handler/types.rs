use serde::{Deserialize, Serialize};

// Bring the server-facing list item for chats straight from the query module.
// (Your chat_query/mod.rs defines ChatSummary and marks it Serialize.)
use crate::api::chats::chats_query::types::ChatListItem;
use crate::api::chats::chats_query::types::MessageRow as MessageItem;
/* -------- Requests -------- */

#[derive(Debug, Deserialize)]
pub struct OpenChatRequest {
    pub peer_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct SendMessageRequest {
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct UnreadCountResponse {
    pub unread: i64,
}

#[derive(Debug, Serialize)]
pub struct MarkReadResponse {
    pub updated: i64,
}

/* -------- Public message DTO -------- */
// If your query layer already returns this, great. If it returns a different shape
// (e.g., ciphertext fields), we’ll map into this in the handler before responding.
/* -------- Responses -------- */

#[derive(Debug, Serialize)]
pub struct OpenChatResponse {
    pub chat_id: i64,
    pub peer_id: i64,
}

#[derive(Debug, Serialize)]
pub struct ListChatsResponse {
    pub items: Vec<ChatListItem>,
    pub page: u32,
    pub has_more: bool,
}

#[derive(Debug, Serialize)]
pub struct ListMessagesResponse {
    pub items: Vec<MessageItem>, // <- now matches the query return type
    pub page: u32,
    pub has_more: bool,
}

#[derive(Debug, Serialize)]
pub struct SendMessageResponse {
    pub message: MessageItem, // <- ditto
}
