use serde::{Deserialize, Serialize};

/// Public post shape returned by the API (mapped from posts_query::types::PostRow).
#[derive(Debug, Clone, Serialize)]
pub struct PostPublic {
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
    pub liked_by_me: bool, // filled in handler if user is logged in
}

#[derive(Debug, Deserialize)]
pub struct ListParams {
    /// Max number of posts to return (default 20, capped at 100)
    pub limit: Option<u32>,
    /// Offset for pagination (default 0)
    pub offset: Option<u32>,
    /// "new" | "top" (default "new")
    pub sort: Option<String>,
    /// Optional filter by author id
    pub user_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub body: String,
}

#[derive(Debug, Serialize)]
pub struct CreatePostResponse {
    pub id: i64,
}

#[derive(serde::Serialize)]
pub struct LikeResponse {
    pub liked: bool, // final state after the call
    pub score: i64,  // current posts.score
}

#[derive(Deserialize)]
pub struct DeletePostRequest {
    /// Required only for admin moderation of another user's post
    pub reason: Option<String>,
}

#[derive(serde::Serialize)]
pub struct DeletePostResponse {
    pub post_id: i64,
    pub mode: &'static str, // "user_self" | "admin_moderation"
    pub deleted_at: i64,
}
