use axum::{routing::get, Router};
use tokio::net::TcpListener;

mod models;
mod passes;

extern crate dotenv;
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
        .await
        .unwrap();
}
