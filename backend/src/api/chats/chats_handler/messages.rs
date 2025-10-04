use crate::api::auth::auth_handler::utils::load_kek;
use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use serde::Deserialize;

use crate::api::auth::auth_handler::session::require_user_id;
use crate::api::auth::auth_handler::utils::now_unix;
use crate::api::chats::chats_validate::paging::validate_paging;
use crate::api::chats::chats_validate::send::validate_send_message;
use crate::api::chats::chats_validate::types::PageParams;
use crate::infra::db::AppState;

use crate::api::chats::chats_query::create_message::insert_message;
use crate::api::chats::chats_query::get_chat::fetch_chat_for_member;
use crate::api::chats::chats_query::list_message::list_messages_for_member;
use crate::api::chats::chats_query::mark_read::mark_read_for_user; // 👈 NEW

use crate::api::error_types::{ApiError, ApiResult};

use super::types::{
    ListMessagesResponse, MarkReadResponse, SendMessageRequest, SendMessageResponse,
};

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

/// GET /api/chats/:id/messages?page&limit
pub async fn list_messages(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(chat_id): Path<i64>,
    Query(q): Query<ListQuery>,
) -> ApiResult<Json<ListMessagesResponse>> {
    let user_id = require_user_id(&headers, &state.db).await?;

    // Will 404 (RowNotFound) if chat doesn't exist or user isn't a member
    let chat_opt = fetch_chat_for_member(&state.db, chat_id, user_id).await?;
    if chat_opt.is_none() {
        return Err(ApiError::Forbidden);
    }

    let page_norm = validate_paging(
        &PageParams {
            page: q.page,
            limit: q.limit,
        },
        100,
    )?;

    let items = list_messages_for_member(
        &state.db,
        chat_id,
        user_id, // ✅ pass user id
        page_norm.limit,
        page_norm.offset,
    )
    .await?;

    let has_more = (items.len() as i64) >= page_norm.limit;

    Ok(Json(ListMessagesResponse {
        items,
        page: q.page.unwrap_or(1),
        has_more,
    }))
}

/// POST /api/chats/:id/messages  { text }
pub async fn send_message(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(chat_id): Path<i64>,
    Json(body): Json<SendMessageRequest>,
) -> ApiResult<(StatusCode, Json<SendMessageResponse>)> {
    let user_id = require_user_id(&headers, &state.db).await?;

    // Will 404 if not member / no such chat
    let chat_opt = fetch_chat_for_member(&state.db, chat_id, user_id).await?;
    if chat_opt.is_none() {
        return Err(ApiError::Forbidden);
    }

    let input = validate_send_message(&body.text)?;

    let kek = load_kek()?;

    let msg = insert_message(
        &state.db,
        chat_id,
        user_id,
        input.body.as_bytes(), // ✅ &[u8]
        &kek,                  // ✅ KEK
    )
    .await?;

    Ok((
        StatusCode::CREATED,
        Json(SendMessageResponse { message: msg }),
    ))
}

pub async fn mark_read(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(chat_id): Path<i64>,
) -> ApiResult<Json<MarkReadResponse>> {
    let user_id = require_user_id(&headers, &state.db).await?;
    // ensure membership (hide existence if not a member)
    let _chat = fetch_chat_for_member(&state.db, chat_id, user_id)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ApiError::Forbidden,
            other => ApiError::from(other),
        })?;

    let updated = mark_read_for_user(&state.db, chat_id, user_id, now_unix()).await?;
    Ok(Json(MarkReadResponse {
        updated: updated as i64,
    }))
}
