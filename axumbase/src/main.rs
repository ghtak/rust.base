use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let route = Router::new().route("/", get(|| async { "Hello, Axmu!" }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, route).await.unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
