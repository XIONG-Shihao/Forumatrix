use super::types::NotificationRow;
use crate::infra::db::Db;

/// Total notifications for a user (read + unread)
pub async fn count_for_user(db: &Db, user_id: i64) -> Result<i64, sqlx::Error> {
    let n: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM notifications WHERE user_id = ?")
        .bind(user_id)
        .fetch_one(db)
        .await?;
    Ok(n)
}

/// Unread notifications for a user
pub async fn unread_count(db: &Db, user_id: i64) -> Result<i64, sqlx::Error> {
    let n: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM notifications WHERE user_id = ? AND read_at IS NULL",
    )
    .bind(user_id)
    .fetch_one(db)
    .await?;
    Ok(n)
}

/// List notifications for a user, newest first, with paging
pub async fn list_for_user(
    db: &Db,
    user_id: i64,
    page: i64,
    limit: i64,
) -> Result<Vec<NotificationRow>, sqlx::Error> {
    let page = page.max(1);
    let limit = limit.clamp(1, 100);
    let offset = (page - 1) * limit;

    let rows = sqlx::query_as::<_, NotificationRow>(
        r#"
        SELECT
          n.id, n.user_id, n.actor_id, n.post_id, n.comment_id, n.kind,
          n.created_at, n.read_at,
          au.username AS actor_username,
          au.avatar_url AS actor_avatar_url,
          p.title AS post_title
        FROM notifications n
        JOIN users au ON au.id = n.actor_id
        LEFT JOIN posts p ON p.id = n.post_id
        WHERE n.user_id = ?
        ORDER BY n.created_at DESC, n.id DESC
        LIMIT ? OFFSET ?
        "#,
    )
    .bind(user_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(db)
    .await?;

    Ok(rows)
}
