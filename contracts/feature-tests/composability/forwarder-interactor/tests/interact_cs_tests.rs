use forwarder_interact::{Config, ContractInteract};
use multiversx_sc_snippets::imports::*;
use serial_test::serial;

fn chain_simulator_config() -> Config {
    Config {
        connection: ConnectionConfig::chain_simulator(),
        wallet: WalletConfig::from_test_wallet("alice"),
    }
}

async fn cs_interactor() -> ContractInteract {
    let config = chain_simulator_config();
    let interactor = HttpInteractor::empty()
        .with_current_dir(env!("CARGO_MANIFEST_DIR"))
        .with_config(&config)
        .await;
    let wallet_address = config.wallet.address();
    ContractInteract {
        interactor,
        wallet_address,
        contract_code: BytesValue::interpret_from(
            "mxsc:../forwarder/output/forwarder.mxsc.json",
            &InterpreterContext::default(),
        ),
        state: multiversx_sc_snippets::AutoSave::no_save_default(),
    }
}

// Simple deploy test that runs using the chain simulator configuration.
// In order for this test to work, make sure that the `config.toml` file contains the chain simulator config (or choose it manually)
// The chain simulator should already be installed and running before attempting to run this test.
// The chain-simulator-tests feature should be present in Cargo.toml.
// Can be run with `sc-meta test -c`.
#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn deploy_test_forwarder_cs() {
    let mut interactor = cs_interactor().await;

    interactor.deploy().await;
}
