// backend/src/api/docs/doc_query/list.rs
use crate::api::docs::doc_handler::types::DocumentRow;
use crate::infra::db::Db;

/// Total documents where the user is owner or editor (role=2).
pub async fn count_documents_for_user(db: &Db, user_id: i64) -> sqlx::Result<i64> {
    sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)
        FROM documents d
        WHERE d.owner_id = ?
           OR EXISTS (
                SELECT 1 FROM document_collaborators c
                WHERE c.doc_id = d.id AND c.user_id = ? AND c.role = 2
           )
        "#,
    )
    .bind(user_id) // owner
    .bind(user_id) // editor
    .fetch_one(db)
    .await
}

/// Page of documents for user membership (owner or editor).
pub async fn list_documents_for_user(
    db: &Db,
    user_id: i64,
    page: i64,
    limit: i64,
) -> sqlx::Result<Vec<DocumentRow>> {
    let offset = (page - 1) * limit;

    sqlx::query_as::<_, DocumentRow>(
        r#"
        SELECT d.id, d.owner_id, d.title, d.page_count, d.created_at, d.updated_at
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
    .bind(user_id) // owner
    .bind(user_id) // editor
    .bind(limit)
    .bind(offset)
    .fetch_all(db)
    .await
}
