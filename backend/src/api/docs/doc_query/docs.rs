// src/api/docs/doc_query/docs.rs
use crate::api::docs::doc_handler::types::DocumentRow;
use crate::infra::db::Db;

pub async fn fetch_document(db: &Db, doc_id: i64) -> sqlx::Result<DocumentRow> {
    sqlx::query_as::<_, DocumentRow>(
        r#"
        SELECT id, owner_id, title, page_count, created_at, updated_at
        FROM documents
        WHERE id = ?
        "#,
    )
    .bind(doc_id)
    .fetch_one(db)
    .await
}

pub async fn list_documents_for_user(
    db: &Db,
    user_id: i64,
    page: i64,
    limit: i64,
) -> sqlx::Result<Vec<DocumentRow>> {
    let page = page.max(1);
    let limit = limit.clamp(1, 100);
    let offset = (page - 1) * limit;

    // Member if owner OR editor in collaborators
    sqlx::query_as::<_, DocumentRow>(
        r#"
        SELECT d.*
        FROM documents d
        WHERE d.owner_id = ?
           OR EXISTS (
                SELECT 1 FROM document_collaborators c
                WHERE c.doc_id = d.id AND c.user_id = ? AND c.role = 2
           )
        ORDER BY d.updated_at DESC, d.id DESC
        LIMIT ? OFFSET ?
        "#,
    )
    .bind(user_id)
    .bind(user_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(db)
    .await
}

/// True if user is owner OR editor.
pub async fn user_is_member(db: &Db, doc_id: i64, user_id: i64) -> sqlx::Result<bool> {
    let own: Option<i64> =
        sqlx::query_scalar("SELECT 1 FROM documents WHERE id = ? AND owner_id = ?")
            .bind(doc_id)
            .bind(user_id)
            .fetch_optional(db)
            .await?;
    if own.is_some() {
        return Ok(true);
    }
    let edit: Option<i64> = sqlx::query_scalar(
        "SELECT 1 FROM document_collaborators WHERE doc_id = ? AND user_id = ? AND role = 2",
    )
    .bind(doc_id)
    .bind(user_id)
    .fetch_optional(db)
    .await?;
    Ok(edit.is_some())
}

/// create a new document, returning its ID
pub async fn create_doc(
    db: &Db,
    owner_id: i64,
    title: &str,
    page_count: i64,
    now: i64,
) -> sqlx::Result<i64> {
    let res: sqlx::sqlite::SqliteQueryResult = sqlx::query(
        r#"
        INSERT INTO documents (owner_id, title, page_count, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?)
        "#,
    )
    .bind(owner_id)
    .bind(title)
    .bind(page_count)
    .bind(now)
    .bind(now)
    .execute(db)
    .await?;

    Ok(res.last_insert_rowid())
}

/// Set documents.updated_at = now
pub async fn touch_document(db: &Db, doc_id: i64, now: i64) -> sqlx::Result<u64> {
    let res = sqlx::query(r#"UPDATE documents SET updated_at = ? WHERE id = ?"#)
        .bind(now)
        .bind(doc_id)
        .execute(db)
        .await?;
    Ok(res.rows_affected())
}
