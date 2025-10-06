use super::types::UserPublic;
use crate::api::users::users_query::{list as user_q, types::UserRow};
use crate::infra::db::AppState;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[axum::debug_handler]
pub async fn list_users(
    State(state): State<AppState>,
    Query(p): Query<ListParams>,
) -> Result<Json<Vec<UserPublic>>, (StatusCode, String)> {
    let limit_u32: u32 = p.limit.unwrap_or(50).min(100);
    let offset_u32: u32 = p.offset.unwrap_or(0);

    let limit: i64 = i64::from(limit_u32);
    let offset: i64 = i64::from(offset_u32);

    let rows: Vec<UserRow> = user_q::list_users(&state.db, limit, offset)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("db error: {e}")))?;

    let users: Vec<UserPublic> = rows
        .into_iter()
        .map(|r| UserPublic {
            id: r.id,
            email: r.email,
            username: r.username,
            dob: r.dob,
            bio: r.bio,
            is_active: r.is_active,
            is_admin: r.is_admin,
        })
        .collect();

    Ok(Json(users))
}
