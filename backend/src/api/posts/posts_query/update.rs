use crate::infra::db::Db;
use sqlx::Row;

/// Fetch the owner (user_id) of a post, or None if not found.
pub async fn fetch_post_owner(db: &Db, post_id: i64) -> Result<Option<i64>, sqlx::Error> {
    let row = sqlx::query("SELECT user_id FROM posts WHERE id = ?")
        .bind(post_id)
        .fetch_optional(db)
        .await?;
    Ok(row.map(|r| r.get::<i64, _>("user_id")))
}

/// Update selected fields; returns rows_affected (0 if no changes or not matched).
pub async fn update_post_fields(
    db: &Db,
    post_id: i64,
    title: Option<&str>,
    body: Option<&str>,
    now_unix: i64,
) -> Result<u64, sqlx::Error> {
    // Build dynamic SET list
    let mut set_parts: Vec<&str> = Vec::new();
    if title.is_some() {
        set_parts.push("title = ?");
    }
    if body.is_some() {
        set_parts.push("body  = ?");
    }
    // Always bump updated_at and mark edited
    set_parts.push("updated_at = ?");
    set_parts.push("edited = 1");

    let sql = format!("UPDATE posts SET {} WHERE id = ?", set_parts.join(", "));

    let mut q = sqlx::query(&sql);
    if let Some(t) = title {
        q = q.bind(t);
    }
    if let Some(b) = body {
        q = q.bind(b);
    }
    q = q.bind(now_unix);
    q = q.bind(post_id);

    let res = q.execute(db).await?;
    Ok(res.rows_affected())
}
