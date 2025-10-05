use super::types::PostRow;
use crate::infra::db::Db;

pub async fn list_posts_by_user(
    db: &Db,
    viewer_id: i64,
    author_id: i64,
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
            CASE
              WHEN ?1 IS NULL THEN 0
              ELSE EXISTS(SELECT 1 FROM post_likes pl WHERE pl.post_id = p.id AND pl.user_id = ?1)
            END AS liked_by_me
        FROM posts p
        JOIN users u ON u.id = p.user_id
        WHERE p.user_id = ?4
        ORDER BY p.created_at DESC, p.id DESC
        LIMIT ?2 OFFSET ?3
    "#;

    let rows = sqlx::query_as::<_, PostRow>(sql)
        .bind(viewer_id) // ?1
        .bind(limit) // ?2
        .bind(offset) // ?3
        .bind(author_id) // ?4
        .fetch_all(db)
        .await?;

    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM posts WHERE user_id = ?1")
        .bind(author_id)
        .fetch_one(db)
        .await?;

    Ok((rows, total))
}
