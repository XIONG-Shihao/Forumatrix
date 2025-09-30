use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug)]
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
    // NEW: 0/1 from SQL
    pub liked_by_me: i32,
    pub reply_count: i64,
}

#[derive(Serialize)]
pub struct CommentPublic {
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
    // NEW: bool for the client
    pub liked_by_me: bool,
    pub reply_count: i64,
}

#[derive(Serialize)]
pub struct CommentListResp {
    pub items: Vec<CommentPublic>,
    pub page: i64,
    pub total_pages: i64,
    pub total: i64,
}

#[derive(Clone, Copy, Debug)]
pub enum CommentSort {
    Created,
    Score,
}

// NEW: reuse the same shape as post likes
#[derive(Serialize)]
pub struct LikeResponse {
    pub liked: bool,
    pub score: i64,
}
