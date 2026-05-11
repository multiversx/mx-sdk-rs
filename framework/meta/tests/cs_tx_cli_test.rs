use std::process::Command;

use multiversx_sc_meta_lib::tools::find_current_workspace;
use multiversx_sc_snippets::{Interactor, imports::Bech32Address, test_wallets};
use multiversx_sdk::wallet::Wallet;
use serial_test::serial;

const CHAIN_SIMULATOR_URL: &str = "http://localhost:8085";
const CHAIN_SIMULATOR_CHAIN_ID: &str = "chain";

/// 0.1 EGLD in the smallest denomination (10^17).
const TRANSFER_AMOUNT: u128 = 100_000_000_000_000_000;

/// Minimum gas for a plain EGLD transfer.
const GAS_LIMIT: u64 = 50_000;

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

    let interactor = Interactor::new(CHAIN_SIMULATOR_URL)
        .await
        .use_chain_simulator(true);

    let wallet = Wallet::from_pem_file(&wallet_pem_path).unwrap();
    let wallet_address = wallet.to_address().to_bech32_default();
    interactor.send_user_funds(&wallet_address).await.unwrap();

    interactor.generate_blocks(10).await.unwrap();

    // ── deploy ────────────────────────────────────────────────────────────────
    let deploy_output = Command::new(sc_meta_bin)
        .args([
            "tx",
            "deploy",
            "--bytecode",
            wasm_path.to_str().unwrap(),
            "--pem",
            wallet_pem_path.to_str().unwrap(),
            "--proxy",
            CHAIN_SIMULATOR_URL,
            "--chain",
            CHAIN_SIMULATOR_CHAIN_ID,
            "--gas-limit",
            "50000000",
            "--arguments",
            "0",
            "--send",
            "--outfile",
            outfile_deploy.to_str().unwrap(),
        ])
        .output()
        .expect("failed to execute sc-meta tx deploy");

    println!(
        "deploy stdout:\n{}",
        String::from_utf8_lossy(&deploy_output.stdout)
    );
    println!(
        "deploy stderr:\n{}",
        String::from_utf8_lossy(&deploy_output.stderr)
    );
    assert!(deploy_output.status.success(), "deploy failed");

    interactor.generate_blocks(10).await.unwrap();

    // Read the deployed contract address from the interaction output file.
    let outfile_content =
        std::fs::read_to_string(&outfile_deploy).expect("failed to read deploy outfile");
    let deploy_json: serde_json::Value =
        serde_json::from_str(&outfile_content).expect("failed to parse deploy outfile JSON");

    // Verify deterministic deploy fields.
    assert_eq!(
        deploy_json["emittedTransaction"]["sender"]
            .as_str()
            .unwrap(),
        wallet_address.to_bech32_str(),
        "deploy sender mismatch"
    );
    assert_eq!(
        deploy_json["emittedTransaction"]["receiver"]
            .as_str()
            .unwrap(),
        Bech32Address::zero_default_hrp().to_bech32_str(),
        "deploy receiver mismatch"
    );
    assert!(
        deploy_json["emittedTransactionData"]
            .as_str()
            .unwrap()
            .ends_with("@0500@0500@"),
        "deploy emittedTransactionData does not end with @0500@0500@"
    );

    let contract_address = deploy_json["contractAddress"]
        .as_str()
        .expect("contractAddress not found in deploy outfile");

    println!("Deployed adder at: {contract_address}");

    // ── add(5) ────────────────────────────────────────────────────────────────
    let status = Command::new(sc_meta_bin)
        .args([
            "tx",
            "call",
            contract_address,
            "--pem",
            wallet_pem_path.to_str().unwrap(),
            "--proxy",
            CHAIN_SIMULATOR_URL,
            "--chain",
            CHAIN_SIMULATOR_CHAIN_ID,
            "--gas-limit",
            "5000000",
            "--function",
            "add",
            "--arguments",
            "5",
            "--send",
            "--outfile",
            outfile_call.to_str().unwrap(),
        ])
        .status()
        .expect("failed to execute sc-meta tx call");

    assert!(status.success(), "add call failed");

    interactor.generate_blocks(10).await.unwrap();

    // Read and verify deterministic call fields.
    let call_content = std::fs::read_to_string(&outfile_call).expect("failed to read call outfile");
    let call_json: serde_json::Value =
        serde_json::from_str(&call_content).expect("failed to parse call outfile JSON");

    assert_eq!(
        call_json["emittedTransaction"]["sender"].as_str().unwrap(),
        wallet_address.to_bech32_str(),
        "call sender mismatch"
    );
    assert_eq!(
        call_json["emittedTransaction"]["receiver"]
            .as_str()
            .unwrap(),
        contract_address,
        "call receiver mismatch"
    );
    assert_eq!(
        call_json["emittedTransactionData"].as_str().unwrap(),
        "add@05",
        "call emittedTransactionData mismatch"
    );

    // ── getSum ────────────────────────────────────────────────────────────────
    let query_output = Command::new(sc_meta_bin)
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

    assert!(query_output.status.success(), "getSum query failed");

    let stdout = String::from_utf8_lossy(&query_output.stdout);
    println!("getSum result: {stdout}");

    // The result is a JSON array of hex-encoded values, e.g. ["05"].
    // 5 decimal = 0x05.
    let result: Vec<String> =
        serde_json::from_str(stdout.trim()).expect("failed to parse query output as JSON");
    assert_eq!(result, vec!["05"], "getSum returned unexpected value");

    // ── upgrade ───────────────────────────────────────────────────────────────
    let upgrade_output = Command::new(sc_meta_bin)
        .args([
            "tx",
            "upgrade",
            contract_address,
            "--bytecode",
            wasm_path.to_str().unwrap(),
            "--pem",
            wallet_pem_path.to_str().unwrap(),
            "--proxy",
            CHAIN_SIMULATOR_URL,
            "--chain",
            CHAIN_SIMULATOR_CHAIN_ID,
            "--gas-limit",
            "50000000",
            "--send",
            "--outfile",
            outfile_upgrade.to_str().unwrap(),
        ])
        .output()
        .expect("failed to execute sc-meta tx upgrade");

    println!(
        "upgrade stdout:\n{}",
        String::from_utf8_lossy(&upgrade_output.stdout)
    );
    println!(
        "upgrade stderr:\n{}",
        String::from_utf8_lossy(&upgrade_output.stderr)
    );
    assert!(upgrade_output.status.success(), "upgrade failed");

    interactor.generate_blocks(10).await.unwrap();

    // Verify the upgrade outfile references the same contract address.
    let upgrade_content =
        std::fs::read_to_string(&outfile_upgrade).expect("failed to read upgrade outfile");
    let upgrade_json: serde_json::Value =
        serde_json::from_str(&upgrade_content).expect("failed to parse upgrade outfile JSON");

    assert_eq!(
        upgrade_json["emittedTransaction"]["receiver"]
            .as_str()
            .unwrap(),
        contract_address,
        "upgrade receiver mismatch"
    );

    // getSum must still return 5 after the upgrade.
    let query_after_upgrade = Command::new(sc_meta_bin)
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
        .expect("failed to execute sc-meta tx query after upgrade");

    assert!(
        query_after_upgrade.status.success(),
        "getSum query after upgrade failed"
    );

    let stdout_after = String::from_utf8_lossy(&query_after_upgrade.stdout);
    let result_after: Vec<String> =
        serde_json::from_str(stdout_after.trim()).expect("failed to parse query output as JSON");
    assert_eq!(
        result_after,
        vec!["05"],
        "getSum returned unexpected value after upgrade"
    );
}

/// Sends a small amount of EGLD from Alice to Bob via the `sc-meta tx new` CLI command
/// and verifies that both balances change as expected.
#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test_egld_transfer_alice_to_bob() {
    let workspace = find_current_workspace().unwrap();
    let alice_pem_path = workspace.join("framework/meta/tests/alice.pem");

    // Connect to the chain simulator.
    let interactor = Interactor::new(CHAIN_SIMULATOR_URL)
        .await
        .use_chain_simulator(true);

    // Register wallets – `register_wallet` automatically funds each account via
    // the chain simulator's `send_user_funds` endpoint.
    let alice_address = test_wallets::alice().to_address();
    let bob_address = test_wallets::bob().to_address();

    // fund alice
    interactor
        .send_user_funds(&alice_address.to_bech32_default())
        .await
        .unwrap();

    // Allow the funding transactions to settle.
    interactor.generate_blocks(10).await.unwrap();

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
    let sc_meta_bin = env!("CARGO_BIN_EXE_sc-meta");

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
        ])
        .status()
        .expect("failed to execute sc-meta tx new");

    assert!(status.success(), "sc-meta tx new command failed");

    // Allow the transfer transaction to settle.
    interactor.generate_blocks(20).await.unwrap();

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

    // Alice must have spent at least the transfer amount (gas fees are on top).
    assert!(
        alice_balance_before - alice_balance_after >= TRANSFER_AMOUNT,
        "Alice's balance did not decrease by at least the transfer amount"
    );
}
