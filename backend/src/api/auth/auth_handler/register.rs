use axum::{extract::State, http::StatusCode, Json};

use super::password::hash_password;
use super::types::{RegisterRequest, RegisterResponse};
use crate::api::auth::auth_query::users as user_q;
use crate::api::auth::auth_validate::register::validate_register;
use crate::api::avatars::avatars_handler::generate::generate_and_store_avatar;
use crate::api::avatars::avatars_query::update::update_avatar_url;
use crate::api::error_types::{ApiError, ApiResult};
use crate::infra::db::AppState;

#[axum::debug_handler]
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> ApiResult<(StatusCode, Json<RegisterResponse>)> {
    let v = validate_register(payload)?; // ValidationError → ApiError::Validation via From<>

    let password_hash = hash_password(&v.password).map_err(|_| ApiError::Internal {
        message: "failed to hash password".into(),
    })?;

    // Default mapping (db.rs) turns unique violations into 409, others into 500.

    // If you want a friendlier conflict message, specialize here:
    let new_id = user_q::insert_user(
        &state.db,
        &v.email,
        &password_hash,
        &v.username,
        &v.dob,
        &v.bio,
    )
    .await
    .map_err(|e| {
        let api: ApiError = e.into();
        match api {
            ApiError::Conflict { .. } => ApiError::Conflict {
                message: "email or username already exists".into(),
            },
            other => other,
        }
    })?;

    // Generate and store avatar
    let avatar_url =
        generate_and_store_avatar(new_id, &v.username).map_err(|e| ApiError::Internal {
            message: format!("failed to generate avatar: {e}"),
        })?;

    // Update user with avatar_url
    let _ = update_avatar_url(&state.db, new_id, &avatar_url)
        .await
        .map_err(|e| ApiError::Internal {
            message: format!("failed to update avatar_url: {e}"),
        })?;

    Ok((
        StatusCode::CREATED,
        Json(RegisterResponse {
            id: new_id,
            email: v.email,
            username: v.username,
        }),
    ))
}
