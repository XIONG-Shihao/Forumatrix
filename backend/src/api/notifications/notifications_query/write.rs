use crate::infra::db::Db;

/// Insert a "liked your post" notification.
/// Returns rows_affected (0 if ignored by the UNIQUE index to dedupe like-spam).
pub async fn insert_like_post(
    db: &Db,
    recipient_user_id: i64,
    actor_id: i64,
    post_id: i64,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"
        INSERT OR IGNORE INTO notifications
          (user_id, actor_id, post_id, comment_id, kind, created_at)
        VALUES (?, ?, ?, NULL, 1, ?)
        "#,
    )
    .bind(recipient_user_id)
    .bind(actor_id)
    .bind(post_id)
    .bind(now)
    .execute(db)
    .await?;
    Ok(res.rows_affected())
}

/// Insert a "liked your comment" notification.
/// Returns rows_affected (0 if ignored by UNIQUE like-dedupe).
pub async fn insert_like_comment(
    db: &Db,
    recipient_user_id: i64,
    actor_id: i64,
    post_id: i64,
    comment_id: i64,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"
        INSERT OR IGNORE INTO notifications
          (user_id, actor_id, post_id, comment_id, kind, created_at)
        VALUES (?, ?, ?, ?, 2, ?)
        "#,
    )
    .bind(recipient_user_id)
    .bind(actor_id)
    .bind(post_id)
    .bind(comment_id)
    .bind(now)
    .execute(db)
    .await?;
    Ok(res.rows_affected())
}

/// Insert a "replied to your post" notification.
/// Always inserts (no UNIQUE constraint here). Returns the new id.
pub async fn insert_reply_post(
    db: &Db,
    recipient_user_id: i64,
    actor_id: i64,
    post_id: i64,
    now: i64,
) -> Result<i64, sqlx::Error> {
    let res = sqlx::query(
        r#"
        INSERT INTO notifications
          (user_id, actor_id, post_id, comment_id, kind, created_at)
        VALUES (?, ?, ?, NULL, 3, ?)
        "#,
    )
    .bind(recipient_user_id)
    .bind(actor_id)
    .bind(post_id)
    .bind(now)
    .execute(db)
    .await?;
    Ok(res.last_insert_rowid())
}

/// Insert a "replied to your comment" notification.
/// Always inserts. Returns the new id.
pub async fn insert_reply_comment(
    db: &Db,
    recipient_user_id: i64,
    actor_id: i64,
    post_id: i64,
    parent_comment_id: i64,
    now: i64,
) -> Result<i64, sqlx::Error> {
    let res = sqlx::query(
        r#"
        INSERT INTO notifications
          (user_id, actor_id, post_id, comment_id, kind, created_at)
        VALUES (?, ?, ?, ?, 4, ?)
        "#,
    )
    .bind(recipient_user_id)
    .bind(actor_id)
    .bind(post_id)
    .bind(parent_comment_id)
    .bind(now)
    .execute(db)
    .await?;
    Ok(res.last_insert_rowid())
}

/// Mark specific notifications as read for a user. Returns rows_affected.
/// Builds an IN list safely with dynamic placeholders.
pub async fn mark_read(db: &Db, user_id: i64, ids: &[i64], now: i64) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }

    // Dynamic IN (?, ?, ...)
    let mut qs = String::new();
    for i in 0..ids.len() {
        if i > 0 {
            qs.push_str(",");
        }
        qs.push('?');
    }

    let sql = format!(
        "UPDATE notifications SET read_at = ? WHERE user_id = ? AND id IN ({}) AND read_at IS NULL",
        qs
    );

    let mut q = sqlx::query(&sql).bind(now).bind(user_id);
    for id in ids {
        q = q.bind(id);
    }

    let res = q.execute(db).await?;
    Ok(res.rows_affected())
}

/// Mark all notifications for a user as read. Returns rows_affected.
pub async fn mark_all_read(db: &Db, user_id: i64, now: i64) -> Result<u64, sqlx::Error> {
    let res =
        sqlx::query("UPDATE notifications SET read_at = ? WHERE user_id = ? AND read_at IS NULL")
            .bind(now)
            .bind(user_id)
            .execute(db)
            .await?;
    Ok(res.rows_affected())
}
