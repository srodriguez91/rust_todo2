mod models;
use axum::{Router, routing::post};
use models::Task;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", post(Task::create_task));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
