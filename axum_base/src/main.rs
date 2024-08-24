use axum::{routing::get, Router};

mod internal;

#[tokio::main]
async fn main() -> internal::Result<()> {
    internal::diag::init_tracing()?;
    tracing::info!("init");
    let router = Router::new().route("/", get(|| async move { "Hello Axum" }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:18080")
        .await
        .unwrap();
    axum::serve(listener, router).await.unwrap();
    Ok(())
}
