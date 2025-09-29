pub mod create;
pub mod delete;
pub mod update;

use crate::api::comments::comments_handler::types::CommentSort;
use crate::api::error_types::{validation::ValidationError, ApiResult};

use crate::api::error_types::ApiError;

pub fn validate_post_id(id: i64) -> Result<i64, ApiError> {
    if id <= 0 {
        return Err(ValidationError::PostNotFound.into());
    } else {
        Ok(id)
    }
}

pub fn validate_comment_id(id: i64) -> Result<i64, ApiError> {
    if id <= 0 {
        return Err(ValidationError::CommentNotFound.into());
    }
    Ok(id)
}

pub fn parse_sort(s: Option<&str>) -> CommentSort {
    match s {
        Some("score") => CommentSort::Score,
        _ => CommentSort::Created,
    }
}

pub fn validate_paging(page: Option<i64>, limit: Option<i64>) -> (i64, i64) {
    let p = page.unwrap_or(1).max(1);
    let l = limit.unwrap_or(100).clamp(1, 200);
    (p, l)
}

// nested module used by like handler
pub mod like {
    use crate::api::error_types::ApiResult;

    pub fn validate_like_target(id: i64) -> ApiResult<i64> {
        super::validate_comment_id(id)
    }
}
