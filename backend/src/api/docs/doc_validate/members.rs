// backend/src/api/docs/doc_validate/members.rs
use crate::api::docs::doc_query::helpers::fetch_doc_row;
use crate::api::docs::doc_query::members;
use crate::api::error_types::{ApiError, ValidationError};
use crate::infra::db::Db;

/// Require owner rights.
pub async fn validate_owner(db: &Db, doc_id: i64, user_id: i64) -> Result<(), ApiError> {
    if !members::is_owner(db, doc_id, user_id).await? {
        return Err(ApiError::Validation(ValidationError::NotDocOwner));
    }
    Ok(())
}

/// Ensure caller is owner and not removing the owner.
pub async fn validate_remove_member(
    db: &Db,
    doc_id: i64,
    caller_id: i64,
    member_user_id: i64,
) -> Result<i64, ApiError> {
    // Caller must be owner
    validate_owner(db, doc_id, caller_id).await?;

    // Get owner id to prohibit removal of owner
    let some = fetch_doc_row(db, doc_id).await?;
    let (owner_id, _, _, _, _) = some.ok_or(ApiError::Validation(ValidationError::DocNotFound))?;
    if member_user_id == owner_id {
        return Err(ApiError::Validation(ValidationError::CannotRemoveOwner));
    }

    Ok(owner_id)
}
