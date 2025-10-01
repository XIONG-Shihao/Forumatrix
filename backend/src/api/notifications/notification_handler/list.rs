use axum::{
    extract::{Query, State},
    Json,
};
use serde::Deserialize;

use super::types::{NotificationListResp, NotificationPublic};
use crate::{
    api::{
        auth::auth_handler::session::require_user_id, error_types::ApiResult,
        notifications::notifications_query as nq,
        notifications::notifications_validate::validate_paging,
    },
    infra::db::AppState,
};

#[derive(Deserialize)]
pub struct ListParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

fn kind_code_to_str(code: i32) -> &'static str {
    match code {
        1 => "post_liked",
        2 => "comment_liked",
        3 => "post_replied",
        4 => "comment_replied",
        5 => "comment_deleted",
        _ => "unknown",
    }
}

#[axum::debug_handler]
pub async fn list_notifications(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Query(q): Query<ListParams>,
) -> ApiResult<Json<NotificationListResp>> {
    let user_id = require_user_id(&headers, &state.db).await?;
    let (page, limit) = validate_paging(q.page, q.limit)?;

    let total = nq::count_for_user(&state.db, user_id).await?;
    let rows = nq::list_for_user(&state.db, user_id, page, limit).await?;
    let total_pages = ((total + limit - 1) / limit).max(1);

    let items = rows
        .into_iter()
        .map(|r| NotificationPublic {
            id: r.id,
            kind: kind_code_to_str(r.kind).to_string(),
            created_at: r.created_at,
            read_at: r.read_at,

            actor_username: r.actor_username,
            actor_avatar_url: r.actor_avatar_url,

            post_id: r.post_id,       // now Option<i64>
            post_title: r.post_title, // now Option<String>
            comment_id: r.comment_id, // Option<i64>
        })
        .collect();

    Ok(Json(NotificationListResp {
        items,
        page,
        total_pages,
        total,
    }))
}

/// Small endpoint the bell icon can poll
#[axum::debug_handler]
pub async fn unread_count(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> ApiResult<Json<serde_json::Value>> {
    let user_id = require_user_id(&headers, &state.db).await?;
    let n = nq::list::unread_count(&state.db, user_id).await?;
    Ok(Json(serde_json::json!({ "unread": n })))
}
