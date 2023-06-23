use axum::{Router, Server};

#[tokio::main] // (1)
async fn main() {
    let app = Router::new(); // (2)
    let address = "0.0.0.0:8000".parse().unwrap();

    Server::bind(&address) // (3)
        .serve(app.into_make_service())
        .await
        .unwrap();
}