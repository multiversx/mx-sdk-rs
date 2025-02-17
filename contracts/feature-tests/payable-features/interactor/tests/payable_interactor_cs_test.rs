// use multiversx_sc_snippets::{imports::Bech32Address, sdk::gateway::SetStateAccount, test_wallets};
use payable_interactor::{Config, PayableInteract};
use serial_test::serial;

#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn simulator_upgrade_test() {
    let mut payable_interact = PayableInteract::new(Config::chain_simulator_config()).await;

    payable_interact.deploy().await;

    payable_interact.check_all_transfers().await;
}
