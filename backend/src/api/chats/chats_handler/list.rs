use axum::{
    extract::{Query, State},
    http::HeaderMap,
    Json,
};
use serde::Deserialize;

use crate::api::auth::auth_handler::session::require_user_id;
use crate::api::chats::chats_query::list_chat::list_chats_for_user;
use crate::api::chats::chats_validate::paging::validate_paging;
use crate::api::chats::chats_validate::types::PageParams;
use crate::api::error_types::ApiResult;
use crate::infra::db::AppState;

use super::types::ListChatsResponse;

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

/// GET /api/chats?page&limit
pub async fn list_chats(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<ListQuery>,
) -> ApiResult<Json<ListChatsResponse>> {
    let user_id = require_user_id(&headers, &state.db).await?;

    let page_norm = validate_paging(
        &PageParams {
            page: q.page,
            limit: q.limit,
        },
        50,
    )?;

    let items = list_chats_for_user(&state.db, user_id, page_norm.limit, page_norm.offset).await?;
    let has_more = (items.len() as i64) >= page_norm.limit;

    Ok(Json(ListChatsResponse {
        items,
        page: q.page.unwrap_or(1),
        has_more,
    }))
}
