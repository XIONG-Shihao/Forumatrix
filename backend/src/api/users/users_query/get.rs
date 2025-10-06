use super::types::UserRow;
use crate::infra::db::Db;

/// Fetch one user by id.
pub async fn fetch_user_by_id(db: &Db, id: i64) -> Result<Option<UserRow>, sqlx::Error> {
    sqlx::query_as::<_, UserRow>(
        r#"SELECT id, email, username, dob, bio, is_active, is_admin
           FROM users WHERE id = ?"#,
    )
    .bind(id)
    .fetch_optional(db)
    .await
}
