use crate::api::auth::auth_handler::utils::now_unix;
use crate::api::comments::comments_query::get as cget;
use crate::api::notifications::notifications_query::create as ncreate;
use crate::{
    api::{
        auth::auth_handler::session::require_user_id,
        comments::comments_handler::types::LikeResponse, comments::comments_query::like as like_q,
        comments::comments_validate::like::validate_like_target, error_types::ApiResult,
    },
    infra::db::AppState,
};

use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    Json,
};

#[axum::debug_handler]
pub async fn like_comment(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i64>,
) -> ApiResult<(StatusCode, Json<LikeResponse>)> {
    let user_id = require_user_id(&headers, &state.db).await?;
    let comment_id = validate_like_target(id)?;

    if !like_q::comment_exists(&state.db, comment_id).await? {
        return Err(crate::api::error_types::ApiError::NotFound);
    }

    let mut tx = state.db.begin().await?;
    let inserted = like_q::insert_like(&mut tx, comment_id, user_id, now_unix()).await?;
    if inserted > 0 {
        like_q::bump_score(&mut tx, comment_id, 1).await?;
    }
    let score = like_q::fetch_score(&mut tx, comment_id).await?;
    tx.commit().await?;

    // 🔔 notify comment author (if not self) only when a new like was added
    if inserted > 0 {
        if let Some((author_id, post_id)) =
            cget::fetch_comment_author_and_post(&state.db, comment_id).await?
        {
            if author_id != user_id {
                let _ = ncreate::insert(
                    &state.db,
                    author_id,
                    user_id,
                    ncreate::NotificationKind::CommentLiked,
                    Some(post_id),
                    Some(comment_id),
                    now_unix(),
                )
                .await;
            }
        }
    }
    Ok((StatusCode::OK, Json(LikeResponse { liked: true, score })))
}

#[axum::debug_handler]
pub async fn unlike_comment(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i64>,
) -> ApiResult<(StatusCode, Json<LikeResponse>)> {
    let user_id = require_user_id(&headers, &state.db).await?;
    let comment_id = validate_like_target(id)?;

    if !like_q::comment_exists(&state.db, comment_id).await? {
        return Err(crate::api::error_types::ApiError::NotFound);
    }

    let mut tx = state.db.begin().await?;
    let deleted = like_q::delete_like(&mut tx, comment_id, user_id).await?;
    if deleted > 0 {
        like_q::bump_score(&mut tx, comment_id, -1).await?;
    }
    let score = like_q::fetch_score(&mut tx, comment_id).await?;
    tx.commit().await?;

    Ok((
        StatusCode::OK,
        Json(LikeResponse {
            liked: false,
            score,
        }),
    ))
}
