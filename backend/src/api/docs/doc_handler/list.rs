// backend/src/api/docs/doc_handler/list.rs
use axum::{
    extract::{Query, State},
    http::HeaderMap,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::api::auth::auth_handler::session::require_user_id;
use crate::api::docs::doc_handler::types::DocumentRow;
use crate::api::docs::doc_query::list as qlist;
use crate::api::docs::doc_validate::list::normalize_pagination;
use crate::api::error_types::ApiError;
use crate::infra::db::AppState;

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct ListDocsResponse {
    pub items: Vec<DocumentRow>,
    pub page: i64,
    pub total_pages: i64,
    pub total: i64,
}

pub async fn list_docs_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<ListQuery>,
) -> Result<Json<ListDocsResponse>, ApiError> {
    // auth required
    let user_id = require_user_id(&headers, &state.db).await?;

    // normalize page/limit
    let (page, limit) = normalize_pagination(q.page, q.limit);

    // total + page
    let total = qlist::count_documents_for_user(&state.db, user_id).await?;
    let total_pages = if total == 0 {
        1
    } else {
        ((total + (limit - 1)) / limit).max(1)
    };

    let items = qlist::list_documents_for_user(&state.db, user_id, page, limit).await?;

    Ok(Json(ListDocsResponse {
        items,
        page,
        total_pages,
        total,
    }))
}
