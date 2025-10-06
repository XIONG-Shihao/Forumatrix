use crate::api::error_types::{ApiError, ValidationError};
use crate::api::users::users_query::suspend::{fetch_is_active, fetch_is_admin};
use crate::infra::db::Db;

/// Centralized validation for "suspend user"
/// - Only admins can suspend
/// - No self-suspension
/// - Admins cannot suspend other admins
/// - Target must be currently active (not already suspended)
pub async fn validate_suspend(
    db: &Db,
    caller_id: i64,
    target_user_id: i64,
) -> Result<(), ApiError> {
    // Only admins may attempt this
    if !fetch_is_admin(db, caller_id).await? {
        return Err(ApiError::Forbidden);
    }

    // No self-suspension
    if caller_id == target_user_id {
        return Err(ApiError::Validation(ValidationError::CannotSuspendSelf));
    }

    // Cannot suspend another admin
    if fetch_is_admin(db, target_user_id).await? {
        return Err(ApiError::Validation(ValidationError::CannotSuspendAdmin));
    }

    // Must be currently active
    if !fetch_is_active(db, target_user_id).await? {
        return Err(ApiError::Validation(ValidationError::UserAlreadySuspended));
    }

    Ok(())
}
