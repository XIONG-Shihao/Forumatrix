use super::types::PostRow;
use crate::infra::db::Db;

/// List posts that have a given `tag_id`.
/// `sort`: "new" (default) or "top"
pub async fn list_posts_by_tag_id(
    db: &Db,
    tag_id: i64,
    sort: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<PostRow>, sqlx::Error> {
    let order_clause = match sort {
        "top" => " ORDER BY p.score DESC, p.created_at DESC ",
        _ => " ORDER BY p.created_at DESC ",
    };

    let sql = format!(
        r#"
        SELECT
            p.id, p.user_id, p.title, p.body,
            p.created_at, p.updated_at, p.edited, p.score, p.comment_count,
            u.username AS author_username
        FROM post_tags pt
        JOIN posts p  ON p.id = pt.post_id
        JOIN users u  ON u.id = p.user_id
        WHERE pt.tag_id = ?
        {order}
        LIMIT ? OFFSET ?
        "#,
        order = order_clause
    );

    sqlx::query_as::<_, PostRow>(&sql)
        .bind(tag_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(db)
        .await
}
