use super::utils::now_unix;
use axum::http::HeaderMap;
use rand::RngCore;

use super::cookie_helper::extract_sid;
use crate::api::auth::auth_query::sessions as sess_q;
use crate::api::error_types::{ApiError, ApiResult};
use crate::infra::db::Db;

/// Look up the current user id by session cookie; ensure not expired.
/// Returns 401 if missing/invalid/expired, 500 on DB errors.
pub async fn require_user_id(headers: &HeaderMap, db: &Db) -> ApiResult<i64> {
    let sid = extract_sid(headers).ok_or(ApiError::Unauthorized)?;
    let now = now_unix();

    let uid_opt = sess_q::user_id_from_sid_if_valid(db, &sid, now).await?; // sqlx::Error -> ApiError via From
    match uid_opt {
        Some(uid) => Ok(uid),
        None => Err(ApiError::Unauthorized),
    }
}

pub fn make_session_id() -> String {
    let mut buf = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut buf);
    hex::encode(buf)
}
