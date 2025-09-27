use crate::api::error_types::ValidationError;
use crate::api::posts::posts_handler::types::CreatePostRequest;

/// Sanitized payload the handler can trust.
pub struct ValidCreatePost {
    pub title: String,
    pub body: String,
}

pub fn validate_create(input: CreatePostRequest) -> Result<ValidCreatePost, ValidationError> {
    let title = input.title.trim().to_string();
    let body = input.body.trim().to_string();

    if title.is_empty() {
        return Err(ValidationError::TitleRequired);
    }
    if body.is_empty() {
        return Err(ValidationError::BodyRequired);
    }

    Ok(ValidCreatePost { title, body })
}
