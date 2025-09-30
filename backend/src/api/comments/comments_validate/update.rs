use crate::api::error_types::ValidationError;

#[derive(Debug, serde::Deserialize)]
pub struct UpdateCommentRequest {
    pub body: Option<String>,
}

pub struct ValidUpdateComment {
    pub body: String,
}

pub fn validate_update(input: UpdateCommentRequest) -> Result<ValidUpdateComment, ValidationError> {
    let Some(body_raw) = input.body else {
        return Err(ValidationError::NothingToUpdate);
    };
    let body = body_raw.trim().to_string();
    if body.is_empty() {
        return Err(ValidationError::BodyEmpty);
    }
    Ok(ValidUpdateComment { body })
}
