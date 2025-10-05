use crate::infra::db::AppState;
use axum::{
    routing::{get, post, put},
    Router,
};

pub mod create;
pub mod delete;
pub mod get;
pub mod like;
pub mod list;
pub mod list_liked;
pub mod my_list;
pub mod types;
pub mod update;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/posts", get(list::list_posts_handler))
        .route("/api/posts/:id", get(get::get_post))
        .route("/api/posts", post(create::create_post))
        .route("/api/posts/:id", put(update::update_post))
        .route(
            "/api/posts/:id/like",
            put(like::like_post).delete(like::unlike_post),
        )
        .route("/api/users/:id/posts", get(my_list::list_user_posts))
        .route(
            "/api/users/:id/liked_posts",
            get(list_liked::list_liked_posts_handler),
        )
        .route("/api/posts/:id/delete", post(delete::delete_post_handler))
}
