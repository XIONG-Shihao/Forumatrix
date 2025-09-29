use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct CommentRow {
    pub id: i64,
    pub post_id: i64,
    pub user_id: i64,
    pub parent_id: Option<i64>,
    pub body: String,
    pub created_at: i64,
    pub updated_at: Option<i64>,
    pub deleted_at: Option<i64>,
    pub edited: i32,
    pub score: i64,
    pub author_username: String,
    pub author_avatar_url: Option<String>,
    pub parent_author_username: Option<String>,
    pub liked_by_me: i32, // NEW
    pub reply_count: i64, // NEW
    pub author_is_admin: bool,
}
