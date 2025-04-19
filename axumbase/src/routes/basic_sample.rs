use crate::app_context::AppContext;
use crate::extract_ext::{Json, Path};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;
use serde::{Deserialize, Serialize};

pub fn sample_router(state: AppContext) -> Router {
    Router::new()
        .route(
            "/",
            get(|| async {
                tracing::info!("get record");
                Json(Record {
                    key: "key".into(),
                    value: "value".into(),
                })
            }),
        )
        .route("/record", post(post_record))
        .route("/record/{key}/{value}", get(get_path))
        .with_state(state)
}

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    key: String,
    value: String,
}

async fn post_record(Json(record): Json<Record>) -> impl IntoResponse {
    Json(record)
}

async fn get_path(Path((key, value)): Path<(String, String)>) -> impl IntoResponse {
    Json(Record { key, value })
}
