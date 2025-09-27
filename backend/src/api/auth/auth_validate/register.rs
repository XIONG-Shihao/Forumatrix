use crate::api::auth::auth_handler::types::RegisterRequest;
use crate::api::auth::auth_handler::utils::normalize_email;
// use crate::domain::auth::{types::RegisterRequest, utils::normalize_email};
use crate::api::error_types::ValidationError; // 👈 use the domain error

/// Sanitized + validated payload the handler can trust.
pub struct ValidRegister {
    pub email: String,
    pub username: String,
    pub password: String,
    pub dob: Option<String>,
    pub bio: Option<String>,
}

pub fn validate_register(input: RegisterRequest) -> Result<ValidRegister, ValidationError> {
    // normalize
    let email = normalize_email(&input.email);
    let username = input.username.trim().to_string();
    let password = input.password;

    // email checks
    if email.is_empty() || !email.contains('@') || email.len() > 254 {
        return Err(ValidationError::EmailInvalid);
    }

    // username checks
    if username.is_empty() || username.len() > 32 {
        return Err(ValidationError::UsernameInvalid);
    }

    // password checks
    if password.len() < 8 {
        return Err(ValidationError::PasswordTooShort);
    }

    Ok(ValidRegister {
        email,
        username,
        password,
        dob: input.dob,
        bio: input.bio,
    })
}
