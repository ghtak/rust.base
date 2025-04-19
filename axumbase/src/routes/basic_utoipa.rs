use crate::{app_context::AppContext, extract_ext::Json};
use axum::routing::{get, post};
use axum::Router;
use axum::{extract::State, response::IntoResponse};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

#[derive(OpenApi)]
#[openapi(
    tags(
        (name="Sample", description="Sample API"),
    ),
    paths(sample_get, sample_post)
)]
pub struct ApiDoc;

pub fn utoipa_router<S>(state: AppContext) -> Router<S> {
    Router::new()
        .route("/sample", get(sample_get).post(sample_post))
        .with_state(state)
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct Sample {
    name: String,
    detail: String,
}

#[utoipa::path(post, path="/sample", request_body=Sample)]
async fn sample_post(Json(sample): Json<Sample>) -> impl IntoResponse {
    Json(sample)
}

#[utoipa::path(get, path = "/sample")]
async fn sample_get(State(ctx): State<AppContext>) -> impl IntoResponse {
    Json((*ctx.settings).clone())
}
