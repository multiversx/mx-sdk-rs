use multiversx_sc_snippets::imports::*;
use payable_interactor::{Config, PayableInteract};
use serial_test::serial;

fn chain_simulator_config() -> Config {
    Config {
        connection: ConnectionConfig::chain_simulator(),
        owner: WalletConfig::from_test_wallet("mike"),
        wallet: WalletConfig::from_test_wallet("ivan"),
    }
}

async fn cs_interactor() -> PayableInteract {
    let config = chain_simulator_config();
    let interactor = HttpInteractor::empty()
        .with_current_dir(env!("CARGO_MANIFEST_DIR"))
        .with_config(&config)
        .await;
    PayableInteract {
        interactor,
        config,
        state: multiversx_sc_snippets::AutoSave::no_save_default(),
    }
}

#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn payable_interactor_test() {
    let mut payable_interact = cs_interactor().await;

    payable_interact.deploy().await;

    payable_interact
        .check_multi_transfer_only_egld_transfer()
        .await;
}
