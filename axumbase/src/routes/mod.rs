mod basic_sample;
mod basic_utoipa;
mod basic_database;

use crate::app_context::AppContext;
use axum::Router;
use basic_database::{database_router, DatabaseApiDoc};
use basic_sample::sample_router;
use basic_utoipa::{utoipa_router, ApiDoc};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;


pub fn routes(app_context: AppContext) -> Router {
    let router = Router::new()
        .merge(sample_router(app_context.clone()))
        .merge(database_router(app_context.clone()))
        .merge(utoipa_router(app_context.clone()));
    if app_context.settings.openapi.enable {
        let mut openapi = ApiDoc::openapi();
        openapi.merge(DatabaseApiDoc::openapi());
        router.merge(
            SwaggerUi::new(app_context.settings.openapi.url.clone())
                .url("/api-docs/openapi.json", openapi),
        )
    } else {
        router
    }
}
