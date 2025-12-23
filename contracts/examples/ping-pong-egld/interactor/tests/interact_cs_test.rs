use multiversx_sc_snippets::imports::{DurationMillis, RustBigUint, TimestampMillis};
use ping_pong_egld_interact::{Config, PingPongEgldInteract};

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test_ping_pong_egld() {
    let mut interact = PingPongEgldInteract::init(Config::chain_simulator_config()).await;
    let wallet_address = interact.wallet_address.clone();
    let ping_pong_owner_address = interact.ping_pong_owner_address.clone();

    let ping_amount = 1u64;

    // test_ping_unmatched_amount
    let duration = DurationMillis::new(5000u64);
    let activation_timestamp = TimestampMillis::new(2000u64);
    let max_funds = 100_000u64;

    interact
        .deploy(
            ping_amount.into(),
            duration,
            Some(activation_timestamp),
            multiversx_sc_snippets::imports::OptionalValue::Some(max_funds.into()),
        )
        .await;

    interact
        .ping(
            0u64,
            Some("the payment must match the fixed sum"),
            &wallet_address,
        )
        .await;

    // test_ping_inactive_contracts
    let duration = DurationMillis::new(5000);
    let activation_timestamp = TimestampMillis::new(2_000_000_000_000);
    let max_funds = 100_000u64;

    interact
        .deploy(
            ping_amount.into(),
            duration,
            Some(activation_timestamp),
            multiversx_sc_snippets::imports::OptionalValue::Some(max_funds.into()),
        )
        .await;

    interact
        .ping(1u64, Some("smart contract not active yet"), &wallet_address)
        .await;

    // test_ping_passed_deadline
    let duration = DurationMillis::new(5000);
    let activation_timestamp = TimestampMillis::new(2_000);
    let max_funds = 100_000u64;

    interact
        .deploy(
            ping_amount.into(),
            duration,
            Some(activation_timestamp),
            multiversx_sc_snippets::imports::OptionalValue::Some(max_funds.into()),
        )
        .await;

    interact
        .ping(1u64, Some("deadline has passed"), &wallet_address)
        .await;

    // test_ping_max_funds
    let ping_amount = 10u64;
    let duration = DurationMillis::new(30_000_000);
    let max_funds = 10u64;

    interact
        .deploy(
            ping_amount.into(),
            duration,
            None,
            multiversx_sc_snippets::imports::OptionalValue::Some(max_funds.into()),
        )
        .await;

    interact
        .ping(10u64, Some("smart contract full"), &wallet_address)
        .await;

    // test ping
    let ping_amount = 1u64;
    let duration = DurationMillis::new(20_000);
    let max_funds = 100_000u64;

    interact
        .deploy(
            ping_amount.into(),
            duration,
            None,
            multiversx_sc_snippets::imports::OptionalValue::Some(max_funds.into()),
        )
        .await;

    interact.ping(1u64, None, &wallet_address).await;
    interact
        .ping(1u64, Some("can only ping once"), &wallet_address)
        .await;

    assert_eq!(interact.get_ping_amount().await, RustBigUint::from(1u64));

    interact
        .pong(Some("can't withdraw before deadline"), &wallet_address)
        .await;

    interact.pong(None, &wallet_address).await;

    // test_pong_all
    let ping_amount = 1u64;
    let duration = DurationMillis::new(18_000);
    let max_funds = 100_000u64;

    interact
        .deploy(
            ping_amount.into(),
            duration,
            None,
            multiversx_sc_snippets::imports::OptionalValue::Some(max_funds.into()),
        )
        .await;

    interact.ping(1u64, None, &ping_pong_owner_address).await;

    interact.ping(1u64, None, &wallet_address).await;

    interact.pong_all(None, &ping_pong_owner_address).await;
    interact
        .pong(Some("already withdrawn"), &wallet_address)
        .await;
}
