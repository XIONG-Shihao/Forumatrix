use crate::api::error_types::ValidationError;

/// Sanitized create payload
pub struct ValidCreateComment {
    pub body: String,
    pub parent_id: Option<i64>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateCommentRequest {
    pub body: String,
    pub parent_id: Option<i64>,
}

pub fn validate_create(input: CreateCommentRequest) -> Result<ValidCreateComment, ValidationError> {
    let body = input.body.trim().to_string();
    if body.is_empty() {
        return Err(ValidationError::BodyRequired);
    }
    Ok(ValidCreateComment {
        body,
        parent_id: input.parent_id,
    })
}
