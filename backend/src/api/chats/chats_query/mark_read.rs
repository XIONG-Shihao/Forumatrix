use crate::infra::db::Db;

pub async fn mark_read_for_user(
    db: &Db,
    chat_id: i64,
    user_id: i64,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"
        UPDATE chat_messages
        SET read_at = ?
        WHERE chat_id = ?
          AND sender_id <> ?
          AND read_at IS NULL
        "#,
    )
    .bind(now)
    .bind(chat_id)
    .bind(user_id)
    .execute(db)
    .await?;
    Ok(res.rows_affected())
}
