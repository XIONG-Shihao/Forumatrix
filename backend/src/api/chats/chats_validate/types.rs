/// Shapes used only by validators → handler/query layers.
/// Keep these small and transport-agnostic.

#[derive(Debug, Clone)]
pub struct OpenChatInput {
    pub caller_id: i64,
    pub peer_id: i64,
}

/// Pagination query coming from axum's `Query<T>` layer.
#[derive(Debug, Default)]
pub struct PageParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

/// Normalized pagination the DB layer expects.
#[derive(Debug, Clone, Copy)]
pub struct Page {
    pub limit: i64,
    pub offset: i64,
}

/// Valid payload to send a message.
#[derive(Debug, Clone)]
pub struct SendMessageInput {
    /// Already trimmed, non-empty, <= 200 UTF-8 chars.
    pub body: String,
}
