// src/api/users/update.rs
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::api::error_types::{ApiError, ApiResult};
use crate::infra::db::AppState;

// ⬇️ adjust this import if your sessions module lives elsewhere
use crate::api::auth::auth_handler::session::require_user_id;

// validators & queries you’ll implement next
use crate::api::users::users_query::update::update_user;
use crate::api::users::users_validate::update::validate_update_user;

// Public shape you already return elsewhere
use crate::api::users::users_query::update::UserPublic;

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub username: String,
    pub dob: Option<String>, // ISO-8601 "YYYY-MM-DD" or null
    pub bio: Option<String>, // null/empty allowed
}

#[derive(Serialize)]
pub struct UpdateUserResponse {
    pub user: UserPublic,
}

/// PUT /api/users/:id
/// - Requires a valid session. Caller must be the same user as :id (or add admin check later).
/// - Expects all 4 fields (username, email, dob, bio) in the body.
/// - Validates, updates, and returns the updated public user.
pub async fn update_user_handler(
    State(state): State<AppState>,
    Path(path_id): Path<i64>,
    headers: HeaderMap,
    Json(body): Json<UpdateUserRequest>,
) -> ApiResult<(StatusCode, Json<UpdateUserResponse>)> {
    // 1) Auth: must be the same user (extend to allow admins if desired)
    let caller_id = require_user_id(&headers, &state.db).await?;
    if caller_id != path_id {
        return Err(ApiError::Forbidden);
    }

    // 2) Validate & normalize input (you'll implement the validator next)
    let input = validate_update_user(body)?; // -> ValidationError maps to ApiError::Validation

    // 3) Persist (maps unique constraint errors to Conflict in your query layer)
    let updated = update_user(&state.db, path_id, &input).await?;

    // 4) Respond
    Ok((StatusCode::OK, Json(UpdateUserResponse { user: updated })))
}
