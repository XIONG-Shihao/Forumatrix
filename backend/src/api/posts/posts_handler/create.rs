use super::types::{CreatePostRequest, CreatePostResponse};
use crate::api::auth::auth_handler::session::require_user_id;
use crate::api::error_types::ApiResult;
use crate::api::posts::posts_query::create as post_q;
use crate::api::posts::posts_validate::create::validate_create;
use crate::infra::db::AppState;

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};

#[axum::debug_handler]
pub async fn create_post(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<CreatePostRequest>,
) -> ApiResult<(StatusCode, Json<CreatePostResponse>)> {
    // 👈 return ApiResult
    // transport-free validation; ValidationError -> ApiError via From<>
    let v = validate_create(payload)?;

    // session; ApiError bubbles via ?
    let user_id = require_user_id(&headers, &state.db).await?;

    // DB insert; sqlx::Error -> ApiError via From<sqlx::Error>
    let id = post_q::insert_post(&state.db, user_id, &v.title, &v.body).await?;

    Ok((StatusCode::CREATED, Json(CreatePostResponse { id })))
}
