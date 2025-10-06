use crate::infra::db::AppState;
use axum::{routing::get, routing::post, routing::put, Router};

pub mod get;
pub mod list;
pub mod suspend;
pub mod types;
pub mod update;

// 👇 Import the external module instead of declaring it as a child.
// If your folder is named `user_avatars`, change this line accordingly.

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/users", get(list::list_users))
        .route("/api/users/:id", get(get::get_user))
        .route("/api/users/:id", put(update::update_user_handler))
        .route(
            "/api/users/:user_id/suspend",
            put(suspend::suspend_user_handler), // <-- PUT is what the script calls
        )
}
