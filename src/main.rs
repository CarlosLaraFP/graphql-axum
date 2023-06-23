use axum::{Router, Server, routing::get};
mod routes;
use crate::routes::health;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health", get(health));

    let address = "0.0.0.0:8000".parse().unwrap();

    Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}