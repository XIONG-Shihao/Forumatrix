use crate::api::error_types::ValidationError;
use axum::extract::Multipart;
use image::{imageops::FilterType, DynamicImage, GenericImageView, ImageFormat};

/// Hard caps (kept here so the handler stays dumb).
pub const MAX_BYTES: usize = 2 * 1024 * 1024; // 2 MiB
pub const TARGET_SIZE: u32 = 512; // 512×512 avatars

/// Output of validation + normalization.
/// We always produce a square 512×512 PNG so URLs/format are predictable.
pub struct ValidAvatar {
    pub img: DynamicImage,        // normalized, square image
    pub target_ext: &'static str, // always "png"
}

/// Extract the "file" part from `multipart`, validate it, decode, square-crop,
/// resize to 512, and return the normalized image (always PNG as output format).
///
/// Errors are expressed as `ValidationError` so your global `IntoResponse`
/// will map them to 400s with stable codes.
pub async fn extract_validate_and_normalize(
    mut multipart: Multipart,
) -> Result<ValidAvatar, ValidationError> {
    // 1) Extract "file" field bytes
    let mut file_bytes: Option<Vec<u8>> = None;
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| ValidationError::AvatarBadMultipart)?
    {
        if field.name() == Some("file") {
            let data = field
                .bytes()
                .await
                .map_err(|_| ValidationError::AvatarBadMultipart)?;
            file_bytes = Some(data.to_vec());
            break;
        }
    }
    let file_bytes = file_bytes.ok_or(ValidationError::AvatarMissingFile)?;

    // 2) Size cap
    if file_bytes.len() > MAX_BYTES {
        return Err(ValidationError::AvatarTooLarge);
    }

    // 3) Sniff content type (don’t trust filename)
    let format =
        image::guess_format(&file_bytes).map_err(|_| ValidationError::AvatarInvalidFormat)?;
    match format {
        ImageFormat::Png | ImageFormat::Jpeg => {}
        _ => return Err(ValidationError::AvatarInvalidFormat),
    }

    // 4) Decode
    let img =
        image::load_from_memory(&file_bytes).map_err(|_| ValidationError::AvatarDecodeFailed)?;

    // 5) Normalize to square (center crop) and resize to 512×512
    let (w, h) = img.dimensions();
    let side = w.min(h);
    let x0 = (w.saturating_sub(side)) / 2;
    let y0 = (h.saturating_sub(side)) / 2;
    let cropped = img.crop_imm(x0, y0, side, side);
    let normalized = cropped.resize_exact(TARGET_SIZE, TARGET_SIZE, FilterType::Lanczos3);

    // 6) Force consistent output format for storage/URL
    Ok(ValidAvatar {
        img: normalized,
        target_ext: "png",
    })
}
