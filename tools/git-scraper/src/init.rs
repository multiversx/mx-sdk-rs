use reqwest::blocking::Client;
use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::io::{self, BufWriter};

#[derive(Deserialize)]
struct Config {
    github_token: String,
}

const CONFIG_PATH: &str = "tools/git-scraper/config.toml";

pub(crate) fn create_client() -> Client {
    let config_content = fs::read_to_string(CONFIG_PATH).expect("Failed to read config.toml");

    let config: Config = toml::from_str(&config_content).expect("Failed to parse config.toml");

    println!(
        "Creating client with token: {}...",
        &config.github_token[..10]
    ); // Print first 10 chars of token

    Client::builder()
        .user_agent("Rust GitHub Scraper")
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", config.github_token).parse().unwrap(),
            );
            headers.insert(
                reqwest::header::ACCEPT,
                "application/vnd.github.v3+json".parse().unwrap(),
            );
            headers
        })
        .build()
        .expect("Failed to create HTTP client")
}

pub(crate) fn initialize_writer(file_path: &str) -> io::Result<BufWriter<File>> {
    let output_file = File::create(file_path)?;
    Ok(BufWriter::new(output_file))
}
