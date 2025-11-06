use lottery_interactor::{Config, LotteryInteract};
use multiversx_sc_snippets::{imports::*, sdk::gateway::NetworkStatusRequest};
use serial_test::serial;

pub const CHAIN_SIMULATOR_GATEWAY: &str = "http://localhost:8085";
const ONE_MINUTE_IN_SECONDS: u64 = 60;
const LOTTERY_NAME: &str = "LOTTERY";

#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn determine_winner_with_caller_shard_check_test() {
    let mut interact = LotteryInteract::new(Config::chain_simulator_config()).await;

    interact.deploy().await;
    let current_timestamp = get_current_timestamp().await;

    interact
        .create_lottery_pool(
            &LOTTERY_NAME.to_string(),
            &"LOTTERY-123456".to_string(),
            100u128,
            Some(11),
            Some(current_timestamp + ONE_MINUTE_IN_SECONDS),
            Some(1),
            Some(vec![10, 50, 25, 5, 5, 1, 1, 1, 1, 1]),
            None,
            OptionalValue::None,
        )
        .await;

    interact.generate_blocks_until_epoch(5).await;

    // Call `determine_winner` from the same shard as the SC - should fail
    interact
        .determine_winner(
            &interact.lottery_owner.address.clone(),
            &LOTTERY_NAME.to_string(),
            Some(ExpectError(4, "Caller needs to be on a remote shard")),
        )
        .await;

    // Call `determine_winner` from a different shard - should pass
    interact
        .determine_winner(
            &interact.other_shard_account.address.clone(),
            &LOTTERY_NAME.to_string(),
            None,
        )
        .await;
    // Call `determine_winner` after awarding ended as a safe check that awarding the same lotteyr cannot be done twice - should fail-
    interact
        .determine_winner(
            &interact.other_shard_account.address.clone(),
            &LOTTERY_NAME.to_string(),
            Some(ExpectError(4, "Lottery is inactive!")),
        )
        .await;
}

async fn get_current_timestamp() -> u64 {
    let blockchain = GatewayHttpProxy::new(CHAIN_SIMULATOR_GATEWAY.to_string());

    let network_config = blockchain
        .http_request(NetworkStatusRequest::default())
        .await
        .unwrap();
    network_config.current_block_timestamp
}
