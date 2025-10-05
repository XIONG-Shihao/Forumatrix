use crate::infra::db::Db;

pub async fn post_exists(db: &Db, post_id: i64) -> Result<bool, sqlx::Error> {
    let exists: Option<i64> = sqlx::query_scalar("SELECT 1 FROM posts WHERE id = ?")
        .bind(post_id)
        .fetch_optional(db)
        .await?;
    Ok(exists.is_some())
}

pub async fn insert_like(
    conn: &mut sqlx::SqliteConnection,
    post_id: i64,
    user_id: i64,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT OR IGNORE INTO post_likes (post_id, user_id, created_at) VALUES (?,?,?)",
    )
    .bind(post_id)
    .bind(user_id)
    .bind(now)
    .execute(&mut *conn)
    .await?;
    Ok(res.rows_affected())
}

pub async fn delete_like(
    conn: &mut sqlx::SqliteConnection,
    post_id: i64,
    user_id: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("DELETE FROM post_likes WHERE post_id = ? AND user_id = ?")
        .bind(post_id)
        .bind(user_id)
        .execute(&mut *conn)
        .await?;
    Ok(res.rows_affected())
}

pub async fn bump_score(
    conn: &mut sqlx::SqliteConnection,
    post_id: i64,
    delta: i64,
) -> Result<(), sqlx::Error> {
    if delta >= 0 {
        sqlx::query("UPDATE posts SET score = score + ? WHERE id = ?")
            .bind(delta)
            .bind(post_id)
            .execute(&mut *conn)
            .await?;
    } else {
        sqlx::query(
            "UPDATE posts
             SET score = CASE WHEN score + ? < 0 THEN 0 ELSE score + ? END
             WHERE id = ?",
        )
        .bind(delta)
        .bind(delta)
        .bind(post_id)
        .execute(&mut *conn)
        .await?;
    }
    Ok(())
}

pub async fn fetch_score(
    conn: &mut sqlx::SqliteConnection,
    post_id: i64,
) -> Result<i64, sqlx::Error> {
    let s: i64 = sqlx::query_scalar("SELECT score FROM posts WHERE id = ?")
        .bind(post_id)
        .fetch_one(&mut *conn)
        .await?;
    Ok(s)
}
