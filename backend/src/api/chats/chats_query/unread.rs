use crate::infra::db::Db;

pub async fn count_unread_for_user(db: &Db, user_id: i64) -> Result<i64, sqlx::Error> {
    let cnt: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)
        FROM chat_messages m
        JOIN chats c ON c.id = m.chat_id
        WHERE (c.user_lo = ? OR c.user_hi = ?)
          AND m.sender_id <> ?
          AND m.read_at IS NULL
        "#,
    )
    .bind(user_id)
    .bind(user_id)
    .bind(user_id)
    .fetch_one(db)
    .await?;
    Ok(cnt.0)
}
