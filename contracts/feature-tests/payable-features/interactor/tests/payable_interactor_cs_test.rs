use payable_interactor::{Config, PayableInteract};
use serial_test::serial;

#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn payable_interactor_test() {
    let mut payable_interact = PayableInteract::new(Config::chain_simulator_config()).await;

    payable_interact.deploy().await;

    payable_interact
        .check_multi_transfer_only_egld_transfer()
        .await;
}
