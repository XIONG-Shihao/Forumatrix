use crate::{
    api::{
        auth::auth_handler::session::require_user_id,
        comments::comments_handler::types::CommentPublic, comments::comments_query::get as cq,
        error_types::ApiResult,
    },
    infra::db::AppState,
};
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    Json,
};

#[axum::debug_handler]
pub async fn get_comment(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i64>,
) -> ApiResult<Json<CommentPublic>> {
    let viewer = require_user_id(&headers, &state.db).await?;

    let r = cq::fetch_comment_by_id(&state.db, id, viewer)
        .await?
        .ok_or(crate::api::error_types::core::ApiError::NotFound)?;

    Ok(Json(CommentPublic {
        id: r.id,
        post_id: r.post_id,
        user_id: r.user_id,
        parent_id: r.parent_id,
        body: r.body,
        created_at: r.created_at,
        updated_at: r.updated_at,
        deleted_at: r.deleted_at,
        edited: r.edited,
        score: r.score,
        author_username: r.author_username,
        author_avatar_url: r.author_avatar_url,
        parent_author_username: r.parent_author_username,
        liked_by_me: r.liked_by_me != 0,
        reply_count: r.reply_count,
    }))
}
