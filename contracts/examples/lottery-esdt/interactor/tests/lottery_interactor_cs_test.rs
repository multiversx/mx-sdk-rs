use lottery_interactor::{Config, LotteryInteract};
use multiversx_sc_snippets::{imports::*, sdk::gateway::NetworkStatusRequest};
use serial_test::serial;

pub const CHAIN_SIMULATOR_GATEWAY: &str = "http://localhost:8085";
const TEN_MINUTES_IN_SECONDS: u64 = 60 * 10;

#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn set_state_from_file_cs_test() {
    let mut interact = LotteryInteract::new(Config::chain_simulator_config()).await;

    interact.deploy().await;
    let current_timestamp = get_current_timestamp().await;

    interact
        .create_lottery_pool(
            &"lottery_name".to_string(),
            &"LOTTERY-123456".to_string(),
            100u128,
            None,
            Some(current_timestamp + TEN_MINUTES_IN_SECONDS),
            None,
            None,
            None,
            OptionalValue::None,
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

