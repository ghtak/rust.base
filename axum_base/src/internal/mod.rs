mod error;
pub mod diag;

pub type Error = error::Error;
pub type Result<T> = core::result::Result<T, Error>;