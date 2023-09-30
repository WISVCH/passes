use axum::{routing::get, Router};
use std::net::SocketAddr;
use std::str::FromStr;
mod passes;

mod models;

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

    // // convert the port to a socket address
    let addr = SocketAddr::from_str(&format!("0.0.0.0:{}", port)).unwrap();

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
