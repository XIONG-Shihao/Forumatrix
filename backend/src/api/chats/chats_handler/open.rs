use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};
use base64::{engine::general_purpose::STANDARD, Engine as _};

use crate::api::auth::auth_handler::session::require_user_id;
use crate::api::chats::chats_query::create_chat::get_or_create_chat;
use crate::api::chats::chats_validate::open::validate_open_chat;
use crate::api::error_types::{ApiError, ApiResult};
use crate::infra::db::AppState;

use super::types::{OpenChatRequest, OpenChatResponse};
use crate::api::auth::auth_handler::utils::load_kek;

/// POST /api/chats/open  { peer_id }
pub async fn open_chat(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<OpenChatRequest>,
) -> ApiResult<(StatusCode, Json<OpenChatResponse>)> {
    let caller_id = require_user_id(&headers, &state.db).await?;
    let input = validate_open_chat(caller_id, body.peer_id)?;

    let kek = load_kek()?;
    let chat = get_or_create_chat(&state.db, input.caller_id, input.peer_id, &kek).await?;

    Ok((
        StatusCode::OK,
        Json(OpenChatResponse {
            chat_id: chat.id,
            peer_id: input.peer_id,
        }),
    ))
}
