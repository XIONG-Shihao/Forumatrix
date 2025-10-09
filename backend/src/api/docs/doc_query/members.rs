use crate::infra::db::Db;
use sqlx::FromRow;
use sqlx::{Executor, Sqlite};

#[derive(Debug, FromRow)]
pub struct OwnerRow {
    pub owner_id: i64,
    pub created_at: i64,
}

#[derive(Debug, FromRow)]
pub struct EditorRow {
    pub user_id: i64,
    pub role: i64, // expect 2
    pub added_at: i64,
}

/// Is the user the owner?
pub async fn is_owner<'e, E>(exec: E, doc_id: i64, user_id: i64) -> sqlx::Result<bool>
where
    E: Executor<'e, Database = Sqlite>,
{
    let own: Option<i64> =
        sqlx::query_scalar("SELECT 1 FROM documents WHERE id = ? AND owner_id = ?")
            .bind(doc_id)
            .bind(user_id)
            .fetch_optional(exec)
            .await?;
    Ok(own.is_some())
}

/// Can the user edit? (owner counts as editor)
pub async fn is_editor<'e, E>(mut exec: E, doc_id: i64, user_id: i64) -> sqlx::Result<bool>
where
    E: Executor<'e, Database = Sqlite> + Copy,
{
    if is_owner(exec, doc_id, user_id).await? {
        return Ok(true);
    }
    let edit: Option<i64> = sqlx::query_scalar(
        "SELECT 1 FROM document_collaborators WHERE doc_id = ? AND user_id = ? AND role = 2",
    )
    .bind(doc_id)
    .bind(user_id)
    .fetch_optional(exec)
    .await?;
    Ok(edit.is_some())
}

/// Count members (owner + editors).
pub async fn member_count_including_owner<'e, E>(exec: E, doc_id: i64) -> sqlx::Result<i64>
where
    E: Executor<'e, Database = Sqlite>,
{
    let editors: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM document_collaborators WHERE doc_id = ? AND role = 2",
    )
    .bind(doc_id)
    .fetch_one(exec)
    .await?;
    Ok(1 + editors) // +1 owner
}

/// Is there capacity to add one more member?
pub async fn has_capacity<'e, E>(exec: E, doc_id: i64, max_members: i64) -> sqlx::Result<bool>
where
    E: Executor<'e, Database = Sqlite>,
{
    let count = member_count_including_owner(exec, doc_id).await?;
    Ok(count < max_members)
}

pub async fn add_editor<'e, E>(exec: E, doc_id: i64, user_id: i64, now: i64) -> sqlx::Result<u64>
where
    E: Executor<'e, Database = Sqlite>,
{
    let res = sqlx::query(
        r#"
        INSERT OR IGNORE INTO document_collaborators (doc_id, user_id, role, added_at)
        VALUES (?, ?, 2, ?)
        "#,
    )
    .bind(doc_id)
    .bind(user_id)
    .bind(now)
    .execute(exec)
    .await?;
    Ok(res.rows_affected())
}

pub async fn remove_editor<'e, E>(exec: E, doc_id: i64, user_id: i64) -> sqlx::Result<u64>
where
    E: Executor<'e, Database = Sqlite>,
{
    let res = sqlx::query(r#"DELETE FROM document_collaborators WHERE doc_id = ? AND user_id = ?"#)
        .bind(doc_id)
        .bind(user_id)
        .execute(exec)
        .await?;
    Ok(res.rows_affected())
}

pub async fn fetch_owner_and_created(db: &Db, doc_id: i64) -> sqlx::Result<(i64, i64)> {
    let row: OwnerRow = sqlx::query_as(
        r#"
        SELECT owner_id, created_at
        FROM documents
        WHERE id = ?
        "#,
    )
    .bind(doc_id)
    .fetch_one(db)
    .await?;

    Ok((row.owner_id, row.created_at))
}

/// List all editors (role=2) for a doc, ordered by added_at ASC.
pub async fn list_editors_by_doc(db: &Db, doc_id: i64) -> sqlx::Result<Vec<EditorRow>> {
    let rows: Vec<EditorRow> = sqlx::query_as(
        r#"
        SELECT user_id, role, added_at
        FROM document_collaborators
        WHERE doc_id = ? AND role = 2
        ORDER BY added_at ASC
        "#,
    )
    .bind(doc_id)
    .fetch_all(db)
    .await?;

    Ok(rows)
}
