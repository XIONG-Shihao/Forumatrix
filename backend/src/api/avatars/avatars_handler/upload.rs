use axum::{
    extract::{Multipart, Path, State},
    http::StatusCode,
    Json,
};
use image::ImageFormat;
use serde::Serialize;
use std::path::PathBuf;

use crate::api::avatars::avatars_query::update::update_avatar_url;
use crate::api::avatars::avatars_validate::upload::extract_validate_and_normalize;
use crate::api::error_types::{ApiError, ApiResult};
use crate::infra::db::AppState;

#[derive(Serialize)]
pub struct UploadAvatarResponse {
    id: i64,
    avatar_url: String,
    updated: bool,
}

/// POST /users/:id/avatar   (multipart/form-data, field name = "file")
pub async fn upload_avatar(
    State(state): State<AppState>,
    Path(user_id): Path<i64>,
    multipart: Multipart,
) -> ApiResult<(StatusCode, Json<UploadAvatarResponse>)> {
    // Centralized validation + normalization
    let valid = extract_validate_and_normalize(multipart).await?;

    // Overwrite at a predictable path (always .png)
    let ext = valid.target_ext; // "png"
    let fs_path = PathBuf::from(format!("/app/uploads/avatars/{user_id}.{ext}"));
    valid
        .img
        .save_with_format(&fs_path, ImageFormat::Png)
        .map_err(|e| ApiError::Internal {
            message: format!("failed to save avatar: {e}"),
        })?;

    // Update DB with canonical static URL
    let avatar_url = format!("/static/avatars/{user_id}.{ext}");
    let updated = update_avatar_url(&state.db, user_id, &avatar_url)
        .await
        .map_err(|e| ApiError::Internal {
            message: format!("failed to update avatar_url: {e}"),
        })?;

    Ok((
        StatusCode::OK,
        Json(UploadAvatarResponse {
            id: user_id,
            avatar_url,
            updated: updated > 0,
        }),
    ))
}
