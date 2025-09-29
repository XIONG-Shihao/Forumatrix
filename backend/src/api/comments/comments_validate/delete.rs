use crate::api::comments::comments_query::delete::{
    fetch_comment_deleted_at, fetch_comment_owner_and_admin,
};
use crate::api::error_types::{ApiError, ValidationError};
use crate::api::posts::posts_query::delete::fetch_is_admin;
use crate::infra::db::Db;

pub enum DeleteMode {
    UserSelf,    // caller deletes own comment
    AdminOnUser, // admin deletes a normal user's comment
}

pub async fn validate_comment_delete_mode(
    db: &Db,
    caller_id: i64,
    comment_id: i64,
) -> Result<DeleteMode, ApiError> {
    // Already deleted?
    if let Some(_when) = fetch_comment_deleted_at(db, comment_id).await? {
        return Err(ApiError::Validation(ValidationError::CommentAlreadyDeleted));
    }

    let (author_id, author_is_admin) = fetch_comment_owner_and_admin(db, comment_id).await?;

    if caller_id == author_id {
        return Ok(DeleteMode::UserSelf);
    }

    let caller_is_admin = fetch_is_admin(db, caller_id).await?;
    if caller_is_admin && !author_is_admin {
        return Ok(DeleteMode::AdminOnUser);
    }

    if author_is_admin {
        return Err(ApiError::Validation(
            ValidationError::CannotDeleteAdminComment,
        ));
    }

    Err(ApiError::Forbidden)
}

/// Same rules as posts: admin justification required, ≤ 200 chars.
pub fn validate_admin_reason(reason: &str) -> Result<(), ApiError> {
    let r = reason.trim();
    if r.is_empty() {
        return Err(ApiError::Validation(ValidationError::DeleteReasonEmpty));
    }
    if r.chars().count() > 200 {
        return Err(ApiError::Validation(ValidationError::DeleteReasonTooLong));
    }
    Ok(())
}
