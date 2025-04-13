mod app_error;
mod extract_ext;
mod logging;
mod route_sample;
mod settings;
mod utoipa_sample;

use crate::logging::init_logging;
use crate::route_sample::sample_router;
use crate::settings::load_settings;
use crate::utoipa_sample::{utoipa_router, ApiDoc};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;


#[tokio::main]
async fn main() {
    let settings = load_settings().unwrap();
    let _guards = init_logging(&settings.log);
    let listener = tokio::net::TcpListener::bind(settings.server.address().as_str())
        .await
        .unwrap();
    let router = sample_router().merge(utoipa_router())
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()));
    axum::serve(listener, router)
        .await
        .unwrap();
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
