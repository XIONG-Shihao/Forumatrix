use super::types::UserPublic;
use crate::api::users::users_query::{get as user_q, types::UserRow};
use crate::infra::db::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

#[axum::debug_handler]
pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<UserPublic>, (StatusCode, String)> {
    let row: Option<UserRow> = user_q::fetch_user_by_id(&state.db, id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("db error: {e}")))?;

    match row {
        Some(r) => Ok(Json(UserPublic {
            id: r.id,
            email: r.email,
            username: r.username,
            dob: r.dob,
            bio: r.bio,
            is_active: r.is_active,
            is_admin: r.is_admin,
        })),
        None => Err((StatusCode::NOT_FOUND, "user not found".into())),
    }
}
