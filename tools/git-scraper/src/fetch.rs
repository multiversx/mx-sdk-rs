use reqwest::blocking::Client;
use serde_json::Value;

type InteractorContent = (Vec<(String, String)>, Option<String>);

pub(crate) fn fetch_directory_listing(client: &Client, url: &str) -> reqwest::Result<Value> {
    println!("Fetching directory listing from: {}", url);
    let response = client
        .get(url)
        .header("Accept", "application/vnd.github.v3+json")
        .send()?;

    println!("Got response with status: {}", response.status());

    if !response.status().is_success() {
        println!("Error response body: {}", response.text()?);
        panic!("Failed to fetch directory listing");
    }

    let json = response.json()?;
    println!("Successfully parsed JSON response");
    Ok(json)
}

pub(crate) fn fetch_file_content(
    client: &Client,
    folder_url: &str,
    file_name: &str,
) -> Option<String> {
    let folder_response: Value = client.get(folder_url).send().ok()?.json().ok()?;

    if let Some(entries) = folder_response.as_array() {
        for entry in entries {
            if let Some(name) = entry["name"].as_str() {
                if name == file_name {
                    if let Some(download_url) = entry["download_url"].as_str() {
                        return client.get(download_url).send().ok()?.text().ok();
                    }
                }
            }
        }
    }
    None
}

pub(crate) fn fetch_directory_contents(
    client: &Client,
    folder_url: &str,
    subfolder: &str,
) -> Option<Vec<(String, String)>> {
    println!(
        "Fetching contents from {} in subfolder {}",
        folder_url, subfolder
    );

    let folder_response: Value = client.get(folder_url).send().ok()?.json().ok()?;

    if let Some(entries) = folder_response.as_array() {
        for entry in entries {
            if let Some(name) = entry["name"].as_str() {
                if name == subfolder {
                    if let Some(url) = entry["url"].as_str() {
                        println!("Found directory: {}", name);
                        return fetch_files_from_directory(client, url);
                    }
                }
            }
        }
    }

    println!("Directory {} not found", subfolder);
    None
}

pub(crate) fn fetch_interactor_contents(
    client: &Client,
    folder_url: &str,
) -> Option<InteractorContent> {
    println!("Fetching interactor contents from {}", folder_url);

    let folder_response: Value = client.get(folder_url).send().ok()?.json().ok()?;

    if let Some(entries) = folder_response.as_array() {
        let mut src_contents = None;
        let mut cargo_contents = None;

        for entry in entries {
            if let Some(name) = entry["name"].as_str() {
                if name == "interactor" {
                    if let Some(url) = entry["url"].as_str() {
                        println!("Found interactor directory");
                        let interactor_response: Value =
                            client.get(url).send().ok()?.json().ok()?;

                        if let Some(interactor_entries) = interactor_response.as_array() {
                            for interactor_entry in interactor_entries {
                                match interactor_entry["name"].as_str() {
                                    Some("src") => {
                                        if let Some(src_url) = interactor_entry["url"].as_str() {
                                            src_contents =
                                                fetch_files_from_directory(client, src_url);
                                        }
                                    }
                                    Some("Cargo.toml") => {
                                        if let Some(download_url) =
                                            interactor_entry["download_url"].as_str()
                                        {
                                            if let Ok(content) =
                                                client.get(download_url).send().unwrap().text()
                                            {
                                                cargo_contents = Some(content);
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
        }

        return Some((src_contents.unwrap_or_default(), cargo_contents));
    }

    None
}

fn fetch_files_from_directory(client: &Client, url: &str) -> Option<Vec<(String, String)>> {
    println!("Fetching files from {}", url);
    let response: Value = client.get(url).send().ok()?.json().ok()?;

    if let Some(files) = response.as_array() {
        let mut results = Vec::new();
        for file_entry in files {
            if let (Some(file_name), Some(download_url)) = (
                file_entry["name"].as_str(),
                file_entry["download_url"].as_str(),
            ) {
                println!("Fetching file: {}", file_name);
                if let Ok(content) = client.get(download_url).send().unwrap().text() {
                    results.push((file_name.to_string(), content));
                }
            }
        }
        return Some(results);
    }
    None
}

pub(crate) fn fetch_meta_contents(client: &Client, folder_url: &str) -> Option<InteractorContent> {
    println!("Fetching meta contents from {}", folder_url);

    let folder_response: Value = client.get(folder_url).send().ok()?.json().ok()?;

    if let Some(entries) = folder_response.as_array() {
        let mut src_contents = None;
        let mut cargo_contents = None;

        for entry in entries {
            if let Some(name) = entry["name"].as_str() {
                if name == "meta" {
                    if let Some(url) = entry["url"].as_str() {
                        println!("Found meta directory");
                        let interactor_response: Value =
                            client.get(url).send().ok()?.json().ok()?;

                        if let Some(interactor_entries) = interactor_response.as_array() {
                            for interactor_entry in interactor_entries {
                                match interactor_entry["name"].as_str() {
                                    Some("src") => {
                                        if let Some(src_url) = interactor_entry["url"].as_str() {
                                            src_contents =
                                                fetch_files_from_directory(client, src_url);
                                        }
                                    }
                                    Some("Cargo.toml") => {
                                        if let Some(download_url) =
                                            interactor_entry["download_url"].as_str()
                                        {
                                            if let Ok(content) =
                                                client.get(download_url).send().unwrap().text()
                                            {
                                                cargo_contents = Some(content);
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
        }

        return Some((src_contents.unwrap_or_default(), cargo_contents));
    }

    None
}
