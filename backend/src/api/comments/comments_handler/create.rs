use crate::api::{
    auth::auth_handler::{session::require_user_id, utils::now_unix},
    comments::comments_query::{create as cq, get as gq},
    error_types::{ApiResult, ValidationError},
    notifications::notifications_query::create as ncreate,
    posts::posts_query::get as post_get,
};

use crate::api::comments::comments_query::get as cget;

use crate::api::comments::comments_validate::create::{validate_create, CreateCommentRequest};

use crate::infra::db::AppState;
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    Json,
};

#[axum::debug_handler]
pub async fn create_comment(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(post_id): Path<i64>,
    Json(payload): Json<CreateCommentRequest>,
) -> ApiResult<(StatusCode, Json<serde_json::Value>)> {
    let user_id = require_user_id(&headers, &state.db).await?;
    let v = validate_create(payload)?;

    // If parent_id provided, validate existence and same post
    if let Some(parent_id) = v.parent_id {
        match gq::fetch_parent_post_id(&state.db, parent_id).await? {
            None => return Err(ValidationError::ParentNotFound.into()),
            Some(parent_post_id) if parent_post_id != post_id => {
                return Err(ValidationError::ParentNotInPost.into());
            }
            _ => {}
        }
    }

    let id = cq::insert_comment(&state.db, post_id, user_id, v.parent_id, &v.body).await?;

    // 🔔 notifications
    if let Some(parent_id) = v.parent_id {
        // reply to comment
        if let Some((parent_author_id, post_of_parent)) =
            cget::fetch_comment_author_and_post(&state.db, parent_id).await?
        {
            if parent_author_id != user_id {
                let _ = ncreate::insert(
                    &state.db,
                    parent_author_id,
                    user_id,
                    ncreate::NotificationKind::CommentReplied,
                    Some(post_of_parent),
                    Some(parent_id),
                    now_unix(),
                )
                .await;
            }
        }
    } else {
        // top-level comment → notify post author
        if let Some((post_author_id, _title)) =
            post_get::fetch_post_author_and_title(&state.db, post_id).await?
        {
            if post_author_id != user_id {
                let _ = ncreate::insert(
                    &state.db,
                    post_author_id,
                    user_id,
                    ncreate::NotificationKind::PostReplied,
                    Some(post_id),
                    Some(id), // include the new comment id for convenience
                    now_unix(),
                )
                .await;
            }
        }
    }
    Ok((StatusCode::CREATED, Json(serde_json::json!({ "id": id }))))
}
