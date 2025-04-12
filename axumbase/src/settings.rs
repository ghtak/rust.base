use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: ServerSettings,
}

#[derive(Debug, Deserialize)]
pub struct ServerSettings {
    pub port: u16,
    pub host: String,
}

pub fn load_settings() -> Result<Settings, ConfigError> {
    let settings = Config::builder()
        .add_source(File::with_name("settings.toml"))
        .build()?;
    settings.try_deserialize()
}
