// backend/src/api/docs/doc_query/helpers.rs
use sqlx::Row;

use crate::infra::db::Db;

/// Returns (owner_id, title, page_count, created_at, updated_at)
pub async fn fetch_doc_row(
    db: &Db,
    doc_id: i64,
) -> sqlx::Result<Option<(i64, String, i64, i64, i64)>> {
    let row = sqlx::query(
        r#"
        SELECT owner_id, title, page_count, created_at, updated_at
        FROM documents
        WHERE id = ?
        "#,
    )
    .bind(doc_id)
    .fetch_optional(db)
    .await?;

    Ok(row.map(|r| {
        (
            r.get::<i64, _>("owner_id"),
            r.get::<String, _>("title"),
            r.get::<i64, _>("page_count"),
            r.get::<i64, _>("created_at"),
            r.get::<i64, _>("updated_at"),
        )
    }))
}

pub async fn create_initial_pages(
    db: &Db,
    doc_id: i64,
    page_count: i64,
    now: i64,
) -> sqlx::Result<()> {
    let mut tx = db.begin().await?;
    for idx in 0..page_count {
        sqlx::query(
            r#"
            INSERT INTO document_pages (doc_id, page_index, style, y_update, created_at, updated_at)
            VALUES (?, ?, 3, x'', ?, ?)
            "#,
        )
        .bind(doc_id)
        .bind(idx)
        .bind(now)
        .bind(now)
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    Ok(())
}
