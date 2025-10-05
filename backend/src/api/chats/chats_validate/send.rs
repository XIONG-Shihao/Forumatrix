use super::types::SendMessageInput;
use crate::api::error_types::validation::ValidationError;

/// Validate outgoing message body:
/// - trim
/// - not empty
/// - <= 200 Unicode scalar values (chars)
pub fn validate_send_message(raw: &str) -> Result<SendMessageInput, ValidationError> {
    let body = raw.trim();
    if body.is_empty() {
        return Err(ValidationError::MessageEmpty);
    }
    // Count chars, not bytes
    if body.chars().count() > 200 {
        return Err(ValidationError::MessageTooLong);
    }
    Ok(SendMessageInput {
        body: body.to_string(),
    })
}
