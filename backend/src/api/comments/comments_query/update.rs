use crate::infra::db::Db;
use sqlx::Row;

/// Author check helper
pub async fn fetch_comment_owner(db: &Db, id: i64) -> Result<Option<i64>, sqlx::Error> {
    let row = sqlx::query("SELECT user_id FROM comments WHERE id = ?")
        .bind(id)
        .fetch_optional(db)
        .await?;
    Ok(row.map(|r| r.get::<i64, _>("user_id")))
}

/// Update body; bump updated_at and set edited=1.
/// Returns rows_affected.
pub async fn update_comment_body(
    db: &Db,
    id: i64,
    new_body: &str,
    now_unix: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"
        UPDATE comments
           SET body = ?, updated_at = ?, edited = 1
         WHERE id = ? AND deleted_at IS NULL
        "#,
    )
    .bind(new_body)
    .bind(now_unix)
    .bind(id)
    .execute(db)
    .await?;
    Ok(res.rows_affected())
}
