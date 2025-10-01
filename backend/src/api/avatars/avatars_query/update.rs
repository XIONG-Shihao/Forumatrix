use crate::infra::db::Db;
use sqlx::Result;

/// Update the avatar_url for a given user.

pub async fn update_avatar_url(
    db: &Db,
    user_id: i64,
    avatar_url: &str,
) -> Result<u64, sqlx::Error> {
    let row = sqlx::query(
        r#"
        UPDATE users
        SET avatar_url = ?
        WHERE id = ?
        "#,
    )
    .bind(avatar_url)
    .bind(user_id)
    .execute(db)
    .await?;

    Ok(row.rows_affected())
}
