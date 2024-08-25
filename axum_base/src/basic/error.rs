use crate::basic::extract::Json;
use axum::{
    extract::rejection::{JsonRejection, PathRejection},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Serialize;
use serde_json::json;
use thiserror::Error;

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

    #[error("app error status: {0} error_code: {1} message: {2}")]
    AppError(StatusCode, String, String),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (status, code, message) = match self {
            Error::JsonRejection(rejection) => {
                (rejection.status(), "".to_owned(), rejection.body_text())
            }
            Error::PathRejection(rejection) => {
                (rejection.status(), "".to_owned(), rejection.body_text())
            }
            Error::AppError(s, e, m) => (s, e, m),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "9999".to_owned(),
                format!("{:?}", self),
            ),
        };
        (
            status,
            Json(json!({
                "error": json!({
                    "code": code,
                    "message": message
                })
            })),
        )
            .into_response()
    }
}
