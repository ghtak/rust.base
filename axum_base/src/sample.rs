use axum::{response::IntoResponse, routing::{get, post}, Router};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

use crate::basic::{self, extract::{Json, Path}};

#[derive(OpenApi)]
#[openapi(paths(sample_post),components(schemas(Sample)))]
pub(super) struct Api;

pub fn router() -> Router<basic::state::State> {
    Router::new().route("/sample/", post(sample_post))
    .route("/sample/:user_id/teams/:team_id", get(sample_path))
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct Sample {
    name: String,
    detail: String,
}


#[utoipa::path(
    post,
    path="/sample/",
    request_body=Sample,
)]
async fn sample_post(Json(sample): Json<Sample>) -> impl IntoResponse {
    Json(sample)
}

#[derive(Debug, Deserialize, Serialize)]
struct SampleParams {
    user_id: u32,
    team_id: u32,
}

async fn sample_path(Path(sp): Path<SampleParams>) -> impl IntoResponse {
    axum::Json(sp)
}