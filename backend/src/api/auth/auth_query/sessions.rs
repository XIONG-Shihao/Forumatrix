use crate::infra::db::Db;
use sqlx::Row;

/// Create a new session row.
pub async fn insert_session(
    db: &Db,
    sid: &str,
    user_id: i64,
    expires_at: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(r#"INSERT INTO sessions (id, user_id, expires_at) VALUES (?, ?, ?)"#)
        .bind(sid)
        .bind(user_id)
        .bind(expires_at)
        .execute(db)
        .await?;
    Ok(())
}

/// Delete exactly one session by its id.
pub async fn delete_session_by_id(db: &Db, sid: &str) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(r#"DELETE FROM sessions WHERE id = ?"#)
        .bind(sid)
        .execute(db)
        .await?;
    Ok(res.rows_affected())
}

/// Delete all sessions for the user who owns this sid.
pub async fn delete_all_sessions_for_sid_owner(db: &Db, sid: &str) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"
        DELETE FROM sessions
         WHERE user_id = (SELECT user_id FROM sessions WHERE id = ?)
        "#,
    )
    .bind(sid)
    .execute(db)
    .await?;
    Ok(res.rows_affected())
}

/// Return user_id if `sid` exists and hasn’t expired yet.
pub async fn user_id_from_sid_if_valid(
    db: &Db,
    sid: &str,
    now: i64,
) -> Result<Option<i64>, sqlx::Error> {
    let row = sqlx::query(r#"SELECT user_id FROM sessions WHERE id = ? AND expires_at > ?"#)
        .bind(sid)
        .bind(now)
        .fetch_optional(db)
        .await?;
    Ok(row.map(|r| r.get::<i64, _>("user_id")))
}
