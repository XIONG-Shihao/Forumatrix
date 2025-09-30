/// Central API error type used by handlers. Transport-agnostic.
#[derive(Debug)]
pub enum ApiError {
    Validation(super::validation::ValidationError), // → 400
    Unauthorized,                                   // → 401
    Forbidden,                                      // → 403
    NotFound,                                       // → 404
    Conflict { message: String },                   // → 409
    Internal { message: String },                   // → 500
}

/// Convenience alias for handler return types.
pub type ApiResult<T> = Result<T, ApiError>;

// Allow `?` on validator results:
impl From<super::validation::ValidationError> for ApiError {
    fn from(e: super::validation::ValidationError) -> Self {
        ApiError::Validation(e)
    }
}
