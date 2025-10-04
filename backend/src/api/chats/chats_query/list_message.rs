use super::types::MessageRow;
use crate::infra::db::Db;
/// List encrypted messages for a chat the user belongs to, newest first.
pub async fn list_messages_for_member(
    db: &Db,
    chat_id: i64,
    _member_id: i64, // not used here, but kept for symmetry/consistency
    limit: i64,
    offset: i64,
) -> Result<Vec<MessageRow>, sqlx::Error> {
    sqlx::query_as::<_, MessageRow>(
        r#"
        SELECT id, chat_id, sender_id, nonce, ciphertext, created_at, read_at
        FROM chat_messages
        WHERE chat_id = ?
        ORDER BY id DESC
        LIMIT ? OFFSET ?
        "#,
    )
    .bind(chat_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(db)
    .await
}
