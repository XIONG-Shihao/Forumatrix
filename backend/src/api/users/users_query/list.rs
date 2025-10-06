use super::types::UserRow;
use crate::infra::db::Db;

/// List users with pagination.
pub async fn list_users(db: &Db, limit: i64, offset: i64) -> Result<Vec<UserRow>, sqlx::Error> {
    sqlx::query_as::<_, UserRow>(
        r#"
        SELECT id, email, username, dob, bio, is_active, is_admin
        FROM users
        ORDER BY id
        LIMIT ? OFFSET ?
        "#,
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(db)
    .await
}
