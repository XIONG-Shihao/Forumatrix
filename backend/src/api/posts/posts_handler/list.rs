use super::types::PostPublic;
use crate::api::auth::auth_handler::session::require_user_id;
use crate::api::error_types::ApiResult;
use crate::api::posts::posts_query::list::{list_posts, FeedSort};
use crate::infra::db::AppState;
use axum::{
    extract::{Query, State},
    http::HeaderMap,
    Json,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ListParams {
    sort: Option<String>,
    page: Option<i64>,
    limit: Option<i64>,
}

#[axum::debug_handler]
pub async fn list_posts_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<ListParams>,
) -> ApiResult<Json<serde_json::Value>> {
    let sort = match q.sort.as_deref() {
        Some("popular") => FeedSort::Popular,
        Some("controversial") => FeedSort::Controversial,
        _ => FeedSort::Latest,
    };
    let page = q.page.unwrap_or(1).max(1);
    let limit = q.limit.unwrap_or(7).clamp(1, 100);
    let viewer = require_user_id(&headers, &state.db).await?;
    let (rows, total) = list_posts(&state.db, viewer, sort, page, limit).await?;
    tracing::info!("posts list: rows_len={} total={}", rows.len(), total);
    let items: Vec<PostPublic> = rows
        .into_iter()
        .map(|r| PostPublic {
            id: r.id,
            user_id: r.user_id,
            title: r.title,
            body: r.body,
            created_at: r.created_at,
            updated_at: r.updated_at,
            edited: r.edited,
            score: r.score,
            comment_count: r.comment_count,
            author_username: r.author_username,
            author_avatar_url: r.author_avatar_url,
            liked_by_me: r.liked_by_me != 0,
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
