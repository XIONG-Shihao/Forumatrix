// src/api/docs/doc_query/mod.rs
pub mod docs;
pub mod helpers;
pub mod join_requests;
pub mod list;
pub mod members;
pub mod pages;
pub mod types;

// One place for shared constants:
pub const MAX_DOC_MEMBERS: i64 = 10; // owner + up to 9 editors
