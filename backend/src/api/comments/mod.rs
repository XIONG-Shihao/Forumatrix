pub mod comments_handler;
pub mod comments_query;
pub mod comments_validate;

use crate::infra::db::AppState;
use axum::Router;

pub fn router() -> Router<AppState> {
    // collect all API sub-routers here
    Router::new().merge(comments_handler::router())
}
