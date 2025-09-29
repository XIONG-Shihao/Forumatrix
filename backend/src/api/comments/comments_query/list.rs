use crate::api::comments::comments_handler::types::{CommentRow, CommentSort};
use crate::infra::db::Db;

pub async fn count_for_post(db: &Db, post_id: i64) -> Result<i64, sqlx::Error> {
    let n: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM comments WHERE post_id = ?")
        .bind(post_id)
        .fetch_one(db)
        .await?;
    Ok(n)
}

pub async fn list_for_post(
    db: &Db,
    post_id: i64,
    sort: CommentSort,
    page: i64,
    limit: i64,
    viewer_id: i64, // since all pages require auth now
) -> Result<Vec<CommentRow>, sqlx::Error> {
    let page = page.max(1);
    let limit = limit.clamp(1, 200);
    let offset = (page - 1) * limit;

    let base = r#"
        SELECT
            c.id, c.post_id, c.user_id, c.parent_id, c.body,
            c.created_at, c.updated_at, c.deleted_at, c.edited, c.score,
            u.username AS author_username, u.avatar_url AS author_avatar_url,
            pu.username AS parent_author_username,
            EXISTS(
              SELECT 1 FROM comment_likes cl
              WHERE cl.comment_id = c.id AND cl.user_id = ?4
            ) AS liked_by_me,
            (SELECT COUNT(*) FROM comments r WHERE r.parent_id = c.id) AS reply_count,
            u.is_admin AS author_is_admin                     -- <-- ADD THIS
        FROM comments c
        JOIN users u ON u.id = c.user_id
        LEFT JOIN comments pc ON pc.id = c.parent_id
        LEFT JOIN users pu ON pu.id = pc.user_id
        WHERE c.post_id = ?1
          AND c.deleted_at IS NULL                            -- <-- HIDE DELETED
    "#;

    let order = match sort {
        CommentSort::Created => "ORDER BY c.created_at ASC, c.id ASC",
        CommentSort::Score => "ORDER BY c.score DESC, c.created_at ASC",
    };

    let sql = format!("{base} {order} LIMIT ?2 OFFSET ?3");

    sqlx::query_as::<_, CommentRow>(&sql)
        .bind(post_id) // ?1
        .bind(limit) // ?2
        .bind(offset) // ?3
        .bind(viewer_id) // ?4
        .fetch_all(db)
        .await
}
