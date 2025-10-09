// backend/src/api/docs/doc_handler/meta.rs
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    Json,
};

use crate::api::docs::doc_handler::types::DocumentMeta;
use crate::api::docs::doc_query::helpers::fetch_doc_row;
use crate::api::docs::doc_query::{members, pages};
use crate::api::error_types::{ApiError, ValidationError};
use crate::api::{
    auth::auth_handler::session::require_user_id, docs::doc_validate::pages::validate_editor,
};
use crate::infra::db::AppState;

#[derive(Debug, serde::Serialize)]
pub struct GetDocMetaResponse(pub DocumentMeta);

pub async fn get_doc_meta_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(doc_id): Path<i64>,
) -> Result<Json<GetDocMetaResponse>, ApiError> {
    let user_id = require_user_id(&headers, &state.db).await?;

    validate_editor(&state.db, doc_id, user_id).await?;

    let some = fetch_doc_row(&state.db, doc_id).await?;
    let (owner_id, title, page_count, created_at, updated_at) =
        some.ok_or(ApiError::Validation(ValidationError::DocNotFound))?;

    let pages_meta = pages::list_page_meta(&state.db, doc_id).await?;

    Ok(Json(GetDocMetaResponse(DocumentMeta {
        id: doc_id,
        owner_id,
        title,
        page_count,
        created_at,
        updated_at,
        pages: pages_meta,
    })))
}
