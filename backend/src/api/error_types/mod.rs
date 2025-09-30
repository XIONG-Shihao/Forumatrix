pub mod codes;
pub mod core;
pub mod db;
pub mod http;
pub mod validation;

// Re-exports for ergonomic imports elsewhere
pub use core::{ApiError, ApiResult};
pub use validation::ValidationError;
