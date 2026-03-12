use axum::{routing::get, Router};

mod config;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health", get(routes::health));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind TCP listener");

    println!("service running on http://0.0.0.0:3000");

    axum::serve(listener, app)
        .await
        .expect("failed to start HTTP server");
}
