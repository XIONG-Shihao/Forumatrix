// backend/src/api/docs/doc_validate/create.rs
use crate::api::error_types::{ApiError, ValidationError};

const MAX_TITLE_CHARS: usize = 120;

/// Validate create-doc input (title + page_count).
pub fn validate_create_doc_input(title: &str, page_count: i64) -> Result<(), ApiError> {
    let t = title.trim();
    if t.is_empty() {
        return Err(ApiError::Validation(ValidationError::DocTitleEmpty));
    }
    if t.chars().count() > MAX_TITLE_CHARS {
        return Err(ApiError::Validation(ValidationError::DocTitleTooLong));
    }
    if !(1..=10).contains(&page_count) {
        // Reuse existing error bucket per your enum; or introduce a dedicated PageCountInvalid.
        return Err(ApiError::Validation(ValidationError::UpdateTooLarge));
    }
    Ok(())
}
