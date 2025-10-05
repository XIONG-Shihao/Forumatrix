use super::types::ChatListItem;
use crate::infra::db::Db;

/// List chats for a user with peer info, last activity, and unread count.
pub async fn list_chats_for_user(
    db: &Db,
    user_id: i64,
    limit: i64,
    offset: i64,
) -> Result<Vec<ChatListItem>, sqlx::Error> {
    sqlx::query_as::<_, ChatListItem>(
        r#"
        SELECT
          c.id,
          CASE WHEN c.user_lo = ? THEN c.user_hi ELSE c.user_lo END AS peer_id,
          u.username AS peer_username,
          u.avatar_url AS peer_avatar_url,
          c.last_msg_at AS last_message_at,
          c.created_at,
          (
            SELECT COUNT(*)
            FROM chat_messages m
            WHERE m.chat_id = c.id
              AND m.sender_id <> ?
              AND m.read_at IS NULL
          ) AS unread_count
        FROM chats c
        JOIN users u
          ON u.id = CASE WHEN c.user_lo = ? THEN c.user_hi ELSE c.user_lo END
        WHERE c.user_lo = ? OR c.user_hi = ?
        ORDER BY c.last_msg_at DESC
        LIMIT ? OFFSET ?
        "#,
    )
    .bind(user_id) // CASE for peer_id
    .bind(user_id) // subquery unread sender != user
    .bind(user_id) // JOIN users CASE
    .bind(user_id) // WHERE user member
    .bind(user_id) // WHERE user member
    .bind(limit)
    .bind(offset)
    .fetch_all(db)
    .await
}
