mod basic_sample;
mod basic_utoipa;

use crate::{app_context::{self, AppContext}, settings::OpenApiSettings};
use axum::Router;
use basic_sample::sample_router;
use basic_utoipa::{utoipa_router, ApiDoc};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;


pub fn routes(app_context: AppContext) -> Router {
    let router = Router::new()
        .merge(sample_router(app_context.clone()))
        .merge(utoipa_router(app_context.clone()));
    if app_context.settings.openapi.enable {
        router.merge(
            SwaggerUi::new(app_context.settings.openapi.url.clone())
                .url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
    } else {
        router
    }
}
