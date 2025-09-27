pub mod posts_handler;
pub mod posts_query;
pub mod posts_validate;

use crate::infra::db::AppState;
use axum::Router;

pub fn router() -> Router<AppState> {
    // collect all API sub-routers here
    Router::new().merge(posts_handler::router())
}
