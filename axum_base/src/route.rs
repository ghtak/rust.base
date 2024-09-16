use axum::{http::StatusCode, response::Html, routing::get, Router};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use utoipa::OpenApi;

use crate::{app_state::AppState, basic::error::Error, oauth2sample, sample};
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(info(description = "Api description"))]
struct ApiDoc;

pub fn router(state: AppState) -> Router {
    let mut api_doc = ApiDoc::openapi();
    api_doc.merge(sample::Api::openapi());
    api_doc.merge(oauth2sample::Api::openapi());
    Router::new()
        .route("/helloworld", get(hello_world))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api_doc))
        .merge(sample::router(state.clone()))
        .merge(oauth2sample::router(state.clone()))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .fallback(handle_404)
}

async fn handle_404() -> Error {
    Error::AppError(StatusCode::NOT_FOUND, "nothing to see here".to_owned())
}

async fn hello_world() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
