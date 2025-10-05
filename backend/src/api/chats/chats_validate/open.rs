use super::types::OpenChatInput;
use crate::api::error_types::validation::ValidationError;

/// Validate opening (or getting) a 1–1 chat.
/// - `caller_id` comes from session
/// - `peer_id` must be positive and not equal to caller
/// NOTE: Existence/active status of `peer_id` is checked in the handler/query layer.
pub fn validate_open_chat(caller_id: i64, peer_id: i64) -> Result<OpenChatInput, ValidationError> {
    if peer_id <= 0 {
        return Err(ValidationError::MissingField); // reuse generic; you can add a specific one later
    }
    if peer_id == caller_id {
        return Err(ValidationError::ChatForbidden); // “not allowed in this chat”
    }
    Ok(OpenChatInput { caller_id, peer_id })
}
