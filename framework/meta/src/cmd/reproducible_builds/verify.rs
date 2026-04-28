use std::io::{self, Write};
use std::time::Duration;

use hex::ToHex;
use multiversx_sc_snippets::sdk::{crypto::private_key::PrivateKey, wallet::Wallet};
use sha2::{Digest, Sha256};
use sha3::Keccak256;

use multiversx_sc_snippets::imports::Bech32Address;

use crate::cli::VerifyArgs;

const ELROND_SIGNED_MESSAGE_PREFIX: &[u8] = b"\x17Elrond Signed Message:\n";
const HTTP_STATUS_OK: u16 = 200;
const HTTP_STATUS_TIMEOUT: u16 = 408;

/// CLI entry point for `sc-meta reproducible-build verify`.
pub async fn verify_contract(args: &VerifyArgs) {
    let contract = Bech32Address::from_bech32_str(&args.contract);

    if !args.skip_confirmation {
        print!(
            "Are you sure you want to verify contract {}? \
             This will publish the contract's source code (y/n): ",
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

    // Read source.json as a raw JSON value (to re-serialize compact with no extra whitespace).
    let source_json_text = std::fs::read_to_string(&args.packaged_src)
        .unwrap_or_else(|e| panic!("Failed to read {}: {e}", args.packaged_src));
    let source_code: serde_json::Value = serde_json::from_str(&source_json_text)
        .unwrap_or_else(|e| panic!("Failed to parse {}: {e}", args.packaged_src));

    // Build compact payload JSON.
    let payload = build_payload(
        &contract,
        &source_code,
        &args.docker_image,
        args.contract_variant.as_deref(),
    );

    // Sign the payload.
    let signature_hex = sign_payload(&priv_key, &contract, &payload);

    let url = format!("{}/verifier", args.verifier_url.trim_end_matches('/'));
    let (status, body) = post_verification_request(
        &url,
        &signature_hex,
        &contract,
        &args.docker_image,
        &source_code,
        args.contract_variant.as_deref(),
    )
    .await;
    handle_verification_response(&args.verifier_url, status, body).await;
}

// ─── Helpers ────────────────────────────────────────────────────────────────

/// Loads the private key from a PEM file or keystore, depending on CLI args.
pub(super) fn load_private_key(
    pem: Option<&str>,
    keystore: Option<&str>,
    keystore_password: Option<&str>,
) -> PrivateKey {
    match (pem, keystore) {
        (Some(pem_path), None) => {
            let (priv_key_hex, _) = Wallet::get_wallet_keys_pem(pem_path);
            PrivateKey::from_hex_str(&priv_key_hex)
                .unwrap_or_else(|e| panic!("Failed to parse PEM private key: {e}"))
        }
        (None, Some(keystore_path)) => {
            let password = match keystore_password {
                Some(pw) => pw.to_string(),
                None => Wallet::get_keystore_password(),
            };
            Wallet::get_private_key_from_keystore_secret(keystore_path, &password)
                .unwrap_or_else(|e| panic!("Failed to load keystore: {e}"))
        }
        (None, None) => panic!("Either --pem or --keystore must be provided"),
        (Some(_), Some(_)) => panic!("--pem and --keystore are mutually exclusive"),
    }
}

/// Builds a compact JSON payload string (no whitespace) matching the Python implementation.
fn build_payload(
    contract: &Bech32Address,
    source_code: &serde_json::Value,
    docker_image: &str,
    contract_variant: Option<&str>,
) -> String {
    let obj = serde_json::json!({
        "contract": contract.to_bech32_str(),
        "dockerImage": docker_image,
        "sourceCode": source_code,
        "contractVariant": contract_variant,
    });
    serde_json::to_string(&obj).expect("Failed to serialize JSON")
}

/// Computes the bytes to sign for a contract verification request.
///
/// Mirrors Python's `MessageComputer.compute_bytes_for_signing(Message(raw_data))`:
/// 1. sha256(payload_bytes) → hex string `hashed_payload`
/// 2. raw_data = "{contract_bech32}{hashed_payload}"
/// 3. content = PREFIX + len(raw_data) + raw_data  (PREFIX = `\x17Elrond Signed Message:\n`)
/// 4. return keccak256(content)
pub(super) fn compute_bytes_for_signing(contract: &Bech32Address, payload: &str) -> Vec<u8> {
    let hashed_payload: String = Sha256::digest(payload.as_bytes()).encode_hex();
    let raw_data = format!("{}{hashed_payload}", contract.to_bech32_str());
    let raw_data_bytes = raw_data.as_bytes();
    let len_str = raw_data_bytes.len().to_string();
    let mut content = Vec::with_capacity(
        ELROND_SIGNED_MESSAGE_PREFIX.len() + len_str.len() + raw_data_bytes.len(),
    );
    content.extend_from_slice(ELROND_SIGNED_MESSAGE_PREFIX);
    content.extend_from_slice(len_str.as_bytes());
    content.extend_from_slice(raw_data_bytes);
    Keccak256::digest(&content).to_vec()
}

/// Signs the payload using the MultiversX message signing protocol and returns the hex signature.
pub(super) fn sign_payload(
    priv_key: &PrivateKey,
    contract: &Bech32Address,
    payload: &str,
) -> String {
    let bytes_to_sign = compute_bytes_for_signing(contract, payload);
    let signature: [u8; 64] = priv_key.sign(bytes_to_sign);
    signature.encode_hex()
}

/// POSTs the verification request and returns the HTTP status code and parsed response body.
async fn post_verification_request(
    url: &str,
    signature: &str,
    contract: &Bech32Address,
    docker_image: &str,
    source_code: &serde_json::Value,
    contract_variant: Option<&str>,
) -> (u16, serde_json::Value) {
    let request_body = serde_json::json!({
        "signature": signature,
        "payload": {
            "contract": contract.to_bech32_str(),
            "dockerImage": docker_image,
            "sourceCode": source_code,
            "contractVariant": contract_variant,
        },
    });
    println!("Submitting verification request to {url} ...");
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .json(&request_body)
        .send()
        .await
        .unwrap_or_else(|e| panic!("HTTP request failed: {e}"));
    let status = response.status().as_u16();
    let body: serde_json::Value = response
        .json()
        .await
        .unwrap_or_else(|e| panic!("Failed to parse response: {e}"));
    (status, body)
}

/// Handles the response from the verifier endpoint, polling for the result if needed.
async fn handle_verification_response(verifier_url: &str, status: u16, body: serde_json::Value) {
    if status == HTTP_STATUS_TIMEOUT {
        // Task accepted asynchronously.
        if let Some(task_id) = body.get("taskId").and_then(|v| v.as_str()) {
            poll_task(verifier_url, task_id).await;
        } else {
            println!(
                "Response (408): {}",
                serde_json::to_string_pretty(&body).unwrap()
            );
        }
    } else if status == HTTP_STATUS_OK {
        let task_status = body.get("status").and_then(|v| v.as_str()).unwrap_or("");
        if !task_status.is_empty() {
            println!("Task status: {task_status}");
            println!("{}", serde_json::to_string_pretty(&body).unwrap());
        } else if let Some(task_id) = body.get("taskId").and_then(|v| v.as_str()) {
            poll_task(verifier_url, task_id).await;
        } else {
            println!("{}", serde_json::to_string_pretty(&body).unwrap());
        }
    } else {
        println!("{}", serde_json::to_string_pretty(&body).unwrap());
        panic!("Verification request failed with status {status}");
    }
}

/// Fetches the current status of a verification task.
async fn check_task_once(client: &reqwest::Client, url: &str) -> serde_json::Value {
    client
        .get(url)
        .send()
        .await
        .unwrap_or_else(|e| panic!("Polling request failed: {e}"))
        .json()
        .await
        .unwrap_or_else(|e| panic!("Failed to parse polling response: {e}"))
}

/// Polls the verifier task endpoint every 10 seconds until `status == "finished"`.
async fn poll_task(verifier_url: &str, task_id: &str) {
    println!("Verification task submitted (id={task_id}). Polling for result...");
    let url = format!("{}/tasks/{task_id}", verifier_url.trim_end_matches('/'));
    let client = reqwest::Client::new();
    let mut last_status = String::new();
    let mut attempt: u32 = 0;

    loop {
        tokio::time::sleep(Duration::from_secs(10)).await;
        attempt += 1;
        println!("Polling attempt {attempt} ...");

        let body = check_task_once(&client, &url).await;
        let status = body
            .get("status")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        if status == "finished" {
            println!("Verification finished!");
            println!("{}", serde_json::to_string_pretty(&body).unwrap());
            break;
        }

        if status != last_status {
            println!("Task status: {status}");
            println!("{}", serde_json::to_string_pretty(&body).unwrap());
            last_status = status;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex::ToHex;
    use sha2::{Digest, Sha256};
    use sha3::Keccak256;

    /// Independently re-implements the signing bytes computation and checks it matches.
    #[test]
    fn test_compute_bytes_for_signing() {
        let contract = "erd1qqqqqqqqqqqqqpgq8qr7fyfxufdzcd9tf7h6kelngnumryyua4sql2fwgs"
            .parse()
            .unwrap();
        let payload = r#"{"contract":"erd1qqqqqqqqqqqqqpgq8qr7fyfxufdzcd9tf7h6kelngnumryyua4sql2fwgs","dockerImage":"img:v1","sourceCode":{},"contractVariant":null}"#;

        let result = compute_bytes_for_signing(&contract, payload);

        // Must be 32 bytes (keccak256 output).
        assert_eq!(result.len(), 32);

        // Independently compute the expected value step by step.
        let hashed_payload: String = Sha256::digest(payload.as_bytes()).encode_hex();
        let raw_data = format!("{}{hashed_payload}", contract.to_bech32_str());
        let mut content = Vec::new();
        content.extend_from_slice(ELROND_SIGNED_MESSAGE_PREFIX);
        content.extend_from_slice(raw_data.len().to_string().as_bytes());
        content.extend_from_slice(raw_data.as_bytes());
        let expected = Keccak256::digest(&content).to_vec();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_compute_bytes_for_signing_differs_on_contract() {
        let payload = "{}";
        assert_ne!(
            compute_bytes_for_signing(
                &"erd1qqqqqqqqqqqqqpgq8qr7fyfxufdzcd9tf7h6kelngnumryyua4sql2fwgs"
                    .parse()
                    .unwrap(),
                payload
            ),
            compute_bytes_for_signing(
                &"erd1qqqqqqqqqqqqqpgq2tpcmqw40u7ncj29n2d0uqnujj0w9vxma4sqwjdx3v"
                    .parse()
                    .unwrap(),
                payload
            ),
        );
    }

    #[test]
    fn test_compute_bytes_for_signing_differs_on_payload() {
        let contract = "erd1qqqqqqqqqqqqqpgq8qr7fyfxufdzcd9tf7h6kelngnumryyua4sql2fwgs"
            .parse()
            .unwrap();
        assert_ne!(
            compute_bytes_for_signing(&contract, r#"{"v":1}"#),
            compute_bytes_for_signing(&contract, r#"{"v":2}"#),
        );
    }
}
