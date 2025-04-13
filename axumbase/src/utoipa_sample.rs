use crate::extract_ext::Json;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::Router;
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

#[derive(OpenApi)]
#[openapi(
    tags(
        (name="Sample", description="Sample API"),
    ),
    paths(
        sample_post
    ),
    components(schemas(Sample))
)]
pub struct ApiDoc;


pub fn utoipa_router() -> Router {
    Router::new().route("/sample", post(sample_post))
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct Sample {
    name: String,
    detail: String,
}

#[utoipa::path(
    post,
    path="/sample",
    request_body=Sample,
)]
async fn sample_post(Json(sample): Json<Sample>) -> impl IntoResponse {
    Json(sample)
}
