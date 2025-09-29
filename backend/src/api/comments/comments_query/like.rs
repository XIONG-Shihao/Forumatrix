use crate::infra::db::Db;

pub async fn comment_exists(db: &Db, comment_id: i64) -> Result<bool, sqlx::Error> {
    let exists: Option<i64> = sqlx::query_scalar("SELECT 1 FROM comments WHERE id = ?")
        .bind(comment_id)
        .fetch_optional(db)
        .await?;
    Ok(exists.is_some())
}

pub async fn insert_like(
    conn: &mut sqlx::SqliteConnection,
    comment_id: i64,
    user_id: i64,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT OR IGNORE INTO comment_likes (comment_id, user_id, created_at) VALUES (?,?,?)",
    )
    .bind(comment_id)
    .bind(user_id)
    .bind(now)
    .execute(&mut *conn)
    .await?;
    Ok(res.rows_affected())
}

pub async fn delete_like(
    conn: &mut sqlx::SqliteConnection,
    comment_id: i64,
    user_id: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("DELETE FROM comment_likes WHERE comment_id = ? AND user_id = ?")
        .bind(comment_id)
        .bind(user_id)
        .execute(&mut *conn)
        .await?;
    Ok(res.rows_affected())
}

pub async fn bump_score(
    conn: &mut sqlx::SqliteConnection,
    comment_id: i64,
    delta: i64,
) -> Result<(), sqlx::Error> {
    if delta >= 0 {
        sqlx::query("UPDATE comments SET score = score + ? WHERE id = ?")
            .bind(delta)
            .bind(comment_id)
            .execute(&mut *conn)
            .await?;
    } else {
        sqlx::query(
            "UPDATE comments
             SET score = CASE WHEN score + ? < 0 THEN 0 ELSE score + ? END
             WHERE id = ?",
        )
        .bind(delta)
        .bind(delta)
        .bind(comment_id)
        .execute(&mut *conn)
        .await?;
    }
    Ok(())
}

pub async fn fetch_score(
    conn: &mut sqlx::SqliteConnection,
    comment_id: i64,
) -> Result<i64, sqlx::Error> {
    let s: i64 = sqlx::query_scalar("SELECT score FROM comments WHERE id = ?")
        .bind(comment_id)
        .fetch_one(&mut *conn)
        .await?;
    Ok(s)
}
