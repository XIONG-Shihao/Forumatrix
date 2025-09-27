use crate::api::error_types::ApiError;

/// Basic path-parameter check
pub fn validate_like_target(id: i64) -> Result<i64, ApiError> {
    if id <= 0 {
        Err(ApiError::NotFound)
    } else {
        Ok(id)
    }
}
