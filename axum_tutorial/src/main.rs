use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(axum_tutorial::index))
        .route("/new", get(axum_tutorial::new))
        .route("/deleteme", get(axum_tutorial::deleteme));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
