// backend/src/api/docs/doc_validate/join_requests.rs
use crate::api::docs::doc_query::helpers::fetch_doc_row;
use crate::api::docs::doc_query::members;
use crate::api::error_types::{ApiError, ValidationError};
use crate::infra::db::Db;

/// Ensure the doc exists and the caller is not already an editor.
/// Your product spec treats duplicate join requests as idempotent OK,
/// so this returns Ok(()) even if already a member (handler can short-circuit).
pub async fn validate_create_join_request(
    db: &Db,
    doc_id: i64,
    user_id: i64,
) -> Result<(), ApiError> {
    let exists = fetch_doc_row(db, doc_id)
        .await?
        .ok_or(ApiError::Validation(ValidationError::DocNotFound))?;
    let _ = exists; // only existence check

    // If already editor, handler may decide to return 200 with request_id=0
    if members::is_editor(db, doc_id, user_id).await? {
        return Ok(());
    }

    Ok(())
}
