use super::types::PostRow;
use crate::infra::db::Db;

/// List posts liked by `user_id` (paged).
pub async fn list_liked_posts(
    db: &Db,
    user_id: i64,
    page: i64,
    limit: i64,
) -> Result<(Vec<PostRow>, i64), sqlx::Error> {
    let page = page.max(1);
    let limit = limit.clamp(1, 100);
    let offset = (page - 1) * limit;

    let sql = r#"
        SELECT
            p.id, p.user_id, p.title, p.body,
            p.created_at, p.updated_at, p.edited, p.score, p.comment_count,
            u.username AS author_username,
            u.avatar_url AS author_avatar_url,
            1 AS liked_by_me
        FROM post_likes pl
        JOIN posts p ON p.id = pl.post_id
        JOIN users u ON u.id = p.user_id
        WHERE pl.user_id = ?1
        ORDER BY pl.created_at DESC, p.id DESC
        LIMIT ?2 OFFSET ?3
    "#;

    let rows = sqlx::query_as::<_, PostRow>(sql)
        .bind(user_id) // ?1
        .bind(limit) // ?2
        .bind(offset) // ?3
        .fetch_all(db)
        .await?;

    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM post_likes WHERE user_id = ?")
        .bind(user_id)
        .fetch_one(db)
        .await?;

    Ok((rows, total))
}
