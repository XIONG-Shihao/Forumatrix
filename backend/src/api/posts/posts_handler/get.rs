use super::types::PostPublic;
use crate::{
    api::auth::auth_handler::session::require_user_id, // 👈
    api::error_types::core::ApiResult,
    api::posts::posts_query::get as post_q,
    infra::db::AppState,
};
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    Json,
};

#[axum::debug_handler]
pub async fn get_post(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i64>,
) -> ApiResult<Json<PostPublic>> {
    // 🔐 must be logged in
    let viewer_id = require_user_id(&headers, &state.db).await?;

    let r = post_q::fetch_post_by_id(&state.db, id, viewer_id)
        .await?
        .ok_or(crate::api::error_types::core::ApiError::NotFound)?;

    Ok(Json(PostPublic {
        id: r.id,
        user_id: r.user_id,
        title: r.title,
        body: r.body,
        created_at: r.created_at,
        updated_at: r.updated_at,
        edited: r.edited as i32,
        score: r.score,
        comment_count: r.comment_count,
        author_username: r.author_username,
        author_avatar_url: r.author_avatar_url,
        liked_by_me: r.liked_by_me != 0,
    }))
}
