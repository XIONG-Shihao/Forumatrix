pub mod auth_handler;
pub mod auth_query;
pub mod auth_validate;

use crate::infra::db::AppState;
use axum::Router;

pub fn router() -> Router<AppState> {
    // collect all API sub-routers here
    Router::new().merge(auth_handler::router())
    // .merge(auth_query::router()) --- IGNORE ---
    // .merge(auth_validate::router()) --- IGNORE ---
}
