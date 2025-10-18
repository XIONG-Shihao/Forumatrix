/// Transport-free validation errors from validators.
/// Group by feature to keep it tidy as the app grows.

#[derive(Debug)]
pub enum ValidationError {
    // Auth
    EmailInvalid,
    UsernameInvalid,
    PasswordTooShort,
    MissingIdentifierOrPassword,

    // Posts
    TitleRequired,
    BodyRequired,
    NothingToUpdate,
    TitleEmpty,
    BodyEmpty,
    PostNotFound,

    // Comments (new)
    ParentNotFound,  // parent_id points to a non-existent comment
    ParentNotInPost, // parent_id belongs to a different post than the path :post_id
    CommentNotFound, // comment_id points to a non-existent comment

    // Tags
    TagNameInvalid,
    TagNotFound,
    TagAlreadyAttached,

    // ---- reports ----
    ReportEntityTypeInvalid, // not in {"post","comment","user"}
    ReportReasonEmpty,
    ReportReasonTooLong,
    ReportStatusInvalid,  // not in {"open","reviewed","dismissed","actioned"}
    ReportTargetNotFound, // reported object doesn't exist

    // Avatar
    AvatarBadMultipart,  // multipart could not be parsed
    AvatarMissingFile,   // no "file" field
    AvatarTooLarge,      // file size exceeds limit
    AvatarInvalidFormat, // not PNG or JPEG
    AvatarDecodeFailed,  // failed to decode image data

    // Users (profile updates)
    UserDobInvalid, // not valid ISO-8601 date or out of reasonable range
    UserBioTooLong, // > MAX_BIO_CHARS (e.g., 500)

    // Notifications
    InvalidPage,
    InvalidLimit,
    EmptyIds,
    TooManyIds,
    MissingField,

    // Chat related
    ChatNotFound,
    MessageTooLong,
    ChatForbidden,
    MessageEmpty,

    // Post delete
    DeleteReasonEmpty,
    DeleteReasonTooLong,
    PostAlreadyDeleted,
    DeleteNotOwner,
    DeleteTargetIsAdmin,

    // Suspension
    UserAlreadySuspended,
    CannotSuspendSelf,
    CannotSuspendAdmin,

    // Comment delete
    CommentAlreadyDeleted,
    CannotDeleteOthersComment,
    CannotDeleteAdminComment,

    // Document (real-time collaboration)
    DocTitleEmpty,
    DocTitleTooLong,
    DocNotFound, // or prefer ApiError::NotFound
    NotDocViewer,
    NotDocEditor,
    NotDocOwner,
    InvalidMemberRole,
    CannotShareWithSelf, // optional
    NegativeAfterSeq,
    UpdateEmpty,
    UpdateTooLarge,
    SnapshotEmpty,
    SnapshotTooLarge,

    // NEW: docs/pages
    DocPagesLimitReached,   // > 10 requested or would exceed 10
    DocPageIndexOutOfRange, // index not within 0..page_count-1
    DocPageNotFound,        // row missing
    PageStyleInvalid,       // not 1/2/3

    // NEW: collaboration / join requests
    JoinAlreadyMember,     // already in collaborators
    JoinAlreadyPending,    // a pending request already exists
    JoinRequestNotFound,   // request id/doc mismatch or missing
    JoinRequestNotPending, // request exists but not in pending state
    CannotRequestOwnDoc,   // owner trying to request join
    CannotRemoveOwner,     // trying to remove document owner

    // NEW: realtime / concurrency
    DocMembersLimitReached, // > 10 concurrent editors on page
}
