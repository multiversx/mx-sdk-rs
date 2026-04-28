use multiversx_sc_snippets::imports::Bech32Address;

use crate::cli::CheckArgs;

/// CLI entry point for `sc-meta reproducible-build check`.
///
/// GETs the verified-contracts list from the verifier service and reports
/// whether the given contract address is currently verified.
pub async fn check_contract_verification(args: &CheckArgs) {
    let contract = Bech32Address::from_bech32_str(&args.contract);
    let url = format!("{}/verifier", args.verifier_url.trim_end_matches('/'));

    println!(
        "Checking verification status for {} at {url} ...",
        contract.to_bech32_str()
    );

    let client = reqwest::Client::new();
    let raw = client
        .get(&url)
        .send()
        .await
        .unwrap_or_else(|e| panic!("HTTP request failed: {e}"))
        .text()
        .await
        .unwrap_or_else(|e| panic!("Failed to read response body: {e}"));

    let list: Vec<String> =
        serde_json::from_str(&raw).unwrap_or_else(|e| panic!("Failed to parse response: {e}"));

    let bech32 = contract.to_bech32_str();
    if list.iter().any(|addr| addr == bech32) {
        println!("Contract {bech32} is VERIFIED.");
    } else {
        println!("Contract {bech32} is NOT verified.");
    }
}
