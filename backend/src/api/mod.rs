use crate::infra::db::AppState;
use axum::Router;

pub mod auth;
pub mod avatars;
pub mod chats;
pub mod comments;
pub mod docs;
pub mod error_types;
pub mod notifications;
pub mod posts;
pub mod users;

pub fn router() -> Router<AppState> {
    // collect all API sub-routers here
    Router::new()
        .merge(auth::router())
        .merge(posts::router())
        .merge(users::router())
        .merge(comments::router())
        .merge(avatars::router())
        .merge(notifications::router())
        .merge(chats::router())
        .merge(docs::router())
}
