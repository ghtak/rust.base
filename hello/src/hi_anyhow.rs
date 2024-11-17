use anyhow::{ensure, Context, Result};
use crate::hi_thiserror::{work_with_io, HiError};

fn inner() -> Result<()> {
    ensure!(false, "foo");
    Ok(())
}

fn foo() -> Result<()> {
    inner().context("fail inner").context("add context")?;
    Ok(())
}

fn inner_hierror() -> Result<(), HiError> {
    work_with_io()?;
    Ok(())
}

fn bar() -> Result<()> {
    inner_hierror().context("fail inner_hierror")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anyhow_context() {
        match foo() {
            Err(e) => println!("{e:?}"),
            _ => assert!(true),
        };
        match bar() {
            Err(e) => println!("{e:?}"),
            _ => assert!(true),
        }
    }

    #[test]
    fn test_anyhow_from_hierror() {
        match bar() {
            Err(e) => println!("{e:?}"),
            _ => assert!(true),
        }
    }
}