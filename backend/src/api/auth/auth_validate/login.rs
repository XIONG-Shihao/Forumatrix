// use crate::domain::auth::types::LoginRequest;
use crate::api::auth::auth_handler::types::LoginRequest;
use crate::api::error_types::ValidationError;
/// Sanitized login payload.
pub struct ValidLogin {
    /// Lower-cased identifier (email or username).
    pub identifier: String,
    pub password: String,
}

pub fn validate_login(input: LoginRequest) -> Result<ValidLogin, ValidationError> {
    let identifier = input.identifier.trim().to_lowercase();
    let password = input.password;

    if identifier.is_empty() || password.is_empty() {
        return Err(ValidationError::MissingIdentifierOrPassword);
    }

    Ok(ValidLogin {
        identifier,
        password,
    })
}
