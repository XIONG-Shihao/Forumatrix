use crate::infra::db::Db;

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum NotificationKind {
    PostLiked = 1,
    CommentLiked = 2,
    PostReplied = 3,
    CommentReplied = 4,
    CommentDeleted = 5, // custom kind for comment deletion by admin
}

pub async fn insert(
    db: &Db,
    recipient_user_id: i64,
    actor_user_id: i64,
    kind: NotificationKind,
    post_id: Option<i64>,
    comment_id: Option<i64>,
    now: i64,
) -> Result<i64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO notifications
            (user_id, actor_id, post_id, comment_id, kind, created_at, read_at)
         VALUES (?,       ?,        ?,       ?,          ?,    ?,          NULL)",
    )
    .bind(recipient_user_id)
    .bind(actor_user_id)
    .bind(post_id)
    .bind(comment_id)
    .bind(kind as i32)
    .bind(now)
    .execute(db)
    .await?;

    Ok(res.last_insert_rowid())
}

pub async fn notify_comment_deleted(
    db: &Db,
    recipient_user_id: i64, // the comment author
    admin_user_id: i64,     // the moderator (caller)
    post_id: i64,
    comment_id: i64,
    now: i64,
) -> Result<i64, sqlx::Error> {
    insert(
        db,
        recipient_user_id,
        admin_user_id,
        NotificationKind::CommentDeleted,
        Some(post_id),
        Some(comment_id),
        now,
    )
    .await
}
