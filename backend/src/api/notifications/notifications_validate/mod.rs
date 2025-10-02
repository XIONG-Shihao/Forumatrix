use crate::api::error_types::validation::ValidationError;

/// Returns (page, limit) with sane clamps.
pub fn validate_paging(
    page: Option<i64>,
    limit: Option<i64>,
) -> Result<(i64, i64), ValidationError> {
    let p = page.unwrap_or(1);
    let l = limit.unwrap_or(20);
    if p < 1 {
        return Err(ValidationError::InvalidPage);
    }
    if l < 1 || l > 100 {
        return Err(ValidationError::InvalidLimit);
    }
    Ok((p, l))
}

/// Shared helper for bodies like `{ "ids": [1,2,3] }` (mark-read).
/// Use a small max (e.g., 200) to prevent abuse.
pub fn validate_non_empty_ids_with_max(ids: &[i64], max: usize) -> Result<(), ValidationError> {
    if ids.is_empty() {
        return Err(ValidationError::EmptyIds);
    }
    if ids.len() > max {
        return Err(ValidationError::TooManyIds);
    }
    Ok(())
}
