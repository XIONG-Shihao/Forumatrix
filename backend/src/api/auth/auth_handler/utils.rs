use crate::api::error_types::{ApiError, ApiResult};
use base64::{engine::general_purpose::STANDARD, Engine as _};

pub fn normalize_email(email: &str) -> String {
    email.trim().to_lowercase()
}

pub fn now_unix() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

pub fn load_kek() -> ApiResult<Vec<u8>> {
    let raw = std::env::var("CHAT_MASTER_KEY").map_err(|_| ApiError::Internal {
        message: "CHAT_MASTER_KEY not set".into(),
    })?;
    let b64 = raw.strip_prefix("base64:").unwrap_or(&raw);
    let bytes = STANDARD.decode(b64).map_err(|_| ApiError::Internal {
        message: "CHAT_MASTER_KEY invalid base64".into(),
    })?;
    if bytes.len() < 16 {
        return Err(ApiError::Internal {
            message: "CHAT_MASTER_KEY too short".into(),
        });
    }
    Ok(bytes)
}
