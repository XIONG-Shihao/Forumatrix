use crate::infra::db::AppState;
use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub mod create;
pub mod delete;
pub mod get;
pub mod like;
pub mod list;
pub mod my_list;
pub mod types;
pub mod update;

pub fn router() -> Router<AppState> {
    Router::new()
        // post-scoped comments
        .route(
            "/api/posts/:post_id/comments",
            get(list::list_comments_for_post).post(create::create_comment),
        )
        // comment-by-id operations
        .route(
            "/api/comments/:id",
            get(get::get_comment).put(update::update_comment),
        )
        .route(
            "/api/comments/:id/delete",
            post(delete::delete_comment_handler),
        )
        // comment likes
        .route(
            "/api/comments/:id/like",
            put(like::like_comment).delete(like::unlike_comment),
        )
        .route("/api/users/:id/comments", get(my_list::list_user_comments))
}
