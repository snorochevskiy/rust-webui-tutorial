use std::net::SocketAddr;

use axum::{extract::Path, routing::{get}, Router};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let app = Router::new()
        .route("/api/hello/{name}", get(greet))
        .layer(CorsLayer::permissive());

    axum::serve(TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

async fn greet(Path(name): Path<String>) -> String {
    format!("Hello, {name}!")
}
