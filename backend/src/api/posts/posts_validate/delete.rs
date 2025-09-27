use crate::api::error_types::validation::ValidationError;
use crate::api::error_types::ApiError;

/// Admin moderation must provide a short reason (<=200 chars).
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

/// Mode inferred from caller/target roles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeleteMode {
    /// Author deletes their own post (works for normal users and admins on their own posts)
    UserSelf,
    /// Admin moderates another user's post (must include reason)
    AdminModeration,
}

/// Decide whether this delete is allowed and which mode to use.
///
/// Matrix:
/// 1) normal user, own post -> UserSelf
/// 2) normal user, others' post -> error(DeleteNotOwner)
/// 3) normal user, admin’s post -> error(DeleteNotOwner)
/// 4) admin, normal user’s post -> AdminModeration
/// 5) admin, own post -> UserSelf (no reason required)
/// 6) admin A, admin B’s post -> error(DeleteTargetIsAdmin)
pub fn validate_delete_mode(
    caller_id: i64,
    caller_is_admin: bool,
    author_id: i64,
    author_is_admin: bool,
) -> Result<DeleteMode, ApiError> {
    if caller_is_admin {
        if author_is_admin && caller_id != author_id {
            // Admin cannot delete another admin's post
            return Err(ApiError::Validation(ValidationError::DeleteTargetIsAdmin));
        }
        // Admin deleting own post → treat as self delete
        if caller_id == author_id {
            return Ok(DeleteMode::UserSelf);
        }
        // Admin moderating a normal user's post
        return Ok(DeleteMode::AdminModeration);
    } else {
        // Normal user must be the author
        if caller_id != author_id {
            return Err(ApiError::Validation(ValidationError::DeleteNotOwner));
        }
        return Ok(DeleteMode::UserSelf);
    }
}
