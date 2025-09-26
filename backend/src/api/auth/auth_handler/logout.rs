use super::cookie_helper::{clear_sid_cookie_headers, extract_sid};
use crate::api::auth::auth_query::sessions as sess_q;
use crate::api::error_types::ApiResult;
use crate::infra::db::AppState;
use axum::{extract::State, http::HeaderMap, http::StatusCode};

#[axum::debug_handler]
pub async fn logout(
    State(state): State<AppState>,
    headers_in: HeaderMap,
) -> ApiResult<(StatusCode, HeaderMap)> {
    if let Some(sid) = extract_sid(&headers_in) {
        let _ = sess_q::delete_session_by_id(&state.db, &sid).await?;
    }
    Ok((StatusCode::NO_CONTENT, clear_sid_cookie_headers()))
}
