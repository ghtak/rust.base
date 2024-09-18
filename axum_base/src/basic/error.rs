use axum::{
    extract::rejection::{FormRejection, JsonRejection, PathRejection},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use error_stack::Report;
use redis::RedisError;
use serde_json::json;
use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum AppError {
    #[error("unknown error {0}")]
    Unknown(String),

    #[error("bad request {0}")]
    BadRequest(String),

    #[error("illegal state {0}")]
    IllegalState(String),

    #[error("port error {0}")]
    PortError(String),

    #[error("init error {0}")]
    InitError(String),
    
    // #[error(transparent)]
    // SqlxError(#[from] sqlx::Error),
}

pub type AppResult<T> = error_stack::Result<T, AppError>;

pub struct AppReport(pub Report<AppError>);

pub type JsonResult<T> = Result<Json<T>, AppReport>;

impl IntoResponse for AppReport {
    fn into_response(self) -> axum::response::Response {
        let report = self.0;
        let (status_code, message) = match report.current_context() {
            AppError::Unknown(message) => (StatusCode::INTERNAL_SERVER_ERROR, message.to_string()),
            AppError::BadRequest(message) => (StatusCode::BAD_REQUEST, message.to_string()),
            AppError::IllegalState(message) => (StatusCode::BAD_REQUEST, message.to_string()),
            other => (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", other)),
        };
        (
            status_code,
            axum::Json(json!({
                "message": message,
                "detail": format!("{:?}", report)
            })),
        )
            .into_response()
    }
}

impl From<Report<AppError>> for AppReport {
    fn from(value: Report<AppError>) -> Self {
        AppReport(value)
    }
}

impl From<JsonRejection> for AppReport {
    fn from(value: JsonRejection) -> Self {
        AppReport(Report::new(AppError::BadRequest(format!("{value}"))))
    }
}

impl From<PathRejection> for AppReport {
    fn from(value: PathRejection) -> Self {
        AppReport(Report::new(AppError::BadRequest(format!("{value}"))))
    }
}

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

    #[error(transparent)]
    RedisError(#[from] RedisError),
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
