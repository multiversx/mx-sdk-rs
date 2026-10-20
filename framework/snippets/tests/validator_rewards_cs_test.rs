use multiversx_sc_snippets::imports::*;
use serial_test::serial;

/// 1 EGLD.
const ONE_EGLD: u64 = 1_000_000_000_000_000_000;

async fn cs_interactor() -> HttpInteractor {
    HttpInteractor::empty()
        .with_current_dir(env!("CARGO_MANIFEST_DIR"))
        .with_config(&ConnectionConfig::chain_simulator())
        .await
}

async fn balance_of(interactor: &HttpInteractor, address: &Bech32Address) -> RustBigUint {
    interactor
        .get_account(address.as_address())
        .await
        .balance
        .parse()
        .expect("unexpected balance value from gateway")
}

#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn inject_validator_reward_cs_test() {
    let mut interactor = cs_interactor().await;
    let address = interactor
        .register_wallet_bech32(test_wallets::judy())
        .await;

    // Spend a transaction first, so the account has a non-zero nonce and the test can
    // tell whether set-state resets it.
    interactor
        .tx()
        .from(&address)
        .to(&address)
        .egld(BigUint::<StaticApi>::from(1u32))
        .gas(50_000u64)
        .run()
        .await;

    let nonce_before = interactor.get_account(address.as_address()).await.nonce;
    assert!(nonce_before > 0, "the account should have spent a nonce");

    let rewards_before = interactor.get_cumulated_validator_rewards(&address).await;
    let balance_before = balance_of(&interactor, &address).await;

    let reward = RustBigUint::from(ONE_EGLD);
    interactor
        .inject_validator_reward(&address, &reward)
        .await
        .expect("injecting a validator reward failed");

    // The counter the contracts read, and the balance, both move by the reward.
    assert_eq!(
        interactor.get_cumulated_validator_rewards(&address).await,
        &rewards_before + &reward
    );
    assert_eq!(
        balance_of(&interactor, &address).await,
        &balance_before + &reward
    );

    // The nonce survives, otherwise the account could no longer send transactions.
    assert_eq!(
        interactor.get_account(address.as_address()).await.nonce,
        nonce_before
    );

    // A second reward accumulates, rather than overwriting the first.
    interactor
        .inject_validator_reward(&address, &reward)
        .await
        .expect("injecting a second validator reward failed");

    assert_eq!(
        interactor.get_cumulated_validator_rewards(&address).await,
        rewards_before + reward * 2u32
    );
}

#[tokio::test]
#[serial]
async fn inject_validator_reward_without_simulator_test() {
    // Not connected to a simulator: the call is skipped, like the other chain-simulator
    // operations, rather than corrupting a real chain's expectations.
    let interactor = HttpInteractor::empty().use_chain_simulator(false);

    let address = Bech32Address::from(&test_wallets::judy().to_address());
    let response = interactor
        .inject_validator_reward(&address, &RustBigUint::from(ONE_EGLD))
        .await;

    assert_eq!(response.unwrap(), "no-simulator");
}
