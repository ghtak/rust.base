use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: ServerSettings,
    pub log: LogSettings,
}

#[derive(Debug, Deserialize)]
pub struct ServerSettings {
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Deserialize)]
pub struct LogSettings {
    pub env_filter: String,
    pub directory: String,
    pub filename_prefix: String,
    pub enable_file: bool,
    pub enable_console: bool,
}

pub fn load_settings() -> Result<Settings, ConfigError> {
    let settings = Config::builder()
        .add_source(File::with_name("settings.toml"))
        .build()?;
    settings.try_deserialize()
}
