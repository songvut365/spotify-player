use axum::{http::StatusCode, routing::get, Router};
use dotenv::dotenv;
use std::{env, net::SocketAddr};
use tokio::signal;

mod spotify;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    env::var("HOST").unwrap();

    let app = Router::new().route("/health", get(|| async { (StatusCode::OK, "OK") }));

    let addr = SocketAddr::from((
        [127, 0, 0, 1],
        env::var("PORT").unwrap().parse::<u16>().unwrap(),
    ));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind TCP listener");

    let server = axum::serve(listener, app.into_make_service()).with_graceful_shutdown(shutdown());
    if let Err(err) = server.await {
        eprintln!("Server error: {:?}", err)
    }
}

async fn shutdown() {
    signal::ctrl_c()
        .await
        .expect("Failed to listen for shutdown signal");

    print!("Shutdown signal received, shutting down...")
}
