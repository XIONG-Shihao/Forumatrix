use crate::api::docs::doc_query::members;
use crate::api::docs::doc_query::types::JoinRequestRow;
use crate::infra::db::Db;
use sqlx::{Sqlite, Transaction};

/// Export your max members constant from `doc_query/mod.rs`
use crate::api::docs::doc_query::MAX_DOC_MEMBERS;

/// Upsert a join request (unique by (doc_id, user_id)).
pub async fn upsert_join_request(
    db: &Db,
    doc_id: i64,
    user_id: i64,
    message: Option<&str>,
    now: i64,
) -> sqlx::Result<i64> {
    let res = sqlx::query(
        r#"
        INSERT INTO document_join_requests (doc_id, user_id, status, message, created_at)
        VALUES (?, ?, 0, ?, ?)
        ON CONFLICT(doc_id, user_id) DO UPDATE SET
          message = COALESCE(excluded.message, document_join_requests.message)
        "#,
    )
    .bind(doc_id)
    .bind(user_id)
    .bind(message)
    .bind(now)
    .execute(db)
    .await?;

    // Return the row id either way
    let id = sqlx::query_scalar::<_, i64>(
        "SELECT id FROM document_join_requests WHERE doc_id = ? AND user_id = ?",
    )
    .bind(doc_id)
    .bind(user_id)
    .fetch_one(db)
    .await?;
    Ok(id)
}

pub async fn fetch_join_request(db: &Db, req_id: i64) -> sqlx::Result<JoinRequestRow> {
    sqlx::query_as::<_, JoinRequestRow>(
        r#"
        SELECT id, doc_id, user_id, status, message, created_at, decided_at, decided_by
        FROM document_join_requests
        WHERE id = ?
        "#,
    )
    .bind(req_id)
    .fetch_one(db)
    .await
}

/// A typed error you can map cleanly in your HTTP handler.
#[derive(Debug)]
pub enum ApproveJoinError {
    NotOwner,
    NotPending,
    CapacityReached,
    Db(sqlx::Error),
}

impl From<sqlx::Error> for ApproveJoinError {
    fn from(e: sqlx::Error) -> Self {
        ApproveJoinError::Db(e)
    }
}

/// Owner approves: ensures capacity, adds editor, updates request status.
pub async fn approve_join_request(
    db: &Db,
    req_id: i64,
    owner_id: i64,
    now: i64,
) -> Result<(), ApproveJoinError> {
    let mut tx: Transaction<'_, Sqlite> = db.begin().await?;

    // Load request row
    let req: JoinRequestRow = sqlx::query_as(
        r#"
        SELECT id, doc_id, user_id, status, message, created_at, decided_at, decided_by
        FROM document_join_requests
        WHERE id = ?
        "#,
    )
    .bind(req_id)
    .fetch_one(&mut *tx)
    .await?;

    // Verify ownership
    let owner_ok: Option<i64> =
        sqlx::query_scalar("SELECT 1 FROM documents WHERE id = ? AND owner_id = ?")
            .bind(req.doc_id)
            .bind(owner_id)
            .fetch_optional(&mut *tx)
            .await?;
    if owner_ok.is_none() {
        return Err(ApproveJoinError::NotOwner);
    }

    // Only pending (0) can be approved
    if req.status != 0 {
        return Err(ApproveJoinError::NotPending);
    }

    // Capacity check using the generic executor
    let count = members::member_count_including_owner(&mut *tx, req.doc_id).await?;
    if count >= MAX_DOC_MEMBERS {
        return Err(ApproveJoinError::CapacityReached);
    }

    // Add as editor
    members::add_editor(&mut *tx, req.doc_id, req.user_id, now).await?;

    // Mark approved
    sqlx::query(
        r#"
        UPDATE document_join_requests
        SET status = 1, decided_at = ?, decided_by = ?
        WHERE id = ? AND status = 0
        "#,
    )
    .bind(now)
    .bind(owner_id)
    .bind(req_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(())
}

pub async fn deny_join_request(db: &Db, req_id: i64, owner_id: i64, now: i64) -> sqlx::Result<u64> {
    let res = sqlx::query(
        r#"
        UPDATE document_join_requests
        SET status = 2, decided_at = ?, decided_by = ?
        WHERE id = ? AND status = 0
        "#,
    )
    .bind(now)
    .bind(owner_id)
    .bind(req_id)
    .execute(db)
    .await?;
    Ok(res.rows_affected())
}
