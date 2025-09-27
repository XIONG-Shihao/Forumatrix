use super::types::PostRow;
use crate::infra::db::Db;

#[derive(Clone, Copy, Debug)]
pub enum FeedSort {
    Latest,
    Popular,
    Controversial,
}

pub async fn list_posts(
    db: &Db,
    viewer_id: i64,
    sort: FeedSort,
    page: i64,
    limit: i64,
) -> Result<(Vec<PostRow>, i64), sqlx::Error> {
    // clamp
    let page = page.max(1);
    let limit = limit.clamp(1, 100);
    let offset = (page - 1) * limit;

    let order_by = match sort {
        FeedSort::Latest => "p.created_at DESC, p.id DESC",
        FeedSort::Popular => "p.score DESC, p.comment_count DESC, p.created_at DESC",
        FeedSort::Controversial => "p.comment_count DESC, p.score DESC, p.created_at DESC",
    };
    tracing::info!(
        "posts list: sort={:?} page={} limit={}",
        order_by,
        page,
        limit
    );
    // IMPORTANT: LIMIT ? OFFSET ?  (SQLite)
    // Using CASE WHEN ? IS NULL THEN 0 ELSE EXISTS(...) to support optional viewer id
    let sql = format!(
        r#"
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
        ORDER BY {order_by}
        LIMIT ?2 OFFSET ?3
        "#
    );

    let rows = sqlx::query_as::<_, PostRow>(&sql)
        .bind(viewer_id) // ?1
        .bind(limit) // ?2
        .bind(offset) // ?3
        .fetch_all(db)
        .await?;

    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM posts")
        .fetch_one(db)
        .await?;

    Ok((rows, total))
}
