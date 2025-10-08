// backend/src/api/docs/doc_handler/members.rs
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    Json,
};
use serde::Serialize;

use crate::api::auth::auth_handler::session::require_user_id;
use crate::api::docs::doc_query::members as qmembers;
use crate::api::docs::doc_validate::members::validate_owner;
use crate::api::docs::doc_validate::members::validate_remove_member;
use crate::api::error_types::{ApiError, ValidationError};
use crate::infra::db::AppState;

#[derive(Debug, Serialize)]
pub struct MemberItem {
    pub user_id: i64,
    pub role: i32, // 3=owner, 2=editor
    pub added_at: i64,
}

#[derive(Debug, Serialize)]
pub struct ListMembersResp {
    pub items: Vec<MemberItem>,
}

pub async fn list_members_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(doc_id): Path<i64>,
) -> Result<Json<ListMembersResp>, ApiError> {
    let user_id = require_user_id(&headers, &state.db).await?;

    // Only owners can list members
    validate_owner(&state.db, doc_id, user_id).await?;

    // Owner + created_at (map RowNotFound → DocNotFound)
    let (owner_id, created_at) = qmembers::fetch_owner_and_created(&state.db, doc_id)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ApiError::Validation(ValidationError::DocNotFound),
            other => ApiError::from(other),
        })?;

    // Editors (role=2)
    let editors = qmembers::list_editors_by_doc(&state.db, doc_id).await?;

    // Compose response
    let mut items = Vec::with_capacity(1 + editors.len());
    items.push(MemberItem {
        user_id: owner_id,
        role: 3,
        added_at: created_at,
    });
    for r in editors {
        items.push(MemberItem {
            user_id: r.user_id,
            role: r.role as i32, // 2
            added_at: r.added_at,
        });
    }

    Ok(Json(ListMembersResp { items }))
}

#[derive(Debug, serde::Deserialize)]
pub struct RemoveMemberPath {
    pub doc_id: i64,
    pub member_user_id: i64,
}

#[derive(Debug, Serialize)]
pub struct RemoveMemberResp {
    pub removed: u64,
}

pub async fn remove_member_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(RemoveMemberPath {
        doc_id,
        member_user_id,
    }): Path<RemoveMemberPath>,
) -> Result<Json<RemoveMemberResp>, ApiError> {
    let caller_id = require_user_id(&headers, &state.db).await?;

    let owner_id: i64 =
        validate_remove_member(&state.db, doc_id, caller_id, member_user_id).await?;

    let removed = qmembers::remove_editor(&state.db, doc_id, member_user_id).await?;
    Ok(Json(RemoveMemberResp { removed }))
}
