use crate::api::error_types::validation::ValidationError;

/// Clean, validated payload we pass to the DB layer.
#[derive(Debug, Clone)]
pub struct UpdateUserInput {
    pub username: String,
    pub dob: Option<String>, // normalized "YYYY-MM-DD" or None
    pub bio: Option<String>, // trimmed or None
}

/// Validate and normalize incoming fields.
///
/// Errors you’ll need in `ValidationError` (add if missing):
/// - `UsernameInvalid`
/// - `EmailInvalid`
/// - `DobInvalid`
/// - `BioTooLong`
pub fn validate_update_user(
    req: crate::api::users::users_handler::update::UpdateUserRequest,
) -> Result<UpdateUserInput, ValidationError> {
    // ---- username ----
    let username = req.username.trim();
    if !is_valid_username(username) {
        return Err(ValidationError::UsernameInvalid);
    }

    // ---- dob (optional) ----
    // Accept empty -> None. If present, must be YYYY-MM-DD and plausible.
    let dob = req
        .dob
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string());
    if let Some(ref d) = dob {
        if !is_valid_yyyy_mm_dd(d) {
            return Err(ValidationError::UserDobInvalid);
        }
    }

    // ---- bio (optional, limit e.g. 500 chars) ----
    let bio_max_len: usize = 500;
    let bio = req
        .bio
        .as_deref()
        .map(str::trim)
        .map(|s| {
            if s.is_empty() {
                None
            } else {
                Some(s.to_string())
            }
        })
        .flatten();
    if let Some(ref b) = bio {
        if b.chars().count() > bio_max_len {
            return Err(ValidationError::UserBioTooLong);
        }
    }

    Ok(UpdateUserInput {
        username: username.to_string(),
        dob,
        bio,
    })
}

fn is_valid_username(s: &str) -> bool {
    let len = s.chars().count();
    if len < 3 || len > 32 {
        return false;
    }
    s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn is_valid_email(s: &str) -> bool {
    // Lightweight sanity check (you likely already use something similar in register)
    let bytes = s.as_bytes();
    if !bytes.contains(&b'@') {
        return false;
    }
    // must have one dot after the '@'
    let parts: Vec<&str> = s.split('@').collect();
    if parts.len() != 2 {
        return false;
    }
    let domain = parts[1];
    domain.contains('.')
}

fn is_valid_yyyy_mm_dd(s: &str) -> bool {
    // Basic shape check and plausible ranges; avoids adding chrono/regex deps here.
    if s.len() != 10 {
        return false;
    }
    let bytes = s.as_bytes();
    if bytes[4] != b'-' || bytes[7] != b'-' {
        return false;
    }
    let (y, m, d) = (&s[0..4], &s[5..7], &s[8..10]);
    if !(y.chars().all(|c| c.is_ascii_digit())
        && m.chars().all(|c| c.is_ascii_digit())
        && d.chars().all(|c| c.is_ascii_digit()))
    {
        return false;
    }
    let (mm, dd) = (m.parse::<u32>().unwrap_or(0), d.parse::<u32>().unwrap_or(0));
    (1..=12).contains(&mm) && (1..=31).contains(&dd)
}
