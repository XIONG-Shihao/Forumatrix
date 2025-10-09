// src/api/docs/types.rs
use crate::infra::db::Db;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct DocumentRow {
    pub id: i64,
    pub owner_id: i64,
    pub title: String,
    pub page_count: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct DocumentPageRow {
    pub id: i64,
    pub doc_id: i64,
    pub page_index: i64,
    pub style: i64,        // 1=Title, 2=Heading, 3=Body
    pub y_update: Vec<u8>, // merged yrs update
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct DocumentCollaboratorRow {
    pub doc_id: i64,
    pub user_id: i64,
    pub role: i64, // always 2 (editor). Owner is in documents.owner_id
    pub added_at: i64,
}

/// Maximum members (owner + collaborators). Keep hard-coded to 10 as requested.
pub const MAX_DOC_MEMBERS: i64 = 10;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PageSnapshotRow {
    pub id: i64,
    pub doc_id: i64,
    pub page_index: i64,
    pub snapshot: Vec<u8>, // full encoded yrs state or snapshot
    pub state_vec: Option<Vec<u8>>,
    pub created_at: i64,
}

#[derive(Debug, Serialize)]
pub struct DocumentMeta {
    pub id: i64,
    pub owner_id: i64,
    pub title: String,
    pub page_count: i64,
    pub created_at: i64,
    pub updated_at: i64,
    pub pages: Vec<PageMeta>, // no content, just meta for nav
}

#[derive(Debug, Serialize)]
pub struct PageMeta {
    pub page_index: i64,
    pub style: i64, // 1/2/3
    pub updated_at: i64,
}

// When a page is opened for editing, you’ll usually deliver the
// current merged `y_update` so the client can hydrate its yrs doc.
// (Clients then connect over WS for live updates.)
#[derive(Debug, Serialize)]
pub struct PageOpenPayload {
    pub doc_id: i64,
    pub page_index: i64,
    pub style: i64,
    pub y_update: Vec<u8>, // merged state to date
}

#[derive(Debug, sqlx::FromRow)]
pub struct JoinRequestRow {
    pub id: i64,
    pub doc_id: i64,
    pub user_id: i64,
    pub status: i64, // 0=pending,1=approved,2=denied
    pub message: Option<String>,
    pub created_at: i64,
    pub decided_at: Option<i64>,
    pub decided_by: Option<i64>,
}
