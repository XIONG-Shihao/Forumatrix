use super::types::CommentRow;
use crate::infra::db::Db;
use sqlx::Row;

// viewer_id required to compute liked_by_me
pub async fn fetch_comment_by_id(
    db: &Db,
    id: i64,
    viewer_id: i64,
) -> Result<Option<CommentRow>, sqlx::Error> {
    sqlx::query_as::<_, CommentRow>(
        r#"
        SELECT
            c.id, c.post_id, c.user_id, c.parent_id, c.body,
            c.created_at, c.updated_at, c.deleted_at, c.edited, c.score,
            u.username AS author_username, u.avatar_url AS author_avatar_url,
            pu.username AS parent_author_username,
            CASE
              WHEN ?2 IS NULL THEN 0
              ELSE EXISTS(SELECT 1 FROM comment_likes cl
                          WHERE cl.comment_id = c.id AND cl.user_id = ?2)
            END AS liked_by_me,
            (SELECT COUNT(*) FROM comments r WHERE r.parent_id = c.id) AS reply_count
        FROM comments c
        JOIN users u ON u.id = c.user_id
        LEFT JOIN comments pc ON pc.id = c.parent_id
        LEFT JOIN users pu ON pu.id = pc.user_id
        WHERE c.id = ?1
        "#,
    )
    .bind(id) // ?1
    .bind(viewer_id) // ?2
    .fetch_optional(db)
    .await
}

/// Returns Some(post_id) if the parent exists, otherwise None.
pub async fn fetch_parent_post_id(db: &Db, parent_id: i64) -> Result<Option<i64>, sqlx::Error> {
    let row = sqlx::query("SELECT post_id FROM comments WHERE id = ?")
        .bind(parent_id)
        .fetch_optional(db)
        .await?;
    Ok(row.map(|r| r.get::<i64, _>("post_id")))
}

pub async fn fetch_comment_author_and_post(
    db: &Db,
    comment_id: i64,
) -> Result<Option<(i64, i64)>, sqlx::Error> {
    let row = sqlx::query(
        "SELECT c.user_id, c.post_id
           FROM comments c
          WHERE c.id = ?",
    )
    .bind(comment_id)
    .fetch_optional(db)
    .await?;

    Ok(row.map(|r| (r.get::<i64, _>("user_id"), r.get::<i64, _>("post_id"))))
}
