use crate::settings::LogSettings;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter, Layer, Registry};

pub fn init_logging(log_settings: &LogSettings) -> Vec<WorkerGuard> {
    let env_filter = EnvFilter::new(log_settings.env_filter.as_str());
    let mut guards = vec![];
    let mut layers: Box<dyn Layer<Registry> + Sync + Send> = env_filter.boxed();
    if log_settings.enable_console {
        let console_layer = fmt::layer().with_writer(std::io::stdout).pretty();
        layers = layers.and_then(console_layer).boxed()
    }
    if log_settings.enable_file {
        let file_appender = RollingFileAppender::builder()
            .rotation(Rotation::HOURLY)
            .filename_prefix(log_settings.filename_prefix.as_str())
            .max_log_files(24 * 7)
            .build(log_settings.directory.as_str())
            .unwrap();
        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
        let file_layer = fmt::layer().with_writer(non_blocking).with_ansi(false);
        guards.push(guard);
        layers = layers.and_then(file_layer).boxed();
    }
    tracing_subscriber::registry().with(layers).init();
    guards
}
