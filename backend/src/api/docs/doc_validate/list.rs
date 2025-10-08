// backend/src/api/docs/doc_validate/list.rs

/// Normalize pagination inputs (defaults + clamping).
/// We keep it simple and *never* error on pagination—just coerce.
pub fn normalize_pagination(page_in: Option<i64>, limit_in: Option<i64>) -> (i64, i64) {
    let page = page_in.unwrap_or(1).max(1);
    let limit = limit_in.unwrap_or(20).clamp(1, 100);
    (page, limit)
}
