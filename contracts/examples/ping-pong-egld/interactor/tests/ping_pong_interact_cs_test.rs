use multiversx_sc_snippets::imports::*;
use ping_pong_egld_interact::{Config, PingPongEgldInteract};
use serial_test::serial;

fn chain_simulator_config() -> Config {
    Config {
        connection: ConnectionConfig::chain_simulator(),
        owner: WalletConfig::from_test_wallet("simon"),
        wallet: WalletConfig::from_test_wallet("siobhan"),
    }
}

async fn cs_interactor() -> PingPongEgldInteract {
    let config = chain_simulator_config();
    let interactor = Interactor::empty()
        .with_current_dir(env!("CARGO_MANIFEST_DIR"))
        .use_chain_simulator(true)
        .with_config(&config)
        .await;
    PingPongEgldInteract {
        interactor,
        config,
        state: AutoSave::no_save_default(),
    }
}

#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test_ping_unmatched_amount() {
    let mut interact = cs_interactor().await;
    let wallet_address = interact.config.wallet.address();

    let ping_amount = 1u64;
    let duration = DurationMillis::new(5000u64);
    let activation_timestamp = TimestampMillis::new(2000u64);
    let max_funds = 100_000u64;

    interact
        .deploy(
            ping_amount.into(),
            duration,
            Some(activation_timestamp),
            OptionalValue::Some(max_funds.into()),
        )
        .await;

    let err = interact.ping(&wallet_address, 0u64).await.unwrap_err();
    assert_eq!(err.message, "the payment must match the fixed sum");
}

#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test_ping_inactive_contract() {
    let mut interact = cs_interactor().await;
    let wallet_address = interact.config.wallet.address();

    let ping_amount = 1u64;
    let duration = DurationMillis::new(5000);
    let activation_timestamp = TimestampMillis::new(2_000_000_000_000);
    let max_funds = 100_000u64;

    interact
        .deploy(
            ping_amount.into(),
            duration,
            Some(activation_timestamp),
            OptionalValue::Some(max_funds.into()),
        )
        .await;

    let err = interact.ping(&wallet_address, 1u64).await.unwrap_err();
    assert_eq!(err.message, "smart contract not active yet");
}

#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test_ping_passed_deadline() {
    let mut interact = cs_interactor().await;
    let wallet_address = interact.config.wallet.address();

    let ping_amount = 1u64;
    let duration = DurationMillis::new(5000);
    let activation_timestamp = TimestampMillis::new(2_000);
    let max_funds = 100_000u64;

    interact
        .deploy(
            ping_amount.into(),
            duration,
            Some(activation_timestamp),
            OptionalValue::Some(max_funds.into()),
        )
        .await;

    let err = interact.ping(&wallet_address, 1u64).await.unwrap_err();
    assert_eq!(err.message, "deadline has passed");
}

#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test_ping_max_funds() {
    let mut interact = cs_interactor().await;
    let wallet_address = interact.config.wallet.address();

    let ping_amount = 10u64;
    let duration = DurationMillis::new(30_000_000);
    let max_funds = 10u64;

    interact
        .deploy(
            ping_amount.into(),
            duration,
            None,
            OptionalValue::Some(max_funds.into()),
        )
        .await;

    let err = interact.ping(&wallet_address, 10u64).await.unwrap_err();
    assert_eq!(err.message, "smart contract full");
}

#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test_ping_twice() {
    let mut interact = cs_interactor().await;
    let wallet_address = interact.config.wallet.address();

    let ping_amount = 1u64;
    let duration = DurationMillis::new(20_000_000);
    let max_funds = 100_000u64;

    interact
        .deploy(
            ping_amount.into(),
            duration,
            None,
            OptionalValue::Some(max_funds.into()),
        )
        .await;

    interact.ping(&wallet_address, 1u64).await.unwrap();
    let err = interact.ping(&wallet_address, 1u64).await.unwrap_err();
    assert_eq!(err.message, "can only ping once");
}

#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test_pong_before_deadline() {
    let mut interact = cs_interactor().await;
    let wallet_address = interact.config.wallet.address();

    let ping_amount = 1u64;
    let duration = DurationMillis::new(20_000_000);
    let max_funds = 100_000u64;

    interact
        .deploy(
            ping_amount.into(),
            duration,
            None,
            OptionalValue::Some(max_funds.into()),
        )
        .await;

    interact.ping(&wallet_address, 1u64).await.unwrap();

    assert_eq!(interact.get_ping_amount().await, RustBigUint::from(1u64));

    let err = interact.pong(&wallet_address).await.unwrap_err();
    assert_eq!(err.message, "can't withdraw before deadline");
}

#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test_pong_all() {
    let mut interact = cs_interactor().await;
    let wallet_address = interact.config.wallet.address();
    let ping_pong_owner_address = interact.config.owner.address();

    let ping_amount = 1u64;
    let duration = DurationMillis::new(18_000);
    let max_funds = 100_000u64;

    interact
        .deploy(
            ping_amount.into(),
            duration,
            None,
            OptionalValue::Some(max_funds.into()),
        )
        .await;

    interact.ping(&ping_pong_owner_address, 1u64).await.unwrap();
    interact.ping(&wallet_address, 1u64).await.unwrap();

    interact.pong_all(&ping_pong_owner_address).await.unwrap();
    let err = interact.pong(&wallet_address).await.unwrap_err();
    assert_eq!(err.message, "already withdrawn");
}
