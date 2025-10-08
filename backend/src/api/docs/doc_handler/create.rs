// backend/src/api/docs/doc_handler/create.rs
use axum::{extract::State, http::HeaderMap, Json};
use serde::{Deserialize, Serialize};

use crate::api::auth::auth_handler::session::require_user_id;
use crate::api::auth::auth_handler::utils::now_unix;
use crate::api::docs::doc_query::docs::create_doc;
use crate::api::docs::doc_query::helpers::create_initial_pages;
use crate::api::docs::doc_validate::create::validate_create_doc_input;
use crate::api::error_types::ApiError;
use crate::infra::db::AppState;

const MAX_TITLE_CHARS: usize = 120;

#[derive(Debug, Deserialize)]
pub struct CreateDocRequest {
    pub title: String,
    #[serde(default = "default_page_count")]
    pub page_count: i64, // 1..=10
}
fn default_page_count() -> i64 {
    1
}

#[derive(Debug, Serialize)]
pub struct CreateDocResponse {
    pub id: i64,
}

pub async fn create_doc_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<CreateDocRequest>,
) -> Result<Json<CreateDocResponse>, ApiError> {
    let owner_id = require_user_id(&headers, &state.db).await?;

    let title = body.title.trim();
    validate_create_doc_input(title, body.page_count)?;

    let now = now_unix();

    let doc_id = create_doc(&state.db, owner_id, title, body.page_count, now).await?;

    create_initial_pages(&state.db, doc_id, body.page_count, now).await?;

    Ok(Json(CreateDocResponse { id: doc_id }))
}
