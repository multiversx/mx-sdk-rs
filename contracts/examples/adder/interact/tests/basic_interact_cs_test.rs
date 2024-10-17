use basic_interact::{AdderInteract, Config};

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn simulator_upgrade_test() {
    let mut basic_interact = AdderInteract::init(Config::chain_simulator_config()).await;
    let wallet_address = basic_interact.wallet_address.clone();
    let adder_owner_address = basic_interact.adder_owner_address.clone();
    let error_not_owner = (4, "upgrade is allowed only for owner");

    basic_interact.deploy().await;
    basic_interact.add(1u32).await;

    // Sum will be 1
    let sum = basic_interact.get_sum().await;
    assert_eq!(sum, 1u32.into());

    basic_interact
        .upgrade(7u32, &adder_owner_address, None)
        .await;

    // Sum will be the updated value of 7
    let sum = basic_interact.get_sum().await;
    assert_eq!(sum, 7u32.into());

    basic_interact
        .upgrade(10u32, &wallet_address, Some(error_not_owner))
        .await;

    // // Sum will remain 7
    let sum = basic_interact.get_sum().await;
    assert_eq!(sum, 7u32.into());
}
