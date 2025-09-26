use axum::{routing::get, Router};
use std::{env, net::SocketAddr};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod infra;
use infra::db::{init_state, AppState};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            env::var("RUST_LOG").unwrap_or_else(|_| "info,tower_http=info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state: AppState = init_state().await.expect("DB init & migrations");

    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080);

    let app = Router::new()
        .route(
            "/api/status",
            get({
                let port = port;
                move || async move { format!("running on port {}", port) }
            }),
        )
        .merge(api::router()) // 👈 mount auth endpoints
        // ✅ serve uploaded files at /static/...
        .nest_service("/static", ServeDir::new("/app/uploads"))
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    let addr: SocketAddr = ([0, 0, 0, 0], port).into();
    tracing::info!("listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
