// backend/src/api/posts_query/delete.rs
use crate::api::error_types::{ApiError, ApiResult};
use crate::infra::db::Db;

/// Marker texts that will replace the original body on soft-delete.
const USER_MARKER: &str = "[Deleted By User]";
const ADMIN_MARKER: &str = "[Deleted By Admin]";

/// Author (or admin deleting their *own* post) soft-deletes a post.
/// - Overwrites `body` with `[Deleted By User]`
/// - Sets deleted metadata (who/when)
/// - Only updates if not already deleted (`deleted_at IS NULL`)
///
/// Returns number of affected rows (0 if already deleted or post not found).
pub async fn soft_delete_by_author(
    db: &Db,
    post_id: i64,
    now: i64,
    deleter_user_id: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"
        UPDATE posts
        SET
          body               = ?,
          deleted_at         = ?,
          deleted_by         = 1,      -- 1 = user
          deleted_reason     = NULL,
          deleted_by_user_id = ?
        WHERE id = ?
          AND deleted_at IS NULL
        "#,
    )
    .bind(USER_MARKER)
    .bind(now)
    .bind(deleter_user_id)
    .bind(post_id)
    .execute(db)
    .await?;

    Ok(res.rows_affected())
}

/// Admin moderates another user's post.
/// - Overwrites `body` with `[Deleted By Admin]`
/// - Stores reason + who/when
/// - Only updates if not already deleted (`deleted_at IS NULL`)
///
/// Returns number of affected rows (0 if already deleted or post not found).
pub async fn soft_delete_by_admin(
    db: &Db,
    post_id: i64,
    now: i64,
    admin_user_id: i64,
    reason: &str,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"
        UPDATE posts
        SET
          body               = ?,
          deleted_at         = ?,
          deleted_by         = 2,      -- 2 = admin
          deleted_reason     = ?,
          deleted_by_user_id = ?
        WHERE id = ?
          AND deleted_at IS NULL
        "#,
    )
    .bind(ADMIN_MARKER)
    .bind(now)
    .bind(reason)
    .bind(admin_user_id)
    .bind(post_id)
    .execute(db)
    .await?;

    Ok(res.rows_affected())
}

pub async fn fetch_post_owner_and_admin(db: &Db, post_id: i64) -> Result<(i64, bool), ApiError> {
    // author_id + whether that author is an admin
    let row = sqlx::query_as::<_, (i64, i64)>(
        r#"
        SELECT p.user_id AS author_id,
               COALESCE(u.is_admin, 0) AS author_is_admin
        FROM posts p
        JOIN users u ON u.id = p.user_id
        WHERE p.id = ?
        "#,
    )
    .bind(post_id)
    .fetch_optional(db)
    .await
    .map_err(ApiError::from)?;

    match row {
        Some((author_id, author_is_admin_i64)) => Ok((author_id, author_is_admin_i64 != 0)),
        None => Err(ApiError::NotFound),
    }
}

pub async fn fetch_is_admin(db: &Db, user_id: i64) -> Result<bool, ApiError> {
    // assumes users.is_admin as INTEGER 0/1
    let opt: Option<i64> = sqlx::query_scalar(
        r#"
        SELECT is_admin FROM users WHERE id = ?
        "#,
    )
    .bind(user_id)
    .fetch_optional(db)
    .await
    .map_err(ApiError::from)?;

    Ok(opt.unwrap_or(0) != 0)
}

pub async fn fetch_deleted_at(db: &Db, post_id: i64) -> Result<Option<i64>, ApiError> {
    let opt: Option<i64> = sqlx::query_scalar(
        r#"
        SELECT deleted_at FROM posts WHERE id = ?
        "#,
    )
    .bind(post_id)
    .fetch_optional(db)
    .await
    .map_err(ApiError::from)?;

    Ok(opt)
}
