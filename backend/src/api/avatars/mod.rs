pub mod avatars_handler;
pub mod avatars_query;
pub mod avatars_validate;

use crate::infra::db::AppState;
use axum::routing::get;
use axum::routing::post;
use axum::routing::put;
use axum::Router;

pub fn router() -> Router<AppState> {
    // collect all API sub-routers here
    Router::new().merge(avatars_handler::router())
}
