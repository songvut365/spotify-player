use axum::{
    http::{HeaderValue, StatusCode},
    routing::get,
    Router,
};
use dotenv::dotenv;
use reqwest::Method;
use std::{env, net::SocketAddr};
use tokio::{net, signal};
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, DefaultOnBodyChunk},
};
use tower_http::{
    trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::Level;

mod errors;
mod spotify;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let cors = CorsLayer::new()
        .allow_origin(
            env::var("HOST")
                .unwrap_or("localhost:8080".to_string())
                .parse::<HeaderValue>()
                .unwrap(),
        )
        .allow_methods([Method::GET, Method::POST]);

    let trace = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().include_headers(true))
        .on_body_chunk(DefaultOnBodyChunk::new())
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(
            DefaultOnResponse::new()
                .level(Level::INFO)
                .latency_unit(LatencyUnit::Seconds),
        );

    let spotify_router = spotify::new_handler();

    let app = Router::new()
        .nest("/api/v1/spotify", spotify_router)
        .layer(trace)
        .route("/health", get(|| async { (StatusCode::OK, "OK") }))
        .layer(cors);

    let addr = SocketAddr::from((
        [127, 0, 0, 1],
        env::var("PORT")
            .unwrap_or("8080".to_string())
            .parse::<u16>()
            .unwrap(),
    ));

    let listener = net::TcpListener::bind(addr)
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
