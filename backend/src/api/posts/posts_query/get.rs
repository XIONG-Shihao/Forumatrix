use super::types::PostRow;
use crate::infra::db::Db;
use sqlx::Row;

/// Fetch one post by id including author username.
pub async fn fetch_post_by_id(
    db: &Db,
    id: i64,
    viewer_id: i64, // 👈 NEW
) -> Result<Option<PostRow>, sqlx::Error> {
    let sql = r#"
        SELECT
            p.id, p.user_id, p.title, p.body,
            p.created_at, p.updated_at, p.edited, p.score, p.comment_count,
            u.username AS author_username,
            u.avatar_url AS author_avatar_url,
            CASE
              WHEN ?2 IS NULL THEN 0
              ELSE EXISTS(SELECT 1 FROM post_likes pl WHERE pl.post_id = p.id AND pl.user_id = ?2)
            END AS liked_by_me
        FROM posts p
        JOIN users u ON u.id = p.user_id
        WHERE p.id = ?1
    "#;

    sqlx::query_as::<_, PostRow>(sql)
        .bind(id) // ?1
        .bind(viewer_id) // ?2
        .fetch_optional(db)
        .await
}

/// Returns (author_user_id, title) for the post.
pub async fn fetch_post_author_and_title(
    db: &Db,
    post_id: i64,
) -> Result<Option<(i64, String)>, sqlx::Error> {
    let row = sqlx::query(
        "SELECT u.id as user_id, p.title
           FROM posts p
           JOIN users u ON u.id = p.user_id
          WHERE p.id = ?",
    )
    .bind(post_id)
    .fetch_optional(db)
    .await?;

    Ok(row.map(|r| (r.get::<i64, _>("user_id"), r.get::<String, _>("title"))))
}
