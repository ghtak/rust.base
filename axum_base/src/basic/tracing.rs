use std::str::FromStr;

use tracing::Level;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt::writer::MakeWriterExt;

use crate::basic::env::Env;

pub fn init(env: &Env) {
    let max_level = tracing::Level::from_str(&env.tracing.max_level).unwrap_or(Level::TRACE);
    let subscriber = tracing_subscriber::fmt()
        //.json()
        .with_ansi(false)
        .with_max_level(max_level);

    if env.tracing.with_file {
        let debug_file = RollingFileAppender::builder()
            .rotation(Rotation::HOURLY)
            .filename_prefix("debug")
            .max_log_files(24 * 7)
            .build(env.tracing.directory.as_str())
            .unwrap();

        let warn_file = RollingFileAppender::builder()
            .rotation(Rotation::DAILY)
            .filename_prefix("warn")
            .max_log_files(30)
            .build(env.tracing.directory.as_str())
            .unwrap()
            .with_max_level(tracing::Level::WARN);

        subscriber
            .with_writer(std::io::stderr.and(warn_file.and(debug_file)))
            .init()
    } else {
        subscriber.with_writer(std::io::stdout).init()
    }
}
