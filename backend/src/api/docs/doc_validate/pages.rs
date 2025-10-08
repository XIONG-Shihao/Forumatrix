// backend/src/api/docs/doc_validate/pages.rs
use crate::api::docs::doc_query::members;
use crate::api::error_types::{ApiError, ValidationError};
use crate::infra::db::Db;

const MAX_UPDATE_BYTES: usize = 512 * 1024; // 512KB

/// Must be editor (owner or collaborator).
pub async fn validate_editor(db: &Db, doc_id: i64, user_id: i64) -> Result<(), ApiError> {
    if !members::is_editor(db, doc_id, user_id).await? {
        return Err(ApiError::Validation(ValidationError::NotDocEditor));
    }
    Ok(())
}

/// 0..=9 (A4 pages 10-max).
pub fn validate_page_index(page_index: i64) -> Result<(), ApiError> {
    if !(0..=9).contains(&page_index) {
        // Reuse UpdateTooLarge for out-of-range index per your existing enum set.
        return Err(ApiError::Validation(ValidationError::UpdateTooLarge));
    }
    Ok(())
}

/// 1=Title, 2=Heading, 3=Body
pub fn validate_style(style: i64) -> Result<(), ApiError> {
    if !matches!(style, 1 | 2 | 3) {
        return Err(ApiError::Validation(ValidationError::UpdateTooLarge));
    }
    Ok(())
}

pub fn validate_update_bytes(bytes: &[u8]) -> Result<(), ApiError> {
    if bytes.is_empty() {
        return Err(ApiError::Validation(ValidationError::UpdateEmpty));
    }
    if bytes.len() > MAX_UPDATE_BYTES {
        return Err(ApiError::Validation(ValidationError::UpdateTooLarge));
    }
    Ok(())
}
