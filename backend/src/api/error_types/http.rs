use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

use super::codes::*;
use super::core::ApiError;
use super::validation::ValidationError;

/// Public JSON shape for all errors.
#[derive(Serialize)]
struct ErrorBody<'a> {
    error: ErrorPayload<'a>,
}

#[derive(Serialize)]
struct ErrorPayload<'a> {
    code: &'a str,   // stable machine-readable code
    message: String, // user-facing message
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            ApiError::Validation(v) => {
                let (code, msg) = match v {
                    // auth
                    ValidationError::EmailInvalid => (VALIDATION_EMAIL_INVALID, "invalid email"),
                    ValidationError::UsernameInvalid => {
                        (VALIDATION_USERNAME_INVALID, "invalid username")
                    }
                    ValidationError::PasswordTooShort => (
                        VALIDATION_PASSWORD_TOO_SHORT,
                        "password must be at least 8 characters",
                    ),
                    ValidationError::MissingIdentifierOrPassword => {
                        (VALIDATION_LOGIN_MISSING, "missing identifier or password")
                    }

                    // posts
                    ValidationError::TitleRequired => (VALIDATION_TITLE_REQUIRED, "title required"),
                    ValidationError::BodyRequired => (VALIDATION_BODY_REQUIRED, "body required"),
                    ValidationError::NothingToUpdate => {
                        (VALIDATION_NOTHING_TO_UPDATE, "nothing to update")
                    }
                    ValidationError::TitleEmpty => {
                        (VALIDATION_TITLE_EMPTY, "title cannot be empty")
                    }
                    ValidationError::BodyEmpty => (VALIDATION_BODY_EMPTY, "body cannot be empty"),
                    ValidationError::ParentNotFound => {
                        (VALIDATION_PARENT_NOT_FOUND, "parent comment not found")
                    }
                    ValidationError::ParentNotInPost => (
                        VALIDATION_PARENT_NOT_IN_POST,
                        "parent comment belongs to another post",
                    ),
                    ValidationError::PostNotFound => (VALIDATION_POST_NOT_FOUND, "post not found"),
                    ValidationError::CommentNotFound => {
                        (VALIDATION_COMMENT_NOT_FOUND, "comment not found")
                    }
                    // tags
                    ValidationError::TagNameInvalid => {
                        (VALIDATION_TAG_NAME_INVALID, "invalid tag name")
                    }
                    ValidationError::TagNotFound => (VALIDATION_TAG_NOT_FOUND, "tag not found"),
                    ValidationError::TagAlreadyAttached => {
                        (VALIDATION_TAG_ALREADY_ATTACHED, "tag already attached")
                    }

                    // ---- reports ----
                    ValidationError::ReportEntityTypeInvalid => {
                        (REPORT_ENTITY_TYPE_INVALID, "invalid report entity type")
                    } // not in {"post","comment","user"}
                    ValidationError::ReportReasonEmpty => {
                        (REPORT_REASON_EMPTY, "report reason cannot be empty")
                    }
                    ValidationError::ReportReasonTooLong => {
                        (REPORT_REASON_TOO_LONG, "report reason is too long")
                    }
                    ValidationError::ReportStatusInvalid => {
                        (REPORT_STATUS_INVALID, "invalid report status")
                    } // not in {"open","reviewed","dismissed","actioned"}
                    ValidationError::ReportTargetNotFound => {
                        (REPORT_TARGET_NOT_FOUND, "reported target not found")
                    } // reported object doesn't exist

                    // Avatar-specific
                    ValidationError::AvatarBadMultipart => {
                        (AVATAR_BAD_MULTIPART, "bad multipart form data")
                    }
                    ValidationError::AvatarMissingFile => {
                        (AVATAR_MISSING_FILE, "missing file field")
                    }
                    ValidationError::AvatarTooLarge => (AVATAR_TOO_LARGE, "file too large (>2MB)"),
                    ValidationError::AvatarInvalidFormat => {
                        (AVATAR_INVALID_FORMAT, "only PNG or JPEG allowed")
                    }
                    ValidationError::AvatarDecodeFailed => {
                        (AVATAR_DECODE_FAILED, "failed to decode image")
                    }
                    // Users (profile updates)
                    ValidationError::UserDobInvalid => {
                        (VALIDATION_USER_DOB_INVALID, "invalid date of birth")
                    }
                    ValidationError::UserBioTooLong => {
                        (VALIDATION_USER_BIO_TOO_LONG, "bio is too long")
                    }
                    // Notifications
                    ValidationError::InvalidPage => (VALIDATION_INVALIDE_PAGE, "invalid page"),
                    ValidationError::InvalidLimit => (VALIDATION_INVALIDE_LIMIT, "invalid limit"),
                    ValidationError::EmptyIds => (VALIDATION_EMPTY_IDS, "ids cannot be empty"),
                    ValidationError::TooManyIds => (VALIDATION_TOO_MANY_IDS, "too many ids"),
                    ValidationError::MissingField => (VALIDATION_MISSING_FIELD, "missing field"),

                    // Chat related
                    ValidationError::ChatNotFound => (VALIDATION_CHAT_NOT_FOUND, "chat not found"),
                    ValidationError::MessageTooLong => {
                        (VALIDATION_MESSAGE_TOO_LONG, "message is too long")
                    }
                    ValidationError::ChatForbidden => {
                        (VALIDATION_CHAT_FORBIDDEN, "not allowed in this chat")
                    }
                    ValidationError::MessageEmpty => {
                        (VALIDATION_MESSAGE_EMPTY, "message cannot be empty")
                    }
                    ValidationError::DeleteReasonEmpty => (
                        VALIDATION_DELETE_REASON_EMPTY,
                        "Deletion reason is required",
                    ),
                    ValidationError::DeleteReasonTooLong => (
                        VALIDATION_DELETE_REASON_TOO_LONG,
                        "Deletion reason is too long (max 200)",
                    ),
                    ValidationError::PostAlreadyDeleted => {
                        (VALIDATION_POST_ALREADY_DELETED, "Post is already deleted")
                    }
                    ValidationError::DeleteNotOwner => (
                        VALIDATION_DELETE_NOT_OWNER,
                        "Only the post owner or an admin can delete this post",
                    ),
                    ValidationError::DeleteTargetIsAdmin => (
                        VALIDATION_DELETE_TARGET_IS_ADMIN,
                        "Admin-authored posts cannot be deleted",
                    ),
                    ValidationError::UserAlreadySuspended => (
                        VALIDATION_USER_ALREADY_SUSPENDED,
                        "User is already suspended",
                    ),
                    ValidationError::CannotSuspendSelf => (
                        VALIDATION_CANNOT_SUSPEND_SELF,
                        "You cannot suspend yourself",
                    ),
                    ValidationError::CannotSuspendAdmin => (
                        VALIDATION_CANNOT_SUSPEND_ADMIN,
                        "You cannot suspend an admin user",
                    ),
                    ValidationError::CommentAlreadyDeleted => (
                        VALIDATION_COMMENT_ALREADY_DELETED,
                        "Comment is already deleted",
                    ),
                    ValidationError::CannotDeleteOthersComment => (
                        VALIDATION_CANNOT_DELETE_OTHERS_COMMENT,
                        "You can only delete your own comments",
                    ),
                    ValidationError::CannotDeleteAdminComment => (
                        VALIDATION_CANNOT_DELETE_ADMIN_COMMENT,
                        "You cannot delete comments made by admin users",
                    ),
                    // Document (real-time collaboration)
                    ValidationError::DocTitleEmpty => {
                        (VALIDATION_DOC_TITLE_EMPTY, "document title cannot be empty")
                    }
                    ValidationError::DocTitleTooLong => {
                        (VALIDATION_DOC_TITLE_TOO_LONG, "document title is too long")
                    }
                    ValidationError::DocNotFound => {
                        (VALIDATION_DOC_NOT_FOUND, "document not found")
                    }
                    ValidationError::NotDocViewer => {
                        (VALIDATION_NOT_DOC_VIEWER, "not a viewer of this document")
                    }
                    ValidationError::NotDocEditor => {
                        (VALIDATION_NOT_DOC_EDITOR, "not an editor of this document")
                    }
                    ValidationError::NotDocOwner => {
                        (VALIDATION_NOT_DOC_OWNER, "not the owner of this document")
                    }
                    ValidationError::InvalidMemberRole => {
                        (VALIDATION_INVALID_MEMBER_ROLE, "invalid member role")
                    }
                    ValidationError::CannotShareWithSelf => (
                        VALIDATION_CANNOT_SHARE_WITH_SELF,
                        "cannot share document with yourself",
                    ),
                    ValidationError::SnapshotTooLarge => {
                        (VALIDATION_SNAPSHOT_TOO_LARGE, "snapshot is too large")
                    }
                    ValidationError::NegativeAfterSeq => (
                        VALIDATION_NEGATIVE_AFTER_SEQ,
                        "after_seq cannot be negative",
                    ),
                    ValidationError::UpdateEmpty => {
                        (VALIDATION_UPDATE_EMPTY, "update cannot be empty")
                    }
                    ValidationError::UpdateTooLarge => {
                        (VALIDATION_UPDATE_TOO_LARGE, "update is too large")
                    }
                    ValidationError::SnapshotEmpty => {
                        (VALIDATION_SNAPSHOT_EMPTY, "snapshot cannot be empty")
                    }
                };

                (StatusCode::BAD_REQUEST, code, msg.to_string())
            }
            ApiError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                UNAUTHORIZED,
                "authentication required".into(),
            ),
            ApiError::Forbidden => (StatusCode::FORBIDDEN, FORBIDDEN, "not allowed".into()),
            ApiError::NotFound => (
                StatusCode::NOT_FOUND,
                NOT_FOUND,
                "resource not found".into(),
            ),
            ApiError::Conflict { message } => (StatusCode::CONFLICT, CONFLICT, message),
            ApiError::Internal { message } => {
                (StatusCode::INTERNAL_SERVER_ERROR, INTERNAL, message)
            }
        };

        let body = Json(ErrorBody {
            error: ErrorPayload { code, message },
        });
        (status, body).into_response()
    }
}
