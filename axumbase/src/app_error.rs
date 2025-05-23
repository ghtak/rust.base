use axum::http::StatusCode;
use axum::response::IntoResponse;
use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),

    #[error("invalid parameter {0}")]
    InvalidParameter(String),

    #[error("illegal state {0}")]
    IllegalState(String),

    #[error("custom error status: {0} message: {1}")]
    Custom(StatusCode, String),

    #[error("validation error: {0}")]
    ValidationError(anyhow::Error),

    #[error("path error: {0}")]
    PathError(anyhow::Error),
    
    #[error("database error: {0}")]
    DatabaseError(anyhow::Error),

    #[error("redis error: {0}")]
    RedisError(anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}

#[cfg(test)]
mod tests {
    use crate::app_error::AppError;

    fn return_anyhow_error() -> anyhow::Error {
        anyhow::anyhow!("foo").into()
    }

    fn app_error() -> Result<(), AppError> {
        Err(return_anyhow_error().context("app_error").into())
    }

    #[test]
    fn test_anyhow_context() {
        println!("{:?}", app_error());
    }
}
