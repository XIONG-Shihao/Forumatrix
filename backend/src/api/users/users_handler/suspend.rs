use axum::{
    extract::{Path, State},
    http::HeaderMap,
    Json,
};

use crate::api::auth::auth_handler::session::require_user_id;
use crate::api::error_types::{ApiError, ValidationError};
use crate::infra::db::AppState;

use crate::api::auth::auth_query::sessions as sess_q;
use crate::api::users::users_query::suspend::suspend_user;
use crate::api::users::users_validate::suspend::validate_suspend;

#[derive(serde::Serialize)]
pub struct SuspendUserResponse {
    user_id: i64,
    is_active: i32, // 0 after suspension
}

pub async fn suspend_user_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(target_user_id): Path<i64>,
) -> Result<Json<SuspendUserResponse>, ApiError> {
    // Who is calling?
    let caller_id = require_user_id(&headers, &state.db).await?;

    // All rules live in the validator now
    validate_suspend(&state.db, caller_id, target_user_id).await?;

    // Perform the update
    let rows = suspend_user(&state.db, target_user_id).await?;
    if rows == 0 {
        // Target not found or already inactive (race)
        return Err(ApiError::Validation(ValidationError::UserAlreadySuspended));
    }
    // logout all sessions of the suspended user
    let _revoked = sess_q::delete_all_sessions_for_user(&state.db, target_user_id)
        .await
        .map_err(ApiError::from)?;

    Ok(Json(SuspendUserResponse {
        user_id: target_user_id,
        is_active: 0,
    }))
}
