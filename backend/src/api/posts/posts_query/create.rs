use crate::infra::db::Db;

/// Insert a post and return its id.
pub async fn insert_post(
    db: &Db,
    user_id: i64,
    title: &str,
    body: &str,
) -> Result<i64, sqlx::Error> {
    let res = sqlx::query(
        r#"
        INSERT INTO posts (user_id, title, body)
        VALUES (?, ?, ?)
        "#,
    )
    .bind(user_id)
    .bind(title)
    .bind(body)
    .execute(db)
    .await?;

    Ok(res.last_insert_rowid())
}
