use axum::{extract::State, Json};
use serde::Deserialize;

use crate::{
    api::{
        auth::auth_handler::session::require_user_id, auth::auth_handler::utils::now_unix,
        error_types::ApiResult, notifications::notifications_query as nq,
        notifications::notifications_validate::validate_non_empty_ids_with_max,
    },
    infra::db::AppState,
};

#[derive(Deserialize)]
pub struct MarkReadBody {
    pub ids: Vec<i64>,
}

#[axum::debug_handler]
pub async fn mark_read(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Json(body): Json<MarkReadBody>,
) -> ApiResult<Json<serde_json::Value>> {
    let user_id = require_user_id(&headers, &state.db).await?;
    validate_non_empty_ids_with_max(&body.ids, 200)?;

    let now = now_unix();
    let updated = nq::mark_read(&state.db, user_id, &body.ids, now).await?;
    Ok(Json(serde_json::json!({ "updated": updated })))
}

#[axum::debug_handler]
pub async fn mark_all_read(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> ApiResult<Json<serde_json::Value>> {
    let user_id = require_user_id(&headers, &state.db).await?;
    let now = now_unix();
    let updated = nq::mark_all_read(&state.db, user_id, now).await?;
    Ok(Json(serde_json::json!({ "updated": updated })))
}
