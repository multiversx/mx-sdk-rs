use basic_interactor::{AdderInteract, Config};

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn simulator_upgrade_test() {
    let mut basic_interact = AdderInteract::new(Config::chain_simulator_config()).await;

    basic_interact.deploy().await;
    basic_interact.add(1u32).await;

    // Sum will be 1
    let sum = basic_interact.get_sum().await;
    assert_eq!(sum, 1u32.into());

    basic_interact
        .upgrade(7u32, &basic_interact.adder_owner_address.clone(), None)
        .await;

    // Sum will be the updated value of 7
    let sum = basic_interact.get_sum().await;
    assert_eq!(sum, 7u32.into());

    // Upgrade fails
    basic_interact
        .upgrade(
            10u32,
            &basic_interact.wallet_address.clone(),
            Some("upgrade is allowed only for owner"),
        )
        .await;

    // Sum will remain 7
    let sum = basic_interact.get_sum().await;
    assert_eq!(sum, 7u32.into());
}
