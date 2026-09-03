//! Note: Currently not included in the template.
#![cfg(feature = "ledger")]

use adder_interactor::{AdderInteractor, Config, GeneralConfig};
use multiversx_sc_snippets::imports::*;
use multiversx_sc_snippets::sdk::wallet::ledger::{LedgerApp, WalletTransport};
use serial_test::serial;

/// Builds a chain-simulator config that uses carol's test wallet as the mock
/// ledger signer. `WalletTransport::new(carol)` simulates the Ledger app at
/// the APDU level; the interactor uses the same backing private key for signing.
fn chain_simulator_ledger_config() -> Config {
    Config {
        general: GeneralConfig {
            contract_path: "../output/adder.mxsc.json".into(),
        },
        connection: ConnectionConfig::chain_simulator(),
        owner: WalletConfig::from_test_wallet("mike"),
        wallet: WalletConfig::from_test_wallet("ivan"),
        ledger_wallet: Some(WalletConfig::from_test_wallet("carol")),
        relayer: None,
    }
}

async fn test_interactor() -> AdderInteractor {
    let config = chain_simulator_ledger_config();
    let interactor = Interactor::empty()
        .with_current_dir(env!("CARGO_MANIFEST_DIR"))
        .with_config(&config)
        .await;
    AdderInteractor {
        interactor,
        config,
        state: AutoSave::no_save_default(),
    }
}

#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn add_ledger_mock_test() {
    // Verify that WalletTransport correctly simulates Ledger address lookup.
    // In production the interactor would call LedgerApp::new() to open the
    // physical device; here we inject WalletTransport so no hardware is needed.
    let carol = test_wallets::carol();
    let transport = WalletTransport::new(carol.clone());
    let mut mock_app = LedgerApp::with_transport(Box::new(transport));
    let ledger_addr = mock_app.get_address(0).unwrap();
    assert_eq!(
        ledger_addr,
        carol.to_bech32().to_string(),
        "WalletTransport must report the same address as its backing wallet"
    );

    let mut adder_interact = test_interactor().await;

    adder_interact.generate_blocks(2).await;

    adder_interact.deploy().await;

    adder_interact.generate_blocks(2).await;

    // add_ledger sends the transaction from carol's address (the mock ledger wallet).
    adder_interact.add_ledger(5u32).await;

    adder_interact.generate_blocks(2).await;

    let sum = adder_interact.get_sum().await;
    assert_eq!(sum, 5u32.into());

    // A second add to confirm accumulation.
    adder_interact.add_ledger(3u32).await;

    adder_interact.generate_blocks(2).await;

    let sum = adder_interact.get_sum().await;
    assert_eq!(sum, 8u32.into());
}
