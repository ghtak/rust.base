use thiserror::Error;

#[derive(Error, Debug)]
pub enum HiError {
    #[error("hierror io error {0:?}")]
    IOError(#[from] std::io::Error),
}

pub fn handle_io() -> Result<(), std::io::Error> {
    Err(std::io::Error::new(std::io::ErrorKind::Unsupported, "not supported"))
}

pub fn work_with_io() -> Result<(), HiError> {
    handle_io()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hi_error_from_std_io_error() {
        match work_with_io() {
            Err(e) => println!("{}", e),
            _ => return
        }
    }
}