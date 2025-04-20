use config::{Config, ConfigError, File};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Settings {
    pub server: ServerSettings,
    pub log: LogSettings,
    pub openapi: OpenApiSettings,
    pub database: DatabaseSettings,
    pub redis: RedisSettings,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerSettings {
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OpenApiSettings {
    pub enable: bool,
    pub url: String,
}

impl ServerSettings {
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LogSettings {
    pub env_filter: String,
    pub directory: String,
    pub filename_prefix: String,
    pub enable_file: bool,
    pub enable_console: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseSettings {
    pub url: String,
    pub max_conn: usize,
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RedisSettings {
    pub url: String,
    pub max_conn: usize,
    pub timeout_millis: usize,
}

pub fn load_settings() -> Result<Settings, ConfigError> {
    let settings = Config::builder()
        .add_source(File::with_name("settings.toml"))
        .build()?;
    settings.try_deserialize()
}

#[cfg(test)]
mod tests {
    use crate::settings::load_settings;

    #[test]
    fn it_works() {
        let setting = load_settings().unwrap();
        assert_eq!(setting.server.port, 3009);
        assert_eq!(setting.server.host, "0.0.0.0");
        print!("{:?}", setting);
    }
}
