pub mod doc_handler;
pub mod doc_query;
pub mod doc_validate;

use crate::infra::db::AppState;

pub fn router() -> axum::Router<AppState> {
    // collect all API sub-routers here
    axum::Router::new().merge(doc_handler::router())
}
