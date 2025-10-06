pub mod users_handler;
pub mod users_query;
pub mod users_validate;
use crate::infra::db::AppState;
use axum::Router;

pub fn router() -> Router<AppState> {
    // collect all API sub-routers here
    Router::new().merge(users_handler::router())
}
