use std::process::Command;

use multiversx_sc_meta_lib::tools::find_current_workspace;
use multiversx_sc_snippets::{Interactor, test_wallets};

const CHAIN_SIMULATOR_URL: &str = "http://localhost:8085";

/// 0.1 EGLD in the smallest denomination (10^17).
const TRANSFER_AMOUNT: u128 = 100_000_000_000_000_000;

/// Minimum gas for a plain EGLD transfer.
const GAS_LIMIT: u64 = 50_000;

/// Sends a small amount of EGLD from Alice to Bob via the `sc-meta tx new` CLI command
/// and verifies that both balances change as expected.
#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test_egld_transfer_alice_to_bob() {
    let workspace = find_current_workspace().unwrap();
    let alice_pem_path = workspace.join("framework/meta/tests/alice.pem");

    // Connect to the chain simulator.
    let mut interactor = Interactor::new(CHAIN_SIMULATOR_URL)
        .await
        .use_chain_simulator(true);

    // Register wallets – `register_wallet` automatically funds each account via
    // the chain simulator's `send_user_funds` endpoint.
    let alice_address = interactor.register_wallet(test_wallets::alice()).await;
    let bob_address = interactor.register_wallet(test_wallets::bob()).await;

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
    interactor.generate_blocks(10).await.unwrap();

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
