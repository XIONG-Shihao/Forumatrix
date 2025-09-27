use crate::infra::db::AppState;
use axum::{routing::post, Router};

pub mod cookie_helper;
pub mod login;
pub mod logout;
pub mod logout_all;
pub mod password;
pub mod register;
pub mod session;
pub mod types;
pub mod utils;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/auth/register", post(register::register))
        .route("/api/auth/login", post(login::login))
        .route("/api/auth/logout", post(logout::logout)) // single session
        .route("/api/auth/logout_all", post(logout_all::logout_all)) // all sessions
}
