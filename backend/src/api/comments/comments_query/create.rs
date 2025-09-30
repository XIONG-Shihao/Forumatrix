use crate::infra::db::Db;

/// Inserts a comment and increments the post's `comment_count` in one transaction.
/// Returns the new comment id.
pub async fn insert_comment(
    db: &Db,
    post_id: i64,
    user_id: i64,
    parent_id: Option<i64>,
    body: &str,
) -> Result<i64, sqlx::Error> {
    let mut tx = db.begin().await?;

    let res = sqlx::query(
        r#"
        INSERT INTO comments (post_id, user_id, parent_id, body)
        VALUES (?, ?, ?, ?)
        "#,
    )
    .bind(post_id)
    .bind(user_id)
    .bind(parent_id)
    .bind(body)
    .execute(&mut *tx)
    .await?;

    sqlx::query("UPDATE posts SET comment_count = comment_count + 1 WHERE id = ?")
        .bind(post_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(res.last_insert_rowid())
}
