use serde::Serialize;

/// What the client needs for the notification list
#[derive(Serialize)]
pub struct NotificationPublic {
    pub id: i64,
    /// "post_liked" | "comment_liked" | "post_replied" | "comment_replied"
    pub kind: String,
    pub created_at: i64,
    pub read_at: Option<i64>,

    pub actor_username: String,
    pub actor_avatar_url: Option<String>,

    // These are optional in the row (comment-like/reply joins may still yield NULL
    // depending on the join used), so expose them as Option to avoid panics.
    pub post_id: Option<i64>,
    pub post_title: Option<String>,

    pub comment_id: Option<i64>,
}

/// List envelope (keeps it consistent with posts/comments)
#[derive(Serialize)]
pub struct NotificationListResp {
    pub items: Vec<NotificationPublic>,
    pub page: i64,
    pub total_pages: i64,
    pub total: i64,
}
