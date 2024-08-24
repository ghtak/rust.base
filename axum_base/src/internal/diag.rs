use crate::internal::{Error, Result};
use tracing_subscriber::layer::SubscriberExt;

pub fn init_tracing() -> Result<()> {
    let layer = tracing_subscriber::fmt::layer()
        .json()
        .with_file(false)
        .with_line_number(false)
        .with_ansi(false)
        .with_target(true);
    tracing::subscriber::set_global_default(tracing_subscriber::registry().with(layer))
        .map_err(|e| Error::Unknown(e.to_string()))?;
    Ok(())
}
