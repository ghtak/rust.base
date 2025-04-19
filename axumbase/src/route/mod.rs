mod basic_sample;
mod basic_utoipa;

use crate::settings::OpenApiSettings;
use axum::Router;
use basic_sample::sample_router;
use basic_utoipa::{utoipa_router, ApiDoc};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;


pub fn router(settings: &OpenApiSettings) -> Router {
    let router = Router::new().merge(sample_router()).merge(utoipa_router());
    if settings.enable {
        router.merge(
            SwaggerUi::new(settings.url.clone())
                .url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
    } else {
        router
    }
}
