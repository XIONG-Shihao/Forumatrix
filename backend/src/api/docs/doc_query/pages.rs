use super::docs::touch_document;
use crate::api::docs::doc_handler::types::{PageMeta, PageOpenPayload};
use crate::infra::db::Db;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
struct PageMetaRow {
    page_index: i64,
    style: i64,
    updated_at: i64,
}

pub async fn list_page_meta(db: &Db, doc_id: i64) -> sqlx::Result<Vec<PageMeta>> {
    let rows = sqlx::query_as::<_, PageMetaRow>(
        r#"
        SELECT page_index, style, updated_at
        FROM document_pages
        WHERE doc_id = ?
        ORDER BY page_index ASC
        "#,
    )
    .bind(doc_id)
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| PageMeta {
            page_index: r.page_index,
            style: r.style,
            updated_at: r.updated_at,
        })
        .collect())
}

#[derive(Debug, FromRow)]
struct PageOpenRow {
    style: i64,
    y_update: Vec<u8>,
}

pub async fn open_page_payload(
    db: &Db,
    doc_id: i64,
    page_index: i64,
) -> sqlx::Result<PageOpenPayload> {
    let row = sqlx::query_as::<_, PageOpenRow>(
        r#"
        SELECT style, y_update
        FROM document_pages
        WHERE doc_id = ? AND page_index = ?
        "#,
    )
    .bind(doc_id)
    .bind(page_index)
    .fetch_one(db)
    .await?;

    Ok(PageOpenPayload {
        doc_id,
        page_index,
        style: row.style,
        y_update: row.y_update,
    })
}

/// Upsert/replace page update (merged y_update + style).
pub async fn upsert_page_update(
    db: &Db,
    doc_id: i64,
    page_index: i64,
    style: i64,
    y_update: &[u8],
    now: i64,
) -> sqlx::Result<u64> {
    // Do both writes in a transaction for consistency.
    let mut tx = db.begin().await?;

    let res = sqlx::query(
        r#"
        INSERT INTO document_pages (doc_id, page_index, style, y_update, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?)
        ON CONFLICT(doc_id, page_index) DO UPDATE SET
          style = excluded.style,
          y_update = excluded.y_update,
          updated_at = excluded.updated_at
        "#,
    )
    .bind(doc_id)
    .bind(page_index)
    .bind(style)
    .bind(y_update)
    .bind(now)
    .bind(now)
    .execute(&mut *tx)
    .await?;

    // Bump parent document's updated_at so the DocsHome list reflects the save.
    sqlx::query(r#"UPDATE documents SET updated_at = ? WHERE id = ?"#)
        .bind(now)
        .bind(doc_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(res.rows_affected())
}
