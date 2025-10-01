pub mod generate;
mod types;
pub mod update;
pub mod upload;

use crate::infra::db::AppState;
use axum::routing::put;
use axum::Router;

// Only API endpoints (under /api/…)
pub fn router() -> Router<AppState> {
    Router::new().route("/api/users/:id/avatar", put(upload::upload_avatar))
}
