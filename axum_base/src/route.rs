use axum::{http::StatusCode, response::IntoResponse, Router};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use utoipa::
    OpenApi
;

use crate::{basic::{error::Error, state::State}, sample};
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(info(description = "Basic Api description"))]
struct BasicApiDoc;

pub fn router(state: State) -> Router {
    let mut api_doc = BasicApiDoc::openapi();
    api_doc.merge(sample::Api::openapi());

    let router = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api_doc))
        .merge(sample::router())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .fallback(handle_404)
        .with_state(state);

    router
}

async fn handle_404() -> Error {
    Error::AppError(
        StatusCode::NOT_FOUND, 
        "404".to_owned(), 
        "nothing to see here".to_owned())
}