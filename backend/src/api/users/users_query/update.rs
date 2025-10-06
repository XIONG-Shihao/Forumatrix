use crate::api::users::users_validate::update::UpdateUserInput;
use crate::infra::db::Db;
use sqlx::Row;

use serde::Serialize;

/// Public-safe user fields (no password hash).
#[derive(Debug, Clone, Serialize)]
pub struct UserPublic {
    pub id: i64,
    pub email: String,
    pub username: String,
    pub dob: Option<String>,
    pub bio: Option<String>,
    pub is_active: i32,
    pub is_admin: i32,
    pub avatar_url: Option<String>,
}

/// Update username/email/dob/bio and return the fresh public record.
///
/// Unique constraint violations (email/username) will bubble as `sqlx::Error`,
/// and your global error mapping should convert those to `ApiError::Conflict`.
pub async fn update_user(
    db: &Db,
    user_id: i64,
    input: &UpdateUserInput,
) -> Result<UserPublic, sqlx::Error> {
    // Normalize empty Option -> NULL in DB
    sqlx::query(
        r#"
        UPDATE users
        SET username = ?, dob = ?, bio = ?
        WHERE id = ?
        "#,
    )
    .bind(&input.username)
    .bind(&input.dob) // Option<String> maps to NULL when None
    .bind(&input.bio) // Option<String> maps to NULL when None
    .bind(user_id)
    .execute(db)
    .await?;

    // Return the updated public view
    let row = sqlx::query(
        r#"
        SELECT id, email, username, dob, bio, is_active, is_admin, avatar_url
        FROM users
        WHERE id = ?
        "#,
    )
    .bind(user_id)
    .fetch_one(db)
    .await?;

    // Build UserPublic (adjust field types if your struct differs)
    let user = UserPublic {
        id: row.get::<i64, _>("id"),
        email: row.get::<String, _>("email"),
        username: row.get::<String, _>("username"),
        dob: row.try_get::<String, _>("dob").ok(), // NULL -> None
        bio: row.try_get::<String, _>("bio").ok(), // NULL -> None
        is_active: {
            // SQLite booleans are often stored as INTEGER 0/1
            row.get("is_active")
        },
        is_admin: { row.get("is_admin") },
        avatar_url: row.try_get::<String, _>("avatar_url").ok(), // NULL -> None
    };

    Ok(user)
}
