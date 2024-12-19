use std::fmt::Error;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub http_addr: String,
    pub database_path: String,
}

impl Config {
    pub fn new() -> Config {
        Config {
            http_addr: String::from("127.0.0.1:4000"),
            database_path: String::from("/tmp/rustysearch.db"),
        }
    }

    pub fn load_from_file(_config_path: &str) -> Result<Config, Error> {
        todo!();
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
