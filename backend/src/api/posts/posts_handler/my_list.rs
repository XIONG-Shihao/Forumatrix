use super::types::PostPublic;
use crate::api::auth::auth_handler::session::require_user_id;
use crate::api::error_types::ApiResult;
use crate::api::posts::posts_query::my_list::list_posts_by_user;
use crate::infra::db::AppState;

use axum::{
    extract::{Path, Query, State},
    http::HeaderMap,
    Json,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ListMyParams {
    page: Option<i64>,
    limit: Option<i64>,
}

#[axum::debug_handler]
pub async fn list_user_posts(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(user_id): Path<i64>,
    Query(p): Query<ListMyParams>,
) -> ApiResult<Json<serde_json::Value>> {
    let caller = require_user_id(&headers, &state.db).await?;
    // anyone can view another user's posts
    let page = p.page.unwrap_or(1).max(1);
    let limit = p.limit.unwrap_or(5).clamp(1, 100);

    let (rows, total) = list_posts_by_user(&state.db, caller, user_id, page, limit).await?;

    let items: Vec<PostPublic> = rows
        .into_iter()
        .map(|r| PostPublic {
            id: r.id,
            user_id: r.user_id,
            title: r.title,
            body: r.body,
            created_at: r.created_at,
            updated_at: r.updated_at,
            edited: r.edited,
            score: r.score,
            comment_count: r.comment_count,
            author_username: r.author_username,
            author_avatar_url: r.author_avatar_url,
            liked_by_me: r.liked_by_me != 0,
        })
        .collect();

    let total_pages = ((total + limit - 1) / limit).max(1);

    Ok(Json(serde_json::json!({
        "items": items,
        "page": page,
        "total_pages": total_pages,
        "total": total
    })))
}
