pub mod db;
pub mod env;
pub mod error;
pub mod extract;
pub mod roundrobin;
pub mod tracing;
pub mod redis;
pub mod depends;

#[allow(dead_code)]
pub type Result<T> = core::result::Result<T, error::Error>;

use tokio::signal;

pub async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
