use crate::api::auth::auth_handler::session::require_user_id;
use crate::api::auth::auth_handler::utils::now_unix;
use crate::api::error_types::{ApiError, ApiResult};
use crate::api::posts::posts_query::update as post_q;
use crate::api::posts::posts_validate::update::{validate_update, UpdatePostRequest};
use crate::infra::db::AppState;

use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    Json,
};

#[axum::debug_handler]
pub async fn update_post(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i64>,
    Json(payload): Json<UpdatePostRequest>,
) -> ApiResult<(StatusCode, Json<serde_json::Value>)> {
    // 👈 return ApiResult
    // session (ApiError via ?)
    let caller_id = require_user_id(&headers, &state.db).await?;

    // validation (ValidationError -> ApiError via From)
    let v = validate_update(payload)?;

    // ownership / existence check (sqlx::Error -> ApiError via From)
    match post_q::fetch_post_owner(&state.db, id).await? {
        None => return Err(ApiError::NotFound),
        Some(owner) if owner != caller_id => return Err(ApiError::Forbidden),
        Some(_) => {}
    }

    // perform update
    let rows = post_q::update_post_fields(
        &state.db,
        id,
        v.title.as_deref(),
        v.body.as_deref(),
        now_unix(),
    )
    .await?;

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({ "id": id, "updated": rows > 0 })),
    ))
}
