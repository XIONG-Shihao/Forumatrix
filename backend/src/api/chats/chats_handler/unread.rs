use super::types::UnreadCountResponse;
use crate::api::auth::auth_handler::session::require_user_id;
use crate::api::chats::chats_query::unread::count_unread_for_user;
use crate::api::error_types::ApiResult;
use crate::infra::db::AppState;
use axum::{extract::State, http::HeaderMap, Json};

pub async fn unread_count(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> ApiResult<Json<UnreadCountResponse>> {
    let user_id = require_user_id(&headers, &state.db).await?;
    let unread = count_unread_for_user(&state.db, user_id).await?;
    Ok(Json(UnreadCountResponse { unread }))
}
