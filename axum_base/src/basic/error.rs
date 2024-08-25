use crate::basic::extract::Json;
use axum::{extract::rejection::{JsonRejection, PathRejection}, http::StatusCode, response::IntoResponse};
use serde::Serialize;
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
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        #[derive(Serialize)]
        struct Detail {
            code: String,
            message: String,
        }
        #[derive(Serialize)]
        struct ErrorResp {
            error: Detail,
        }
        let (status, detail) = match self {
            Error::JsonRejection(rejection) => (
                rejection.status(),
                Detail {
                    code: "".to_owned(),
                    message: rejection.body_text(),
                },
            ),
            Error::PathRejection(rejection) => (
                rejection.status(),
                Detail {
                    code: "".to_owned(),
                    message: rejection.body_text(),
                },
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Detail {
                    code: "9999".to_owned(),
                    message: format!("{:?}", self),
                },
            ),
        };
        (status, Json(ErrorResp { error: detail })).into_response()
    }
}
