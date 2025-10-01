pub mod notification_handler;
pub mod notifications_query;
pub mod notifications_validate;

use crate::infra::db::AppState;
use axum::Router;

pub fn router() -> Router<AppState> {
    // collect all API sub-routers here
    Router::new().merge(notification_handler::router())
}
