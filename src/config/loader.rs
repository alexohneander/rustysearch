use crate::types::config::{self, Config};

pub fn load_config(config_path: &str) -> Config {
    let settings: Config = config::Config::new();

    match config::Config::load_from_file(config_path) {
        Ok(config) => {
            return config;
        }
        Err(e) => {
            eprintln!("Error loading config file: {}", e);
        }
    }
    settings
}
