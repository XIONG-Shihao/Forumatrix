// backend/src/api/docs/doc_handler/pages.rs
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::api::auth::auth_handler::session::require_user_id;
use crate::api::auth::auth_handler::utils::now_unix;
use crate::api::docs::doc_handler::types::PageOpenPayload;
use crate::api::docs::doc_query::{members, pages};
use crate::api::docs::doc_validate::pages::{
    validate_editor, validate_page_index, validate_style, validate_update_bytes,
};
use crate::api::error_types::{ApiError, ValidationError};
use crate::infra::db::AppState;

const MAX_UPDATE_BYTES: usize = 512 * 1024; // 512KB

#[derive(Debug, Serialize)]
pub struct OpenPageResponse(pub PageOpenPayload);

#[derive(Debug, Deserialize)]
pub struct PagePath {
    pub doc_id: i64,
    pub page_index: i64,
}

pub async fn open_page_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(PagePath { doc_id, page_index }): Path<PagePath>,
) -> Result<Json<OpenPageResponse>, ApiError> {
    let user_id = require_user_id(&headers, &state.db).await?;

    validate_editor(&state.db, doc_id, user_id).await?;
    validate_page_index(page_index)?;

    let payload = pages::open_page_payload(&state.db, doc_id, page_index)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ApiError::Validation(ValidationError::DocNotFound),
            other => ApiError::from(other),
        })?;

    Ok(Json(OpenPageResponse(payload)))
}

#[derive(Debug, Deserialize)]
pub struct UpsertPageRequest {
    /// 1=Title, 2=Heading, 3=Body
    pub style: Option<i64>,
    /// base64 of merged yrs update
    pub y_update_base64: String,
}

#[derive(Debug, Serialize)]
pub struct UpsertPageResponse {
    pub updated: u64,
    pub updated_at: i64,
}

pub async fn upsert_page_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(PagePath { doc_id, page_index }): Path<PagePath>,
    Json(body): Json<UpsertPageRequest>,
) -> Result<Json<UpsertPageResponse>, ApiError> {
    let user_id = require_user_id(&headers, &state.db).await?;
    // Must be editor (owner or collaborator).
    validate_editor(&state.db, doc_id, user_id).await?;
    validate_page_index(page_index)?;
    let style = body.style.unwrap_or(3);
    // 1=Title, 2=Heading, 3=Body
    validate_style(style)?;
    let bytes = base64::decode(&body.y_update_base64)
        .map_err(|_| ApiError::Validation(ValidationError::UpdateEmpty))?;
    validate_update_bytes(&bytes)?;

    let now = now_unix();
    let updated =
        pages::upsert_page_update(&state.db, doc_id, page_index, style, &bytes, now).await?;

    Ok(Json(UpsertPageResponse {
        updated,
        updated_at: now,
    }))
}
