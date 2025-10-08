// backend/src/api/docs/doc_handler/mod.rs
pub mod create;
pub mod join_requests;
pub mod list;
pub mod members;
pub mod meta;
pub mod pages;
pub mod types;

use crate::infra::db::AppState;
use axum::routing::{delete, get, post, put};
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/docs/:doc_id", get(meta::get_doc_meta_handler))
        .route(
            "/api/docs/:doc_id/pages/:page_index",
            get(pages::open_page_handler),
        )
        .route(
            "/api/docs/:doc_id/pages/:page_index",
            put(pages::upsert_page_handler),
        )
        .route(
            "/api/docs/:doc_id/join_requests",
            post(join_requests::create_join_request_handler),
        )
        .route(
            "/api/docs/requests/:req_id/approve",
            post(join_requests::approve_join_request_handler),
        )
        .route(
            "/api/docs/requests/:req_id/deny",
            post(join_requests::deny_join_request_handler),
        )
        .route(
            "/api/docs/:doc_id/members",
            get(members::list_members_handler),
        )
        .route(
            "/api/docs/:doc_id/members/:member_user_id",
            delete(members::remove_member_handler),
        )
        .route("/api/docs", post(create::create_doc_handler)) // create new doc
        .route("/api/docs", get(list::list_docs_handler)) // list my docs
}
