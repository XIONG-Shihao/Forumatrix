use super::types::UserCred;
use crate::infra::db::Db;

/// Find by email OR username (case-insensitive for username).
pub async fn find_user_by_identifier(
    db: &Db,
    ident_lower: &str,
) -> Result<Option<UserCred>, sqlx::Error> {
    if ident_lower.contains('@') {
        sqlx::query_as::<_, UserCred>(
            r#"SELECT id, email, username, password_hash, is_active
               FROM users WHERE email = ?"#,
        )
        .bind(ident_lower)
        .fetch_optional(db)
        .await
    } else {
        sqlx::query_as::<_, UserCred>(
            r#"SELECT id, email, username, password_hash, is_active
               FROM users WHERE lower(username) = ?"#,
        )
        .bind(ident_lower)
        .fetch_optional(db)
        .await
    }
}

/// Insert user on the pool and return new id.
pub async fn insert_user(
    db: &Db,
    email: &str,
    password_hash: &str,
    username: &str,
    dob: &Option<String>,
    bio: &Option<String>,
) -> Result<i64, sqlx::Error> {
    let res = sqlx::query(
        r#"
        INSERT INTO users (email, password_hash, username, dob, bio, is_active, is_admin)
        VALUES (?, ?, ?, ?, ?, 1, 0)
        "#,
    )
    .bind(email)
    .bind(password_hash)
    .bind(username)
    .bind(dob)
    .bind(bio)
    .execute(db)
    .await?;

    Ok(res.last_insert_rowid())
}
