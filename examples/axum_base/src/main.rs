use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
mod error;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(|| async { "Hello Axum" }))
        .route("/", post(say_hello));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn say_hello(Json(sample): Json<HelloReq>) -> (StatusCode, Json<HelloRes>) {
    (
        StatusCode::OK,
        Json(HelloRes {
            message: format!("Hello {}", sample.name),
        }),
    )
}

#[derive(Deserialize)]
struct HelloReq {
    name: String,
}

#[derive(Serialize)]
struct HelloRes {
    message: String,
}
