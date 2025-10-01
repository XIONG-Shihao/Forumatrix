use crate::api::avatars::avatars_query::update::update_avatar_url;
use crate::api::error_types::ApiResult;
use crate::infra::db::AppState;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateAvatarRequest {
    avatar_url: String,
}

pub async fn update_avatar_handler(
    State(state): State<AppState>,
    Path(user_id): Path<i64>,
    Json(payload): Json<UpdateAvatarRequest>,
) -> ApiResult<(StatusCode, Json<serde_json::Value>)> {
    let updated = update_avatar_url(&state.db, user_id, &payload.avatar_url).await?;

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "id": user_id,
            "avatar_url": payload.avatar_url,
            "updated": updated > 0
        })),
    ))
}
