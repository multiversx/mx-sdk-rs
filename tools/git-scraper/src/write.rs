use crate::fetch::{fetch_directory_contents, fetch_file_content, fetch_interactor_contents};
use reqwest::blocking::Client;
use std::fs::File;
use std::io::{self, BufWriter, Write};

pub(crate) fn write_readme(
    client: &Client,
    folder_url: &str,
    writer: &mut BufWriter<File>,
    folder_name: &str,
) -> io::Result<()> {
    if let Some(readme_content) = fetch_file_content(client, folder_url, "README.md") {
        writeln!(writer, "\nDESCRIPTION:\n{}", readme_content)?;
    } else {
        writeln!(writer, "\nDESCRIPTION:\nNone")?;
        println!("No README.md available for {}", folder_name);
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn write_src_folder(
    client: &Client,
    folder_url: &str,
    writer: &mut BufWriter<File>,
    folder_name: &str,
) -> io::Result<()> {
    writeln!(writer, "\nSRC FOLDER:")?;
    if let Some(src_files) = fetch_directory_contents(client, folder_url, "src") {
        for (file_name, file_content) in src_files {
            writeln!(writer, "FILE_NAME: {}", file_name)?;
            writeln!(writer, "{}", file_content)?;
        }
    } else {
        writeln!(writer, "No src folder found")?;
        println!("No src folder found for {}", folder_name);
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn write_cargo_toml(
    client: &Client,
    folder_url: &str,
    writer: &mut BufWriter<File>,
    folder_name: &str,
) -> io::Result<()> {
    if let Some(cargo_content) = fetch_file_content(client, folder_url, "Cargo.toml") {
        writeln!(writer, "\nCARGO.TOML:")?;
        writeln!(writer, "{}", cargo_content)?;
    } else {
        println!("No Cargo.toml found for {}", folder_name);
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn write_interactor_files(
    client: &Client,
    folder_url: &str,
    writer: &mut BufWriter<File>,
    folder_name: &str,
) -> io::Result<()> {
    if let Some((src_files, cargo_content)) = fetch_interactor_contents(client, folder_url) {
        writeln!(writer, "\nINTERACTOR FOLDER:")?;

        if !src_files.is_empty() {
            for (file_name, file_content) in src_files {
                writeln!(writer, "FILE_NAME: {}", file_name)?;
                writeln!(writer, "{}", file_content)?;
            }
        }

        if let Some(cargo_content) = cargo_content {
            writeln!(writer, "\nINTERACTOR CARGO.TOML:")?;
            writeln!(writer, "{}", cargo_content)?;
        }
    } else {
        println!("No interactor folder found for {}", folder_name);
    }
    writer.flush()?;
    Ok(())
}
