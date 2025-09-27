use crate::infra::db::AppState;

use crate::api::auth::auth_query::{sessions as sess_q, users as user_q};
use crate::api::auth::auth_validate::login::validate_login;
use crate::api::error_types::{ApiError, ApiResult};

use super::password::verify_password;
use super::session::make_session_id;
use super::types::{LoginRequest, LoginResponse};
use super::utils::now_unix;

use axum::{
    extract::State,
    http::{HeaderMap, HeaderValue, StatusCode},
    Json,
};

const SESSION_TTL_SECS: i64 = 60 * 60 * 24 * 7; // 7 days

#[axum::debug_handler]
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> ApiResult<(StatusCode, HeaderMap, Json<LoginResponse>)> {
    let v = validate_login(payload)?; // ValidationError → ApiError

    let user = user_q::find_user_by_identifier(&state.db, &v.identifier).await?;
    let Some(u) = user else {
        return Err(ApiError::Unauthorized);
    };
    if u.is_active == 0 {
        return Err(ApiError::Forbidden);
    }

    verify_password(&v.password, &u.password_hash).map_err(|_| ApiError::Unauthorized)?;

    let sid = make_session_id();
    let expires_at = now_unix() + SESSION_TTL_SECS;
    sess_q::insert_session(&state.db, &sid, u.id, expires_at).await?;

    let cookie = format!(
        "sid={sid}; Path=/; HttpOnly; SameSite=Lax; Max-Age={}",
        SESSION_TTL_SECS
    );
    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::SET_COOKIE,
        HeaderValue::from_str(&cookie).unwrap(),
    );

    Ok((
        StatusCode::OK,
        headers,
        Json(LoginResponse {
            user_id: u.id,
            username: u.username,
            email: u.email,
        }),
    ))
}
