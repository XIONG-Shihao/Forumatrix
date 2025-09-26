use super::cookie_helper::{clear_sid_cookie_headers, extract_sid};
use crate::api::auth::auth_query::sessions as sess_q;
use crate::infra::db::AppState;
use axum::{extract::State, http::HeaderMap, http::StatusCode};

/// POST /api/auth/logout_all — delete *all* sessions for the user owning this sid.
#[axum::debug_handler]
pub async fn logout_all(
    State(state): State<AppState>,
    headers_in: HeaderMap,
) -> Result<(StatusCode, HeaderMap), (StatusCode, String)> {
    if let Some(sid) = extract_sid(&headers_in) {
        let _ = sess_q::delete_all_sessions_for_sid_owner(&state.db, &sid)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("db error: {e}")))?;
    }

    let headers_out = clear_sid_cookie_headers();
    Ok((StatusCode::NO_CONTENT, headers_out))
}
