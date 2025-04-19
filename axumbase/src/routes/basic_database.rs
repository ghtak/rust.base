use crate::database::DatabasePool;
use crate::{app_context::AppContext, extract_ext::Json};
use axum::routing::get;
use axum::Router;
use axum::{extract::State, response::IntoResponse};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

#[derive(OpenApi)]
#[openapi(
    tags(
        (name="Database", description="Database API"),
    ),
    paths(database_post, database_get)
)]
pub struct DatabaseApiDoc;

pub fn database_router<S>(state: AppContext) -> Router<S> {
    Router::new()
        .route("/database", get(database_get).post(database_post))
        .with_state(state)
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct Sample {
    name: String,
    detail: String,
}

#[utoipa::path(post, path="/database", request_body=Sample)]
async fn database_post(Json(sample): Json<Sample>) -> impl IntoResponse {
    Json(sample)
}

#[utoipa::path(get, path = "/database")]
async fn database_get(State(pool): State<DatabasePool>) -> impl IntoResponse {
    println!("{:?}", pool);
}
