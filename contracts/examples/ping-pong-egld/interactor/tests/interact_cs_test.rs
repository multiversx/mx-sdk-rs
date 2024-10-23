use multiversx_sc_snippets::imports::RustBigUint;
use ping_pong_egld_interact::{Config, PingPongEgldInteract};

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test_ping_unmatched_amount() {
    let mut interact = PingPongEgldInteract::init(Config::chain_simulator_config()).await;

    let ping_amount = 1u64;
    let duration_in_seconds = 5u64;
    let opt_activation_timestamp = 2u64;
    let max_funds = 100_000u64;

    interact
        .deploy(
            ping_amount.into(),
            duration_in_seconds,
            Some(opt_activation_timestamp),
            multiversx_sc_snippets::imports::OptionalValue::Some(max_funds.into()),
        )
        .await;

    interact
        .ping(0u64, Some("the payment must match the fixed sum"), None)
        .await;
}

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test_ping_inactive_contracts() {
    let mut interact = PingPongEgldInteract::init(Config::chain_simulator_config()).await;

    let ping_amount = 1u64;
    let duration_in_seconds = 5u64;
    let opt_activation_timestamp = 2_000_000_000u64;
    let max_funds = 100_000u64;

    interact
        .deploy(
            ping_amount.into(),
            duration_in_seconds,
            Some(opt_activation_timestamp),
            multiversx_sc_snippets::imports::OptionalValue::Some(max_funds.into()),
        )
        .await;

    interact
        .ping(1u64, Some("smart contract not active yet"), None)
        .await;
}

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test_ping_passed_deadline() {
    let mut interact = PingPongEgldInteract::init(Config::chain_simulator_config()).await;

    let ping_amount = 1u64;
    let duration_in_seconds = 5u64;
    let opt_activation_timestamp = 2u64;
    let max_funds = 100_000u64;

    interact
        .deploy(
            ping_amount.into(),
            duration_in_seconds,
            Some(opt_activation_timestamp),
            multiversx_sc_snippets::imports::OptionalValue::Some(max_funds.into()),
        )
        .await;

    interact.ping(1u64, Some("deadline has passed"), None).await;
}

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test_ping_max_funds() {
    let mut interact = PingPongEgldInteract::init(Config::chain_simulator_config()).await;

    let ping_amount = 10u64;
    let duration_in_seconds = 30u64;
    let max_funds = 10u64;

    interact
        .deploy(
            ping_amount.into(),
            duration_in_seconds,
            None,
            multiversx_sc_snippets::imports::OptionalValue::Some(max_funds.into()),
        )
        .await;

    interact
        .ping(10u64, Some("smart contract full"), None)
        .await;
}

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test_ping() {
    let mut interact = PingPongEgldInteract::init(Config::chain_simulator_config()).await;

    let ping_amount = 1u64;
    let duration_in_seconds = 100u64;
    let max_funds = 100_000u64;

    interact
        .deploy(
            ping_amount.into(),
            duration_in_seconds,
            None,
            multiversx_sc_snippets::imports::OptionalValue::Some(max_funds.into()),
        )
        .await;

    interact.ping(1u64, None, None).await;
    interact.ping(1u64, Some("can only ping once"), None).await;

    assert_eq!(interact.get_ping_amount().await, RustBigUint::from(1u64));

    interact
        .pong(Some("can't withdraw before deadline"), None)
        .await;

    interact.pong(None, None).await;
}

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test_pong_all() {
    let mut interact = PingPongEgldInteract::init(Config::chain_simulator_config()).await;

    let ping_amount = 1u64;
    let duration_in_seconds = 50u64;
    let max_funds = 100_000u64;

    interact
        .deploy(
            ping_amount.into(),
            duration_in_seconds,
            None,
            multiversx_sc_snippets::imports::OptionalValue::Some(max_funds.into()),
        )
        .await;

    interact
        .ping(1u64, None, Some(&interact.wallet_address_2.clone()))
        .await;

    interact
        .ping(1u64, None, Some(&interact.wallet_address_1.clone()))
        .await;

    interact.pong_all(None, None).await;
    interact
        .pong(
            Some("already withdrawn"),
            Some(&interact.wallet_address_1.clone()),
        )
        .await;
}
