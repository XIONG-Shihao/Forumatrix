use crate::api::posts::posts_handler::types::PostPublic;
use sqlx::FromRow;

/// Row shape returned by post queries (includes author username)
#[derive(Debug, Clone, FromRow)]
pub struct PostRow {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub body: String,
    pub created_at: i64,
    pub updated_at: Option<i64>,
    pub edited: i32,
    pub score: i64,
    pub comment_count: i64,
    pub author_username: String,
    pub author_avatar_url: Option<String>,
    pub liked_by_me: i64, // filled in handler if user is logged in
}

impl From<PostRow> for PostPublic {
    fn from(r: PostRow) -> Self {
        PostPublic {
            id: r.id,
            user_id: r.user_id,
            title: r.title,
            body: r.body,
            created_at: r.created_at,
            updated_at: r.updated_at,
            edited: r.edited,
            score: r.score,
            comment_count: r.comment_count,
            author_username: r.author_username,
            author_avatar_url: r.author_avatar_url,
            liked_by_me: r.liked_by_me != 0,
        }
    }
}
