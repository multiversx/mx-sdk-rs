use std::io::{self, Write};

use hex::ToHex;
use multiversx_sc_snippets::imports::Bech32Address;

use crate::cli::ReproducibleBuildUnpublishArgs;

use super::publish::compute_bytes_for_signing;
use crate::cli::cli_args_sender::load_wallet;

/// CLI entry point for `sc-meta reproducible-build unpublish`.
pub async fn unpublish_contract(args: &ReproducibleBuildUnpublishArgs) {
    let contract = Bech32Address::from_bech32_str(&args.contract);

    if !args.skip_confirmation {
        print!(
            "Are you sure you want to unpublish contract sources {}? (y/n): ",
            contract.to_bech32_str()
        );
        io::stdout().flush().unwrap();
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).unwrap();
        if !answer.trim().eq_ignore_ascii_case("y") {
            println!("Aborted.");
            return;
        }
    }

    let wallet = load_wallet(&args.sender).unwrap_or_else(|e| panic!("Failed to load wallet: {e}"));

    // Build compact payload JSON matching the Python implementation.
    let payload_obj = serde_json::json!({
        "contract": contract.to_bech32_str(),
        "codeHash": args.code_hash,
    });
    let payload = serde_json::to_string(&payload_obj).expect("Failed to serialize JSON");

    // Sign using the same MultiversX message signing protocol as verify.
    let bytes_to_sign = compute_bytes_for_signing(&contract, &payload);
    let signature: [u8; 64] = wallet.sign_bytes(bytes_to_sign);
    let signature_hex: String = signature.encode_hex();

    let request_body = serde_json::json!({
        "signature": signature_hex,
        "payload": payload_obj,
    });

    let url = format!("{}/verifier", args.verifier_url.trim_end_matches('/'));
    println!("Submitting unpublish request to {url} ...");

    let client = reqwest::Client::new();
    let response = client
        .delete(&url)
        .json(&request_body)
        .send()
        .await
        .unwrap_or_else(|e| panic!("HTTP request failed: {e}"));

    let status = response.status().as_u16();
    let raw = response
        .text()
        .await
        .unwrap_or_else(|e| panic!("Failed to read response body: {e}"));

    if raw.is_empty() {
        println!("Unpublish request completed (status {status}).");
        return;
    }

    match serde_json::from_str::<serde_json::Value>(&raw) {
        Ok(body) => {
            if let Some(message) = body.get("message").and_then(|v| v.as_str()) {
                println!("{message}");
            } else {
                println!("{}", serde_json::to_string_pretty(&body).unwrap());
            }
        }
        Err(_) => println!("{raw}"),
    }
}
