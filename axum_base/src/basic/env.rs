use std::fs;

use serde::{Deserialize, Serialize};
use tracing::Value;

#[derive(Deserialize, Debug, Serialize, Default, Clone)]
pub struct Env {
    pub profile: String,
    pub server: Server,
    pub tracing: Tracing,
    pub database: Database
}

impl Env {
    pub fn new(tomfilename: &str) -> Self {
        let contents = fs::read_to_string(tomfilename).unwrap();
        return toml::from_str(&contents).unwrap();
    }
}

#[derive(Deserialize, Debug, Serialize, Default, Clone)]
pub struct Server {
    pub address: String
}

#[derive(Deserialize, Debug, Serialize, Default, Clone)]
pub struct Tracing {
    pub max_level: String,
    pub directory: String,
    pub with_file: bool,
}


#[derive(Deserialize, Debug, Serialize, Default, Clone)]
pub struct DatabaseConnection{
    pub url: String,
    pub max_connections: u32,
}


#[derive(Deserialize, Debug, Serialize, Default, Clone)]
pub struct Database{
    pub sources: Vec<DatabaseConnection>,
    pub replicas: Vec<DatabaseConnection>,
}