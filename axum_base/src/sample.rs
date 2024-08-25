use axum::{response::IntoResponse, routing::{get, post}, Router};
use serde::{Deserialize, Serialize};

use crate::basic::{self, extract::{Json, Path}};

pub fn router() -> Router<basic::state::State> {
    Router::new().route("/sample/", post(sample_post))
    .route("/sample/:user_id/teams/:team_id", get(sample_path))
}

#[derive(Debug, Serialize, Deserialize)]
struct Sample {
    name: String,
    detail: String,
}

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