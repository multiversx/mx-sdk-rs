use fetch::fetch_directory_listing;
use init::{create_client, initialize_writer};
use reqwest::blocking::Client;
use serde_json::Value;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use write::{write_cargo_toml, write_interactor_files, write_readme, write_src_folder};

mod fetch;
mod init;
mod write;

const GITHUB_API_URL: &str =
    "https://api.github.com/repos/multiversx/mx-sdk-rs/contents/contracts/examples";
const FILE_PATH: &str = "tools/git-scraper/contracts_dump.txt";

fn main() -> io::Result<()> {
    let client = create_client();
    let mut writer = initialize_writer(FILE_PATH)?;

    let response = fetch_directory_listing(&client, GITHUB_API_URL).unwrap();
    if let Some(entries) = response.as_array() {
        for entry in entries {
            process_entry(&client, entry, &mut writer)?;
        }
    }

    writeln!(writer, "////////////////////////")?;
    writer.flush()?;
    println!("Contracts processed and saved to contracts_dump.txt");
    Ok(())
}

fn process_entry(client: &Client, entry: &Value, writer: &mut BufWriter<File>) -> io::Result<()> {
    if let Some(folder_name) = entry["name"].as_str() {
        println!("Starting to process entry: {}", folder_name);
        
        if let Some(folder_url) = entry["url"].as_str() {
            println!("Found URL: {}", folder_url);
            
            writeln!(writer, "////////////////////////")?;
            writeln!(writer, "NAME: {}", folder_name)?;
            println!("Processing contract {}", folder_name);

            println!("Fetching README...");
            write_readme(client, folder_url, writer, folder_name)?;
            
            println!("Fetching src folder...");
            write_src_folder(client, folder_url, writer, folder_name)?;
            
            println!("Fetching Cargo.toml...");
            write_cargo_toml(client, folder_url, writer, folder_name)?;
            
            println!("Fetching interactor files...");
            write_interactor_files(client, folder_url, writer, folder_name)?;
            
            writer.flush()?;
            println!("Finished processing {}", folder_name);
        }
    }
    Ok(())
}
