use lottery_interactor::{Config, LotteryInteract};
use multiversx_sc_snippets::{
    imports::{num_bigint, GatewayHttpProxy, OptionalValue},
    sdk::gateway::DEVNET_GATEWAY,
    test_wallets,
};
use num_bigint::BigUint;
use serial_test::serial;

#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn set_state_from_file_cs_test() {
    let mut interact = LotteryInteract::new(Config::chain_simulator_config()).await;

    interact.deploy().await;
    // interact
    //     .create_lottery_pool(
    //         &"lottery_name".to_string(),
    //         "LOTTERY-123456".into(),
    //         BigUint::from(100u64),
    //         None,
    //         Some(123_456),
    //         None,
    //         None,
    //         None,
    //         OptionalValue::None,
    //     )
    //     .await;
}
