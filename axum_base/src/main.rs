use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use basic::extract::Json;
use serde::{Deserialize, Serialize};
use tokio::signal;

mod basic;

mod sample;

#[tokio::main]
async fn main() {
    let env = basic::env::Env::new("dev.toml");
    basic::tracing::init(&env);
    let state = basic::state::State {};
    tokio::spawn(async move {
        tracing::info!("init with env {env:?}");
        let router = Router::new()
            .route("/", get(|| async move { "Hello Axum" }))
            .merge(sample::router())
            .with_state(state);

        let listener = tokio::net::TcpListener::bind(env.server.address.as_str())
            .await
            .expect("listen fail");
        axum::serve(listener, router).await.expect("serve fail");
    });
    signal::ctrl_c().await.expect("signal fail");
}

