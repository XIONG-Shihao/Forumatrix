use crate::infra::db::AppState;
use axum::{
    routing::{get, post, put},
    Router,
};

pub mod list;
pub mod messages;
pub mod open;
pub mod types;
pub mod unread;

/// Mount all chat endpoints under /api/chats
pub fn router() -> Router<AppState> {
    Router::new()
        // POST /api/chats/open   → open (or get) a 1–1 chat with a peer
        .route("/api/chats/open", post(open::open_chat))
        // GET  /api/chats        → list chats for current user (paged)
        .route("/api/chats", get(list::list_chats))
        // GET  /api/chats/:id/messages  → list messages in a chat (paged)
        // POST /api/chats/:id/messages  → send a message to the chat
        .route(
            "/api/chats/:id/messages",
            get(messages::list_messages).post(messages::send_message),
        )
        .route("/api/chats/unread_count", get(unread::unread_count))
        .route("/api/chats/:id/read", put(messages::mark_read))
}
