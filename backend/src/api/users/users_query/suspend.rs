use crate::api::error_types::ApiError;
use crate::infra::db::Db;
use sqlx::Row;

pub async fn fetch_is_admin(db: &Db, user_id: i64) -> Result<bool, ApiError> {
    let v: Option<i64> = sqlx::query_scalar(r#"SELECT is_admin FROM users WHERE id = ?"#)
        .bind(user_id)
        .fetch_optional(db)
        .await
        .map_err(ApiError::from)?;
    Ok(matches!(v, Some(1)))
}

pub async fn fetch_is_active(db: &Db, user_id: i64) -> Result<bool, ApiError> {
    let v: Option<i64> = sqlx::query_scalar(r#"SELECT is_active FROM users WHERE id = ?"#)
        .bind(user_id)
        .fetch_optional(db)
        .await
        .map_err(ApiError::from)?;
    Ok(matches!(v, Some(1)))
}

pub async fn suspend_user(db: &Db, user_id: i64) -> Result<u64, ApiError> {
    let res = sqlx::query(r#"UPDATE users SET is_active = 0 WHERE id = ? AND is_active = 1"#)
        .bind(user_id)
        .execute(db)
        .await
        .map_err(ApiError::from)?;
    Ok(res.rows_affected())
}
