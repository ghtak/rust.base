use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct Env {
    pub profile: String,
    pub server: Server,
    pub tracing: Tracing,
}

impl Env {
    pub fn new(tomfilename: &str) -> Self {
        let contents = fs::read_to_string(tomfilename).unwrap();
        return toml::from_str(&contents).unwrap();
    }
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Server {
    pub address: String
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Tracing {
    pub max_level: String,
    pub directory: String,
    pub with_file: bool,
}