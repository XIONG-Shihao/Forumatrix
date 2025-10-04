use super::types::{Page, PageParams};
use crate::api::error_types::validation::ValidationError;

/// Validate and normalize page/limit with sane defaults.
/// - page: 1-based, default 1
/// - limit: clamped to [1, max_limit], default 20 or max_limit if smaller
pub fn validate_paging(p: &PageParams, max_limit: u32) -> Result<Page, ValidationError> {
    let page_u32 = p.page.unwrap_or(1);
    if page_u32 == 0 {
        return Err(ValidationError::InvalidPage);
    }

    let limit_u32 = p.limit.unwrap_or(20).max(1).min(max_limit);
    if limit_u32 == 0 {
        return Err(ValidationError::InvalidLimit);
    }

    let limit = i64::from(limit_u32);
    let offset = i64::from((page_u32 - 1) * limit_u32);
    Ok(Page { limit, offset })
}
