mod app_error;
mod extract_ext;
mod settings;
mod logging;

use crate::settings::load_settings;
use axum::routing::get;
use axum::Router;
use tracing::subscriber;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter, Layer, Registry};
use crate::logging::init_logging;

#[tokio::main]
async fn main() {
    let settings = load_settings().unwrap();
    let _guard = init_logging(&settings.log);
    let route = Router::new().route(
        "/",
        get(|| async {
            tracing::info!("Route GET");
            "Hello, Axum!"
        }),
    );
    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", settings.server.host, settings.server.port))
            .await
            .unwrap();
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
