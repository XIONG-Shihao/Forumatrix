// backend/src/api/posts/like.rs
use super::types::LikeResponse;
use crate::api::auth::auth_handler::utils::now_unix;
use crate::api::notifications::notifications_query::create as ncreate;
use crate::api::posts::posts_query::get as post_get;
use crate::{
    api::{
        auth::auth_handler::session::require_user_id, error_types::ApiResult,
        posts::posts_query::like as like_q, posts::posts_validate::like::validate_like_target,
    },
    infra::db::AppState,
};
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    Json,
};

#[axum::debug_handler]
pub async fn like_post(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i64>,
) -> ApiResult<(StatusCode, Json<LikeResponse>)> {
    let user_id = require_user_id(&headers, &state.db).await?;
    let post_id = validate_like_target(id)?;

    if !like_q::post_exists(&state.db, post_id).await? {
        return Err(crate::api::error_types::ApiError::NotFound);
    }

    let mut tx = state.db.begin().await?;
    let inserted = like_q::insert_like(&mut tx, post_id, user_id, now_unix()).await?;
    if inserted > 0 {
        like_q::bump_score(&mut tx, post_id, 1).await?;
    }
    let score = like_q::fetch_score(&mut tx, post_id).await?;
    tx.commit().await?;

    // 🔔 notify post author (if not self) only when a new like was added
    if inserted > 0 {
        if let Some((author_id, _title)) =
            post_get::fetch_post_author_and_title(&state.db, post_id).await?
        {
            if author_id != user_id {
                let _ = ncreate::insert(
                    &state.db,
                    author_id,
                    user_id,
                    ncreate::NotificationKind::PostLiked,
                    Some(post_id),
                    None,
                    now_unix(),
                )
                .await;
            }
        }
    }

    Ok((StatusCode::OK, Json(LikeResponse { liked: true, score })))
}

#[axum::debug_handler]
pub async fn unlike_post(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i64>,
) -> ApiResult<(StatusCode, Json<LikeResponse>)> {
    let user_id = require_user_id(&headers, &state.db).await?;
    let post_id = validate_like_target(id)?;

    if !like_q::post_exists(&state.db, post_id).await? {
        return Err(crate::api::error_types::ApiError::NotFound);
    }

    let mut tx = state.db.begin().await?;
    let deleted = like_q::delete_like(&mut tx, post_id, user_id).await?;
    if deleted > 0 {
        like_q::bump_score(&mut tx, post_id, -1).await?;
    }
    let score = like_q::fetch_score(&mut tx, post_id).await?;
    tx.commit().await?;

    Ok((
        StatusCode::OK,
        Json(LikeResponse {
            liked: false,
            score,
        }),
    ))
}
