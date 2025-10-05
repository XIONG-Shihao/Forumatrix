use crate::api::error_types::ValidationError;
use serde::Deserialize;

/// Incoming update payload (title and/or body)
#[derive(Debug, Deserialize)]
pub struct UpdatePostRequest {
    pub title: Option<String>,
    pub body: Option<String>,
}

/// Sanitized update payload
pub struct ValidUpdatePost {
    pub title: Option<String>,
    pub body: Option<String>,
}

pub fn validate_update(input: UpdatePostRequest) -> Result<ValidUpdatePost, ValidationError> {
    if input.title.is_none() && input.body.is_none() {
        return Err(ValidationError::NothingToUpdate);
    }

    let provided_title = input.title.as_ref().is_some();
    let provided_body = input.body.as_ref().is_some();

    let title = input.title.and_then(|s| {
        let t = s.trim();
        if t.is_empty() {
            None
        } else {
            Some(t.to_string())
        }
    });
    let body = input.body.and_then(|s| {
        let t = s.trim();
        if t.is_empty() {
            None
        } else {
            Some(t.to_string())
        }
    });

    if provided_title && title.is_none() {
        return Err(ValidationError::TitleEmpty);
    }
    if provided_body && body.is_none() {
        return Err(ValidationError::BodyEmpty);
    }

    Ok(ValidUpdatePost { title, body })
}
