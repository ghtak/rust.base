mod settings;

use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let route = Router::new().route("/", get(|| async { "Hello, Axum!" }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, route).await.unwrap();
}

#[cfg(test)]
mod tests {
    use crate::settings::load_settings;

    #[test]
    fn it_works() {
        let setting = load_settings().unwrap();
        assert_eq!(setting.server.port, 3009);
        assert_eq!(setting.server.host, "0.0.0.0");
    }
}
