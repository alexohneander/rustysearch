use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    http_addr: String,
    database_path: String,
}

impl Config {
    pub fn new() -> Config {
        Config {
            http_addr: String::from("127.0.0.1:4000"),
            database_path: String::from("/tmp/rustysearch.db"),
        }
    }
}
