use super::types::ChatRow;
use crate::infra::db::Db;

/// Fetch a chat by id if the given user is a member; else RowNotFound.
pub async fn fetch_chat_for_member(
    db: &Db,
    chat_id: i64,
    member_id: i64,
) -> Result<Option<ChatRow>, sqlx::Error> {
    sqlx::query_as::<_, ChatRow>(
        r#"
        SELECT id, user_lo, user_hi, enc_key, key_nonce, created_at, last_msg_at
        FROM chats
        WHERE id = ?
          AND (user_lo = ? OR user_hi = ?)
        "#,
    )
    .bind(chat_id)
    .bind(member_id)
    .bind(member_id)
    .fetch_optional(db)
    .await
}
