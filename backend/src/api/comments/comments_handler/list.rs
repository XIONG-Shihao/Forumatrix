use axum::{
    extract::{Path, Query, State},
    http::HeaderMap,
    Json,
};
use serde::Deserialize;

use crate::{
    api::{
        auth::auth_handler::session::require_user_id,
        comments::comments_handler::types::{CommentListResp, CommentPublic},
        comments::comments_query::list as cq,
        comments::comments_validate as validate,
        error_types::ApiResult,
    },
    infra::db::AppState,
};

#[derive(Deserialize)]
pub struct ListParams {
    pub sort: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[axum::debug_handler]
pub async fn list_comments_for_post(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(post_id): Path<i64>,
    Query(q): Query<ListParams>,
) -> ApiResult<Json<CommentListResp>> {
    let viewer = require_user_id(&headers, &state.db).await?;
    let post_id = validate::validate_post_id(post_id)?;
    let sort = validate::parse_sort(q.sort.as_deref());
    let (page, limit) = validate::validate_paging(q.page, q.limit);

    let total = cq::count_for_post(&state.db, post_id).await?;
    let rows = cq::list_for_post(&state.db, post_id, sort, page, limit, viewer).await?;
    let total_pages = ((total + limit - 1) / limit).max(1);

    let items = rows
        .into_iter()
        .map(|r| CommentPublic {
            id: r.id,
            post_id: r.post_id,
            user_id: r.user_id,
            parent_id: r.parent_id,
            body: r.body,
            created_at: r.created_at,
            updated_at: r.updated_at,
            deleted_at: r.deleted_at,
            edited: r.edited,
            score: r.score,
            author_username: r.author_username,
            author_avatar_url: r.author_avatar_url,
            parent_author_username: r.parent_author_username,
            liked_by_me: r.liked_by_me != 0,
            reply_count: r.reply_count,
        })
        .collect();

    Ok(Json(CommentListResp {
        items,
        page,
        total_pages,
        total,
    }))
}
