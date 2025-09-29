use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::api::auth::auth_handler::session::require_user_id;
use crate::api::auth::auth_handler::utils::now_unix;
use crate::api::comments::comments_query::delete::{
    fetch_comment_owner_and_admin, // add
    fetch_comment_post_id,         // add
    soft_delete_comment_admin,
    soft_delete_comment_user,
};
use crate::api::comments::comments_validate::delete::{
    validate_admin_reason, validate_comment_delete_mode, DeleteMode,
};

use crate::api::error_types::{ApiError, ValidationError};
use crate::api::notifications::notifications_query::create::add_notification_comment_deleted;
use crate::infra::db::AppState;

#[derive(Debug, Deserialize)]
pub struct DeleteCommentRequest {
    pub reason: Option<String>, // required only when AdminOnUser
}

#[derive(Debug, Serialize)]
pub struct DeleteCommentResponse {
    pub comment_id: i64,
    pub deleted_at: i64,
    pub deleted_by: i32, // 1=user, 2=admin
    pub deleted_reason: Option<String>,
}

/// POST /api/comments/:id/delete
pub async fn delete_comment_handler(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(comment_id): Path<i64>,
    Json(body): Json<DeleteCommentRequest>,
) -> Result<Json<DeleteCommentResponse>, ApiError> {
    let caller_id = require_user_id(&headers, &state.db).await?;

    let mode = validate_comment_delete_mode(&state.db, caller_id, comment_id).await?;
    let (author_id, _author_is_admin) =
        fetch_comment_owner_and_admin(&state.db, comment_id).await?;
    let when = now_unix();

    let (deleted_by, reason_opt) = match mode {
        DeleteMode::UserSelf => {
            let rows = soft_delete_comment_user(&state.db, comment_id, when, caller_id).await?;
            if rows == 0 {
                return Err(ApiError::Validation(ValidationError::CommentAlreadyDeleted));
            }
            (1, None)
        }
        DeleteMode::AdminOnUser => {
            let reason = body.reason.unwrap_or_default();
            validate_admin_reason(&reason)?;
            let rows =
                soft_delete_comment_admin(&state.db, comment_id, when, caller_id, &reason).await?;
            if rows == 0 {
                return Err(ApiError::Validation(ValidationError::CommentAlreadyDeleted));
            }

            // 🔔 Notify the comment author (normal user)
            // (Only on admin-on-user path, not on self-delete)
            let post_id = fetch_comment_post_id(&state.db, comment_id).await?;
            // Best-effort: don’t fail the whole request if notification insert fails
            let _ = add_notification_comment_deleted(
                &state.db,
                author_id,
                post_id,
                comment_id,
                Some(&reason),
                when,
            )
            .await;

            (2, Some(reason))
        }
    };

    // TODO: send notification to the comment author if you support it

    Ok(Json(DeleteCommentResponse {
        comment_id,
        deleted_at: when,
        deleted_by,
        deleted_reason: reason_opt,
    }))
}
