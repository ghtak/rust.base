mod app_error;
mod extract_ext;
mod logging;
mod settings;
mod route;

use route::router;

use crate::logging::init_logging;
use crate::settings::load_settings;

#[tokio::main]
async fn main() {
    let settings = load_settings().unwrap();
    let _guards = init_logging(&settings.log);
    let listener = tokio::net::TcpListener::bind(settings.server.address().as_str())
        .await
        .unwrap();
    axum::serve(listener, router(&settings.openapi))
        .await
        .unwrap();
}