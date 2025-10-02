use sqlx::FromRow;

/// Mirrors the `kind` column in `notifications`
/// 1=post_like, 2=comment_like, 3=post_reply, 4=comment_reply
#[repr(i32)]
#[derive(Clone, Copy, Debug, sqlx::Type)]
#[sqlx(type_name = "INTEGER")]
pub enum NotificationKind {
    PostLiked = 1,
    CommentLiked = 2,
    PostReplied = 3,
    CommentReplied = 4,
}

#[derive(Debug, Clone, FromRow)]
pub struct NotificationRow {
    pub id: i64,
    pub user_id: i64,  // recipient
    pub actor_id: i64, // who did it
    pub post_id: Option<i64>,
    pub comment_id: Option<i64>,
    pub kind: i32, // see NotificationKind
    pub created_at: i64,
    pub read_at: Option<i64>,

    // joined fields (actor + post)
    pub actor_username: String,
    pub actor_avatar_url: Option<String>,
    pub post_title: Option<String>,
}
