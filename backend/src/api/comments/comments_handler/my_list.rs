use axum::{
    extract::{Path, Query, State},
    http::HeaderMap,
    Json,
};
use serde::Deserialize;

use crate::{
    api::comments::comments_query::my_list as q,
    api::{
        auth::auth_handler::session::require_user_id,
        error_types::{ApiError, ApiResult},
    },
    infra::db::AppState,
};

#[derive(Deserialize)]
pub struct ListMyParams {
    page: Option<i64>,
    limit: Option<i64>,
}

#[axum::debug_handler]
pub async fn list_user_comments(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(user_id): Path<i64>,
    Query(p): Query<ListMyParams>,
) -> ApiResult<Json<serde_json::Value>> {
    // anyone can view another user's comments
    let page = p.page.unwrap_or(1).max(1);
    let limit = p.limit.unwrap_or(5).clamp(1, 100);

    let (rows, total) = q::list_comments_by_user(&state.db, user_id, page, limit).await?;

    let items: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|r| {
            serde_json::json!({
                "id": r.id,
                "post_id": r.post_id,
                "body": r.body,
                "created_at": r.created_at,
                "updated_at": r.updated_at,
                "deleted_at": r.deleted_at,
                "edited": r.edited,
                "score": r.score,
                "post_title": r.post_title,
            })
        })
        .collect();

    let total_pages = ((total + limit - 1) / limit).max(1);
    Ok(Json(serde_json::json!({
        "items": items,
        "page": page,
        "total_pages": total_pages,
        "total": total
    })))
}
