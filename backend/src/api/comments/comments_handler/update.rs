use crate::api::auth::auth_handler::session::require_user_id;
use crate::api::comments::comments_query::update as uq;
use crate::api::comments::comments_validate::update::{validate_update, UpdateCommentRequest};
use crate::api::error_types::{ApiError, ApiResult};
use crate::infra::db::AppState;
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    Json,
};

fn now_unix() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

#[axum::debug_handler]
pub async fn update_comment(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateCommentRequest>,
) -> ApiResult<(StatusCode, Json<serde_json::Value>)> {
    let caller_id = require_user_id(&headers, &state.db).await?;
    let v = validate_update(payload)?;

    match uq::fetch_comment_owner(&state.db, id).await? {
        None => return Err(ApiError::NotFound),
        Some(owner) if owner != caller_id => return Err(ApiError::Forbidden),
        Some(_) => {}
    }

    let rows = uq::update_comment_body(&state.db, id, &v.body, now_unix()).await?;
    Ok((
        StatusCode::OK,
        Json(serde_json::json!({ "id": id, "updated": rows > 0 })),
    ))
}
