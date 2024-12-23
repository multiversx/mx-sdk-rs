use crate::fetch::{fetch_directory_contents, fetch_file_content};
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
            writer.flush()?;
        }
    } else {
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
    if let Some(interactor_files) = fetch_directory_contents(client, folder_url, "interactor/src") {
        writeln!(writer, "\nINTERACTOR FOLDER:")?;
        for (file_name, file_content) in interactor_files {
            writeln!(writer, "FILE_NAME: {}", file_name)?;
            writeln!(writer, "{}", file_content)?;
            writer.flush()?;
        }
    } else {
        writeln!(writer, "\nINTERACTOR FOLDER: None")?;
        println!("No interactor/src folder found for {}", folder_name);
    }

    if let Some(interactor_cargo_content) = fetch_file_content(client, folder_url, "interactor/Cargo.toml") {
        writeln!(writer, "\nINTERACTOR CARGO.TOML:")?;
        writeln!(writer, "{}", interactor_cargo_content)?;
    } else {
        println!("No interactor Cargo.toml found for {}", folder_name);
    }
    writer.flush()?;
    Ok(())
}
