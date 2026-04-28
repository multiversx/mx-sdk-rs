use base64::{Engine, engine::general_purpose::STANDARD as BASE64};
use multiversx_sc_snippets::imports::Bech32Address;
use serde::Deserialize;
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::cli::DownloadArgs;

use super::source_json_model::{PackedSource, SourceFileEntry};

// ---------------------------------------------------------------------------
// Response wrapper — only new types needed; inner source reuses PackedSource.
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ContractVerifierResponse {
    pub code_hash: Option<String>,
    pub source: Option<ContractVerifierSource>,
    pub status: String,
    pub ipfs_file_hash: Option<String>,
    pub docker_image: Option<String>,
}

#[derive(Deserialize)]
struct ContractVerifierSource {
    pub abi: serde_json::Value,
    pub contract: Option<PackedSource>,
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

/// CLI entry point for `sc-meta reproducible-build download`.
pub async fn download_contract_verification(args: &DownloadArgs) {
    let contract = Bech32Address::from_bech32_str(&args.contract);
    let bech32 = contract.to_bech32_str().to_string();

    let base_url = args.verifier_url.trim_end_matches('/');
    let depth = args.depth.unwrap_or(-1);
    let include_test_files = args.include_test_files;

    let url =
        format!("{base_url}/verifier/{bech32}?depth={depth}&includeTestFiles={include_test_files}");

    println!("Fetching verification data for {bech32} ...");

    let client = reqwest::Client::new();
    let raw = client
        .get(&url)
        .send()
        .await
        .unwrap_or_else(|e| panic!("HTTP request failed: {e}"))
        .text()
        .await
        .unwrap_or_else(|e| panic!("Failed to read response body: {e}"));

    let response: ContractVerifierResponse = serde_json::from_str(&raw)
        .unwrap_or_else(|e| panic!("Failed to parse response: {e}\nRaw: {raw}"));

    if response.status != "success" {
        println!(
            "Contract {bech32} verification status is '{}', not 'success'. Nothing to download.",
            response.status
        );
        return;
    }

    let source = response
        .source
        .unwrap_or_else(|| panic!("Response has no 'source' field."));

    let output_dir = args
        .output
        .as_deref()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));

    fs::create_dir_all(&output_dir).unwrap_or_else(|e| {
        panic!(
            "Failed to create output directory {}: {e}",
            output_dir.display()
        )
    });

    // --- Save ABI ---
    let contract_name = source
        .contract
        .as_ref()
        .map(|c| c.metadata.contract_name.clone())
        .unwrap_or_else(|| bech32.clone());

    let abi_path = output_dir.join(format!("{contract_name}.abi.json"));
    let abi_text = serde_json::to_string_pretty(&source.abi)
        .unwrap_or_else(|e| panic!("Failed to serialize ABI: {e}"));
    fs::write(&abi_path, &abi_text)
        .unwrap_or_else(|e| panic!("Failed to write ABI to {}: {e}", abi_path.display()));
    println!("Saved ABI:    {}", abi_path.display());

    // --- Unpack source entries ---
    if let Some(packed) = &source.contract {
        let sources_dir = output_dir.join("src");
        unpack_entries(&packed.entries, &sources_dir);
        println!(
            "Unpacked {} source files to: {}",
            packed.entries.len(),
            sources_dir.display()
        );
    }

    // --- Print metadata ---
    if let Some(ch) = &response.code_hash {
        println!("Code hash:    {ch}");
    }
    if let Some(img) = &response.docker_image {
        println!("Docker image: {img}");
    }
    if let Some(ipfs) = &response.ipfs_file_hash {
        println!("IPFS hash:    {ipfs}");
    }
}

fn unpack_entries(entries: &[SourceFileEntry], dest: &Path) {
    for entry in entries {
        let file_path = dest.join(&entry.path);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)
                .unwrap_or_else(|e| panic!("Failed to create dir {}: {e}", parent.display()));
        }
        let content = BASE64
            .decode(&entry.content)
            .unwrap_or_else(|e| panic!("Failed to base64-decode entry '{}': {e}", entry.path));
        fs::write(&file_path, content)
            .unwrap_or_else(|e| panic!("Failed to write '{}': {e}", file_path.display()));
    }
}
