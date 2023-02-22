use std::io::Read;

use serde::Deserialize;

const CONFIG_FILE_NAME: &str = "config.toml";

#[derive(Debug, Deserialize)]
pub struct Config {
    gateway: String,
    pem: String,
}

impl Config {
    pub fn load_config() -> Self {
        let mut file = std::fs::File::open(CONFIG_FILE_NAME).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        toml::from_str(&content).unwrap()
    }

    pub fn gateway(&self) -> &str {
        &self.gateway
    }

    pub fn pem(&self) -> &str {
        &self.pem
    }
}
