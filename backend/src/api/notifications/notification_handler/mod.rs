use crate::infra::db::AppState;
use axum::{
    routing::{get, put},
    Router,
};

pub mod list;
pub mod mark;
pub mod types;

pub fn router() -> Router<AppState> {
    Router::new()
        // GET /api/notifications?page=&limit=
        .route("/api/notifications", get(list::list_notifications))
        // GET /api/notifications/unread
        .route("/api/notifications/unread", get(list::unread_count))
        // PUT /api/notifications/read  { "ids": [1,2,3] }
        .route("/api/notifications/read", put(mark::mark_read))
        // PUT /api/notifications/read_all
        .route("/api/notifications/read_all", put(mark::mark_all_read))
}
