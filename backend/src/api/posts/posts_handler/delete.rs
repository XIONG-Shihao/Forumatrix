// backend/src/api/posts/delete.rs
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    Json,
};

use crate::api::auth::auth_handler::session::require_user_id;
use crate::api::auth::auth_handler::utils::now_unix;
use crate::api::error_types::{ApiError, ApiResult};
use crate::infra::db::{AppState, Db};

use crate::api::posts::posts_query::delete::{soft_delete_by_admin, soft_delete_by_author};
use crate::api::posts::posts_validate::delete::{
    validate_admin_reason, validate_delete_mode, DeleteMode,
};

use crate::api::error_types::validation::ValidationError;
use crate::api::posts::posts_query::delete::{
    fetch_deleted_at, fetch_is_admin, fetch_post_owner_and_admin,
};

use super::types::{DeletePostRequest, DeletePostResponse};
/* ----------------- notifications (stub) ----------------- */

async fn notify_post_deleted_by_admin(
    _db: &Db,
    _post_id: i64,
    _author_id: i64,
    _admin_id: i64,
    _reason: &str,
    _when: i64,
) -> Result<(), ApiError> {
    // TODO: insert into notifications with payload containing post_id, reason, when, moderator
    Ok(())
}

/* ----------------- request/response ----------------- */

/* ----------------- handler ----------------- */

pub async fn delete_post_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(post_id): Path<i64>,
    Json(body): Json<DeletePostRequest>,
) -> ApiResult<Json<DeletePostResponse>> {
    let caller_id = require_user_id(&headers, &state.db).await?;

    // Who owns the post and is the author an admin?
    let (author_id, author_is_admin) = fetch_post_owner_and_admin(&state.db, post_id).await?;

    // Is caller an admin?
    let caller_is_admin = fetch_is_admin(&state.db, caller_id).await?;

    // Already deleted?
    if let Some(when) = fetch_deleted_at(&state.db, post_id).await? {
        if when > 0 {
            return Err(ApiError::Validation(ValidationError::PostAlreadyDeleted));
        }
    }

    // Decide mode (enforces all 6 cases including "admin cannot delete another admin’s post")
    let mode = validate_delete_mode(caller_id, caller_is_admin, author_id, author_is_admin)?;

    let now = now_unix();

    match mode {
        DeleteMode::UserSelf => {
            // Author (or admin deleting *their own* post)
            let updated = soft_delete_by_author(&state.db, post_id, now, caller_id).await?;
            if updated == 0 {
                // Either already deleted or not found; we already checked "already deleted",
                // so treat it as NotFound.
                return Err(ApiError::NotFound);
            }

            Ok(Json(DeletePostResponse {
                post_id,
                mode: "user_self",
                deleted_at: now,
            }))
        }
        DeleteMode::AdminModeration => {
            // Must provide reason
            let reason = body.reason.as_deref().unwrap_or_default();
            validate_admin_reason(reason)?;

            let updated = soft_delete_by_admin(&state.db, post_id, now, caller_id, reason).await?;
            if updated == 0 {
                // Either already deleted or not found; above guard already covered "already deleted".
                return Err(ApiError::NotFound);
            }

            // Notify author with the reason and moderator
            notify_post_deleted_by_admin(&state.db, post_id, author_id, caller_id, reason, now)
                .await?;

            Ok(Json(DeletePostResponse {
                post_id,
                mode: "admin_moderation",
                deleted_at: now,
            }))
        }
    }
}
