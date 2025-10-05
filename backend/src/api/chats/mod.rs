pub mod chats_handler;
pub mod chats_query;
pub mod chats_validate;
use crate::infra::db::AppState;
use axum::Router;

pub fn router() -> Router<AppState> {
    // collect all API sub-routers here
    Router::new().merge(chats_handler::router())
    // .merge(chats_query::router()) --- IGNORE ---
    // .merge(chats_validate::router()) --- IGNORE ---
}
