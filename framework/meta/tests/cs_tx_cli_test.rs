use std::process::Command;

use multiversx_sc_meta_lib::tools::find_current_workspace;
use multiversx_sc_snippets::{Interactor, imports::Bech32Address, test_wallets};
use multiversx_sdk::wallet::Wallet;
use serial_test::serial;

const CHAIN_SIMULATOR_URL: &str = "http://localhost:8085";
const CHAIN_SIMULATOR_CHAIN_ID: &str = "chain";

/// 0.1 EGLD in the smallest denomination (10^17).
const TRANSFER_AMOUNT: u128 = 100_000_000_000_000_000;

/// 100 EGLD, the amount senders get funded automatically
/// on interactor/sc-meta CLI + chain simulator.
const FUND_AMOUNT: u128 = 100_000_000_000_000_000_000;

/// Minimum gas for a plain EGLD transfer.
const GAS_LIMIT: u64 = 50_000;

struct AdderTestContext<'a> {
    sc_meta_bin: &'a str,
    wasm_path: &'a std::path::Path,
    wallet_pem_path: &'a std::path::Path,
    wallet_address: &'a str,
}

/// Deploys the adder contract, calls `add`, and verifies `getSum` returns the expected value.
/// Mirrors the deploy / add / getSum flow from the adder snippets.sh.
#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test_adder_deploy_add_get_sum() {
    let workspace = find_current_workspace().unwrap();
    let test_artefacts_dir = workspace.join("framework/meta/tests/cs_tx_cli_test");
    let wallet_pem_path = test_artefacts_dir.join("cs_tx_test_owner.pem");
    let wasm_path = test_artefacts_dir.join("adder.wasm");
    let outfiles_dir = test_artefacts_dir.join("outfiles");
    std::fs::create_dir_all(&outfiles_dir).unwrap();
    let outfile_deploy = outfiles_dir.join("adder-deploy-cs.interaction.json");
    let outfile_call = outfiles_dir.join("adder-call-cs.interaction.json");
    let outfile_upgrade = outfiles_dir.join("adder-upgrade-cs.interaction.json");

    let sc_meta_bin = env!("CARGO_BIN_EXE_sc-meta");

    let wallet = Wallet::from_pem_file(&wallet_pem_path).unwrap();
    let wallet_address = wallet
        .to_address()
        .to_bech32_default()
        .to_bech32_str()
        .to_owned();
    let context = AdderTestContext {
        sc_meta_bin,
        wasm_path: &wasm_path,
        wallet_pem_path: &wallet_pem_path,
        wallet_address: &wallet_address,
    };

    // Connect to the chain simulator, generate some initial blocks.
    let interactor = Interactor::new(CHAIN_SIMULATOR_URL)
        .await
        .use_chain_simulator(true);
    interactor.generate_blocks(10).await.unwrap();

    let contract_address = deploy_adder(&context, &outfile_deploy);
    call_adder(&context, &contract_address, &outfile_call, 5);
    assert_eq!(query_sum(&context, &contract_address), vec!["05"]);

    upgrade_adder(&context, &contract_address, &outfile_upgrade);
    assert_eq!(query_sum(&context, &contract_address), vec!["0a"]);

    let relayer_pem_path = test_artefacts_dir.join("s1mon.pem");
    let outfile_call_relayed = outfiles_dir.join("adder-call-relayed-cs.interaction.json");

    generate_test_wallet(context.sc_meta_bin, "s1mon", &relayer_pem_path);
    call_adder_relayed(
        &context,
        &contract_address,
        &relayer_pem_path,
        &outfile_call_relayed,
        "3",
    );
    assert_eq!(query_sum(&context, &contract_address), vec!["0d"]);
}

fn deploy_adder(context: &AdderTestContext<'_>, outfile: &std::path::Path) -> String {
    let output = Command::new(context.sc_meta_bin)
        .args([
            "tx",
            "deploy",
            "--bytecode",
            context.wasm_path.to_str().unwrap(),
            "--pem",
            context.wallet_pem_path.to_str().unwrap(),
            "--proxy",
            CHAIN_SIMULATOR_URL,
            "--chain",
            CHAIN_SIMULATOR_CHAIN_ID,
            "--gas-limit",
            "50000000",
            "--arguments",
            "0",
            "--send",
            "--wait-result",
            "--outfile",
            outfile.to_str().unwrap(),
        ])
        .output()
        .expect("failed to execute sc-meta tx deploy");
    println!(
        "deploy stdout:\n{}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert!(
        output.status.success(),
        "deploy failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let content = std::fs::read_to_string(outfile).expect("failed to read deploy outfile");
    let json: serde_json::Value =
        serde_json::from_str(&content).expect("failed to parse deploy outfile JSON");
    assert_eq!(
        json["emittedTransaction"]["sender"].as_str().unwrap(),
        context.wallet_address,
        "deploy sender mismatch"
    );
    assert_eq!(
        json["emittedTransaction"]["receiver"].as_str().unwrap(),
        Bech32Address::zero_default_hrp().to_bech32_str(),
        "deploy receiver mismatch"
    );
    assert!(
        json["emittedTransactionData"]
            .as_str()
            .unwrap()
            .ends_with("@0500@0500@"),
        "deploy emittedTransactionData does not end with @0500@0500@"
    );

    let contract_address = json["contractAddress"]
        .as_str()
        .expect("contractAddress not found in deploy outfile")
        .to_owned();
    println!("Deployed adder at: {contract_address}");
    contract_address
}

fn call_adder(
    context: &AdderTestContext<'_>,
    contract_address: &str,
    outfile: &std::path::Path,
    argument: usize,
) {
    let output = Command::new(context.sc_meta_bin)
        .args([
            "tx",
            "call",
            contract_address,
            "--pem",
            context.wallet_pem_path.to_str().unwrap(),
            "--proxy",
            CHAIN_SIMULATOR_URL,
            "--chain",
            CHAIN_SIMULATOR_CHAIN_ID,
            "--gas-limit",
            "5000000",
            "--function",
            "add",
            "--arguments",
            argument.to_string().as_str(),
            "--send",
            "--wait-result",
            "--outfile",
            outfile.to_str().unwrap(),
        ])
        .output()
        .expect("failed to execute sc-meta tx call");
    println!("add stdout:\n{}", String::from_utf8_lossy(&output.stdout));
    assert!(output.status.success(), "add call failed");

    let content = std::fs::read_to_string(outfile).expect("failed to read call outfile");
    let json: serde_json::Value =
        serde_json::from_str(&content).expect("failed to parse call outfile JSON");
    assert_eq!(
        json["emittedTransaction"]["sender"].as_str().unwrap(),
        context.wallet_address,
        "call sender mismatch"
    );
    assert_eq!(
        json["emittedTransaction"]["receiver"].as_str().unwrap(),
        contract_address,
        "call receiver mismatch"
    );
    let expected_transaction_data = format!("add@{:02x}", argument);
    assert_eq!(
        json["emittedTransactionData"].as_str().unwrap(),
        expected_transaction_data,
        "call emittedTransactionData mismatch"
    );
}

fn query_sum(context: &AdderTestContext<'_>, contract_address: &str) -> Vec<String> {
    let output = Command::new(context.sc_meta_bin)
        .args([
            "tx",
            "query",
            contract_address,
            "--proxy",
            CHAIN_SIMULATOR_URL,
            "--function",
            "getSum",
        ])
        .output()
        .expect("failed to execute sc-meta tx query");
    assert!(output.status.success(), "getSum query failed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("getSum result: {stdout}");
    serde_json::from_str(stdout.trim()).expect("failed to parse query output as JSON")
}

fn upgrade_adder(
    context: &AdderTestContext<'_>,
    contract_address: &str,
    outfile: &std::path::Path,
) {
    let output = Command::new(context.sc_meta_bin)
        .args([
            "tx",
            "upgrade",
            contract_address,
            "--bytecode",
            context.wasm_path.to_str().unwrap(),
            "--pem",
            context.wallet_pem_path.to_str().unwrap(),
            "--proxy",
            CHAIN_SIMULATOR_URL,
            "--chain",
            CHAIN_SIMULATOR_CHAIN_ID,
            "--gas-limit",
            "50000000",
            "--arguments",
            "10",
            "--send",
            "--wait-result",
            "--outfile",
            outfile.to_str().unwrap(),
        ])
        .output()
        .expect("failed to execute sc-meta tx upgrade");
    println!(
        "upgrade stdout:\n{}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert!(output.status.success(), "upgrade failed");

    let content = std::fs::read_to_string(outfile).expect("failed to read upgrade outfile");
    let json: serde_json::Value =
        serde_json::from_str(&content).expect("failed to parse upgrade outfile JSON");
    assert_eq!(
        json["emittedTransaction"]["receiver"].as_str().unwrap(),
        contract_address,
        "upgrade receiver mismatch"
    );
}

fn call_adder_relayed(
    context: &AdderTestContext<'_>,
    contract_address: &str,
    relayer_pem_path: &std::path::Path,
    outfile: &std::path::Path,
    argument: &str,
) {
    let output = Command::new(context.sc_meta_bin)
        .args([
            "tx",
            "call",
            contract_address,
            "--pem",
            context.wallet_pem_path.to_str().unwrap(),
            "--relayer-pem",
            relayer_pem_path.to_str().unwrap(),
            "--proxy",
            CHAIN_SIMULATOR_URL,
            "--chain",
            CHAIN_SIMULATOR_CHAIN_ID,
            "--gas-limit",
            "5050000",
            "--function",
            "add",
            "--arguments",
            argument,
            "--send",
            "--wait-result",
            "--outfile",
            outfile.to_str().unwrap(),
        ])
        .output()
        .expect("failed to execute sc-meta tx call (relayed)");
    println!(
        "relayed add stdout:\n{}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert!(output.status.success(), "relayed add call failed");
}

/// Sends a small amount of EGLD from Alice to Bob via the `sc-meta tx new` CLI command
/// and verifies that both balances change as expected.
#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test_egld_transfer_alice_to_bob() {
    let workspace = find_current_workspace().unwrap();
    let test_artefacts_dir = workspace.join("framework/meta/tests/cs_tx_cli_test");
    let alice_pem_path = test_artefacts_dir.join("alice.pem");

    let sc_meta_bin = env!("CARGO_BIN_EXE_sc-meta");

    // Generate alice.pem via the CLI so the file is not stored in the repo.
    generate_test_wallet(sc_meta_bin, "alice", &alice_pem_path);

    // Connect to the chain simulator.
    let interactor = Interactor::new(CHAIN_SIMULATOR_URL)
        .await
        .use_chain_simulator(true);
    interactor.generate_blocks(10).await.unwrap();

    // Register wallets – `register_wallet` automatically funds each account via
    // the chain simulator's `send_user_funds` endpoint.
    let alice_address = test_wallets::alice().to_address();
    let bob_address = test_wallets::bob().to_address();

    // ── balances before transfer ──────────────────────────────────────────────
    let alice_balance_before: u128 = interactor
        .get_account(&alice_address)
        .await
        .balance
        .parse()
        .expect("failed to parse Alice's balance");

    let bob_balance_before: u128 = interactor
        .get_account(&bob_address)
        .await
        .balance
        .parse()
        .expect("failed to parse Bob's balance");

    println!("Alice balance before: {alice_balance_before}");
    println!("Bob balance before:   {bob_balance_before}");

    // Bob's bech32 address is the receiver argument for the CLI command.
    let bob_bech32 = bob_address.to_bech32_default();

    // ── execute the transfer via the sc-meta CLI ──────────────────────────────
    let status = Command::new(sc_meta_bin)
        .args([
            "tx",
            "new",
            "--proxy",
            CHAIN_SIMULATOR_URL,
            "--receiver",
            bob_bech32.to_bech32_str(),
            "--pem",
            alice_pem_path.to_str().unwrap(),
            "--gas-limit",
            &GAS_LIMIT.to_string(),
            "--value",
            &TRANSFER_AMOUNT.to_string(),
            "--send",
            "--wait-result",
        ])
        .status()
        .expect("failed to execute sc-meta tx new");

    assert!(status.success(), "sc-meta tx new command failed");

    // ── balances after transfer ───────────────────────────────────────────────
    let alice_balance_after: u128 = interactor
        .get_account(&alice_address)
        .await
        .balance
        .parse()
        .expect("failed to parse Alice's balance after transfer");

    let bob_balance_after: u128 = interactor
        .get_account(&bob_address)
        .await
        .balance
        .parse()
        .expect("failed to parse Bob's balance after transfer");

    println!("Alice balance after:  {alice_balance_after}");
    println!("Bob balance after:    {bob_balance_after}");

    // Bob must have received exactly the transferred amount.
    assert_eq!(
        bob_balance_after - bob_balance_before,
        TRANSFER_AMOUNT,
        "Bob's balance did not increase by the expected transfer amount"
    );

    // Alice was funded automatically by the CLI,
    // since the chain-simulator config was auto-detected,
    // hence the additional FUND_AMOUNT.
    // She must have spent at least the transfer amount (gas fees are on top).
    assert!(
        alice_balance_before + FUND_AMOUNT - alice_balance_after >= TRANSFER_AMOUNT,
        "Alice's balance did not decrease by at least the transfer amount"
    );
}

fn generate_test_wallet(sc_meta_bin: &str, name: &str, path: &std::path::Path) {
    let status = Command::new(sc_meta_bin)
        .args([
            "wallet",
            "test-wallet",
            "--name",
            name,
            "--path",
            path.to_str().unwrap(),
        ])
        .status()
        .unwrap_or_else(|_| panic!("failed to generate {name}.pem"));
    assert!(
        status.success(),
        "sc-meta wallet test-wallet failed for {name}"
    );
}
