use std::io::{self, Write};

use hex::ToHex;
use multiversx_sc_snippets::imports::Bech32Address;

use crate::cli::UnverifyArgs;

use super::verify::{compute_bytes_for_signing, load_private_key};

/// CLI entry point for `sc-meta reproducible-build unverify`.
pub async fn unverify_contract(args: &UnverifyArgs) {
    let contract = Bech32Address::from_bech32_str(&args.contract);

    if !args.skip_confirmation {
        print!(
            "Are you sure you want to unverify contract {}? (y/n): ",
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

    let priv_key = load_private_key(
        args.pem.as_deref(),
        args.keystore.as_deref(),
        args.keystore_password.as_deref(),
    );

    // Build compact payload JSON matching the Python implementation.
    let payload_obj = serde_json::json!({
        "contract": contract.to_bech32_str(),
        "codeHash": args.code_hash,
    });
    let payload = serde_json::to_string(&payload_obj).expect("Failed to serialize JSON");

    // Sign using the same MultiversX message signing protocol as verify.
    let bytes_to_sign = compute_bytes_for_signing(&contract, &payload);
    let signature: [u8; 64] = priv_key.sign(bytes_to_sign);
    let signature_hex: String = signature.encode_hex();

    let request_body = serde_json::json!({
        "signature": signature_hex,
        "payload": payload_obj,
    });

    let url = format!("{}/verifier", args.verifier_url.trim_end_matches('/'));
    println!("Submitting unverify request to {url} ...");

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
        println!("Unverify request completed (status {status}).");
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
