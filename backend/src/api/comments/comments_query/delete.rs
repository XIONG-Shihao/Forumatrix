use crate::infra::db::Db;

/// Get the author of a comment and whether that author is an admin.
pub async fn fetch_comment_owner_and_admin(
    db: &Db,
    comment_id: i64,
) -> Result<(i64, bool), sqlx::Error> {
    #[derive(sqlx::FromRow)]
    struct Row {
        user_id: i64,
        is_admin: i64,
    }
    let row: Row = sqlx::query_as(
        r#"
        SELECT c.user_id, u.is_admin
        FROM comments c
        JOIN users u ON u.id = c.user_id
        WHERE c.id = ?
        "#,
    )
    .bind(comment_id)
    .fetch_one(db)
    .await?;

    Ok((row.user_id, row.is_admin == 1))
}

/// Has this comment already been soft-deleted?
pub async fn fetch_comment_deleted_at(
    db: &Db,
    comment_id: i64,
) -> Result<Option<i64>, sqlx::Error> {
    // Tell SQLx the *column* type is Option<i64>, and then flatten the
    // outer Option (row presence) with the inner Option (column NULL).
    let opt: Option<Option<i64>> =
        sqlx::query_scalar::<_, Option<i64>>(r#"SELECT deleted_at FROM comments WHERE id = ?"#)
            .bind(comment_id)
            .fetch_optional(db)
            .await?;
    Ok(opt.flatten())
}

/// User deletes own comment
pub async fn soft_delete_comment_user(
    db: &Db,
    comment_id: i64,
    when: i64,
    caller_id: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"
        UPDATE comments
        SET
          body = '[Deleted By User]',
          deleted_at = ?,
          deleted_by = 1,
          deleted_by_user_id = ?
        WHERE id = ? AND deleted_at IS NULL
        "#,
    )
    .bind(when)
    .bind(caller_id)
    .bind(comment_id)
    .execute(db)
    .await?;

    Ok(res.rows_affected())
}

/// Admin deletes a normal user's comment (reason required)
pub async fn soft_delete_comment_admin(
    db: &Db,
    comment_id: i64,
    when: i64,
    admin_id: i64,
    reason: &str,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"
        UPDATE comments
        SET
          body = '[Deleted By Admin]',
          deleted_at = ?,
          deleted_by = 2,
          deleted_reason = ?,
          deleted_by_user_id = ?
        WHERE id = ? AND deleted_at IS NULL
        "#,
    )
    .bind(when)
    .bind(reason)
    .bind(admin_id)
    .bind(comment_id)
    .execute(db)
    .await?;

    Ok(res.rows_affected())
}

pub async fn fetch_comment_post_id(db: &Db, comment_id: i64) -> Result<i64, sqlx::Error> {
    let pid: i64 = sqlx::query_scalar(r#"SELECT post_id FROM comments WHERE id = ?"#)
        .bind(comment_id)
        .fetch_one(db)
        .await?;
    Ok(pid)
}
