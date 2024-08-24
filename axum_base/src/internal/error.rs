
use thiserror::Error;

#[derive(Error,Debug)]
pub enum Error{
    #[error("unknown error {0}")]
    Unknown(String),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error)
}