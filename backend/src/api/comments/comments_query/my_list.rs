use crate::infra::db::Db;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct MyCommentRow {
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
    pub post_title: String,
}

pub async fn list_comments_by_user(
    db: &Db,
    author_id: i64,
    page: i64,
    limit: i64,
) -> Result<(Vec<MyCommentRow>, i64), sqlx::Error> {
    let page = page.max(1);
    let limit = limit.clamp(1, 100);
    let offset = (page - 1) * limit;

    let sql = r#"
        SELECT
            c.id, c.post_id, c.user_id, c.parent_id, c.body,
            c.created_at, c.updated_at, c.deleted_at, c.edited, c.score,
            p.title AS post_title
        FROM comments c
        JOIN posts p ON p.id = c.post_id
        WHERE c.user_id = ?1
        ORDER BY c.created_at DESC, c.id DESC
        LIMIT ?2 OFFSET ?3
    "#;

    let rows = sqlx::query_as::<_, MyCommentRow>(sql)
        .bind(author_id) // ?1
        .bind(limit) // ?2
        .bind(offset) // ?3
        .fetch_all(db)
        .await?;

    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM comments WHERE user_id = ?1")
        .bind(author_id)
        .fetch_one(db)
        .await?;

    Ok((rows, total))
}
