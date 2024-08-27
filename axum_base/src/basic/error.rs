use crate::basic::extract::Json;
use axum::{
    extract::rejection::{FormRejection, JsonRejection, PathRejection},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum Error {
    #[error("unknown error {0}")]
    Message(String),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),

    #[error(transparent)]
    JsonRejection(#[from] JsonRejection),

    #[error(transparent)]
    PathRejection(#[from] PathRejection),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    // #[error(transparent)]
    // ValidationError(#[from] validator::ValidationErrors),
    #[error(transparent)]
    AxumFormRejection(#[from] FormRejection),

    #[error("app error status: {0} message: {1}")]
    AppError(StatusCode, String),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            Error::JsonRejection(rejection) => (rejection.status(), rejection.body_text()),
            Error::PathRejection(rejection) => (rejection.status(), rejection.body_text()),
            Error::AppError(s, m) => (s, m),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", self)),
        };
        (
            status,
            Json(json!({
                "error": json!({
                    "message": message
                })
            })),
        )
            .into_response()
    }
}
