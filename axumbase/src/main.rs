mod app_context;
mod app_error;
mod extract_ext;
mod logging;
mod routes;
mod settings;

use app_context::AppContext;
use routes::routes;
use settings::load_settings;

use crate::logging::init_logging;

#[tokio::main]
async fn main() {
    let app_context = AppContext::new(load_settings().unwrap());
    let _guards = init_logging(&app_context.settings.log);
    let listener = tokio::net::TcpListener::bind(app_context.settings.server.address().as_str())
        .await
        .unwrap();
    axum::serve(listener, routes(app_context))
        .await
        .unwrap();
}
