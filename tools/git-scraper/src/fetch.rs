use reqwest::blocking::Client;
use serde_json::Value;

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
    let folder_response: Value = client.get(folder_url).send().ok()?.json().ok()?;

    if let Some(entries) = folder_response.as_array() {
        for entry in entries {
            if let Some(name) = entry["name"].as_str() {
                if name == subfolder {
                    if let Some(subfolder_url) = entry["url"].as_str() {
                        let subfolder_response: Value =
                            client.get(subfolder_url).send().ok()?.json().ok()?;
                        if let Some(files) = subfolder_response.as_array() {
                            let mut results = Vec::new();
                            for file_entry in files {
                                if let (Some(file_name), Some(download_url)) = (
                                    file_entry["name"].as_str(),
                                    file_entry["download_url"].as_str(),
                                ) {
                                    if let Ok(content) =
                                        client.get(download_url).send().unwrap().text()
                                    {
                                        results.push((file_name.to_string(), content));
                                    }
                                }
                            }
                            return Some(results);
                        }
                    }
                }
            }
        }
    }
    None
}
