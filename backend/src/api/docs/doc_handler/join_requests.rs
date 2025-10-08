// backend/src/api/docs/doc_handler/join_requests.rs
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::api::auth::auth_handler::session::require_user_id;
use crate::api::auth::auth_handler::utils::now_unix;
use crate::api::docs::doc_query::join_requests::{self, ApproveJoinError};
use crate::api::docs::doc_validate::join_requests::validate_create_join_request;
use crate::api::error_types::{ApiError, ValidationError};
use crate::infra::db::AppState;

#[derive(Debug, Deserialize)]
pub struct JoinReqCreateBody {
    pub message: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct JoinReqCreateResp {
    pub request_id: i64,
}

pub async fn create_join_request_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(doc_id): Path<i64>,
    Json(body): Json<JoinReqCreateBody>,
) -> Result<Json<JoinReqCreateResp>, ApiError> {
    let user_id = require_user_id(&headers, &state.db).await?;

    validate_create_join_request(&state.db, doc_id, user_id).await?;

    let now = now_unix();
    let req_id = join_requests::upsert_join_request(
        &state.db,
        doc_id,
        user_id,
        body.message.as_deref(),
        now,
    )
    .await?;
    Ok(Json(JoinReqCreateResp { request_id: req_id }))
}

#[derive(Debug, Deserialize)]
pub struct JoinReqPath {
    pub req_id: i64,
}

#[derive(Debug, Serialize)]
pub struct JoinReqApproveResp {
    pub ok: bool,
}

pub async fn approve_join_request_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(JoinReqPath { req_id }): Path<JoinReqPath>,
) -> Result<Json<JoinReqApproveResp>, ApiError> {
    let owner_id = require_user_id(&headers, &state.db).await?;
    let now = now_unix();

    match join_requests::approve_join_request(&state.db, req_id, owner_id, now).await {
        Ok(()) => Ok(Json(JoinReqApproveResp { ok: true })),
        Err(ApproveJoinError::CapacityReached) => Err(ApiError::Validation(
            ValidationError::DocMembersLimitReached,
        )),
        Err(ApproveJoinError::NotOwner) => Err(ApiError::Validation(ValidationError::NotDocOwner)),
        Err(ApproveJoinError::NotPending) => {
            // Use your own variant if you have one
            Err(ApiError::Validation(ValidationError::JoinRequestNotPending))
        }
        Err(ApproveJoinError::Db(e)) => Err(ApiError::from(e)),
    }
}

#[derive(Debug, Serialize)]
pub struct JoinReqDenyResp {
    pub updated: u64,
}

pub async fn deny_join_request_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(JoinReqPath { req_id }): Path<JoinReqPath>,
) -> Result<Json<JoinReqDenyResp>, ApiError> {
    let owner_id = require_user_id(&headers, &state.db).await?;
    let now = now_unix();
    let updated = join_requests::deny_join_request(&state.db, req_id, owner_id, now).await?;
    Ok(Json(JoinReqDenyResp { updated }))
}
