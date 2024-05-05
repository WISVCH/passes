//! Web service to create Apple Wallet passes.

use axum::{routing::get, Router};
use tokio::net::TcpListener;
use tokio::signal;

mod models;
mod passes;

use dotenv::dotenv;
use passes::passes_handler;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new()
        .route("/", get(|| async { "OK" }))
        .route("/passes", get(passes_handler));

    let port = std::env::var("PORT").unwrap_or(String::from("3000"));
    println!("Listening on port {}", port);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

/// Graceful shutdown, useful for Docker containers.
///
/// Copied from the
/// [axum graceful-shutdown example](https://github.com/tokio-rs/axum/blob/87b86a7066c320cb388ad4d27f32e7092b56b52f/examples/graceful-shutdown/src/main.rs).
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
