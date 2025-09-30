use super::core::ApiError;

/// Central place to map `sqlx::Error` to `ApiError`.
/// SQLite notes:
/// - UNIQUE constraint violation → extended code 2067
/// - RowNotFound (for `fetch_one`) maps nicely to 404
impl From<sqlx::Error> for ApiError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => ApiError::NotFound,
            sqlx::Error::Database(db_err) => {
                // Try a robust unique check for SQLite
                let is_unique = db_err.code().map(|c| c.as_ref() == "2067").unwrap_or(false)
                    || db_err.message().contains("UNIQUE constraint failed");

                if is_unique {
                    ApiError::Conflict {
                        message: "unique constraint violation".into(),
                    }
                } else {
                    ApiError::Internal {
                        message: db_err.message().to_string(),
                    }
                }
            }
            other => ApiError::Internal {
                message: other.to_string(),
            },
        }
    }
}
