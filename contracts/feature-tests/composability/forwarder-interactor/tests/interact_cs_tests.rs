use forwarder_interact::{Config, ContractInteract};
use multiversx_sc_snippets::imports::*;

const MOCK_AMOUNT: u64 = 1_000u64;
const MOCK_TOKEN_DISPLAY_NAME: &[u8] = b"TESTTKN";
const MOCK_TOKEN_TICKER: &[u8] = b"TEST";
const ONE_EGLD: u64 = 1_000_000_000_000_000_000u64;

// Simple deploy test that runs using the chain simulator configuration.
// In order for this test to work, make sure that the `config.toml` file contains the chain simulator config (or choose it manually)
// The chain simulator should already be installed and running before attempting to run this test.
// The chain-simulator-tests feature should be present in Cargo.toml.
// Can be run with `sc-meta test -c`.
#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn deploy_test_forwarder_cs() {
    let mut interactor = ContractInteract::new(Config::chain_simulator_config(), None).await;

    interactor.deploy().await;
}

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn forward_send_async_reject_multi_transfer() {
    let mut interact = ContractInteract::new(Config::chain_simulator_config(), None).await;

    // deploy fwd
    let (forwarder_address, _) = interact.deploy().await;

    // deploy vault
    let (vault_address, _) = interact.deploy_vault().await;

    // mint tokens
    let (token_id, _) = interact
        .issue_fungible_token_from_wallet(MOCK_TOKEN_DISPLAY_NAME, MOCK_TOKEN_TICKER, MOCK_AMOUNT)
        .await;

    // build token payment
    let egld_payment = EgldOrEsdtTokenPayment::egld_payment(BigUint::from(ONE_EGLD));
    let esdt_payment = EgldOrEsdtTokenPayment::new(
        EgldOrEsdtTokenIdentifier::esdt(token_id.as_bytes()),
        0u64,
        BigUint::from(MOCK_AMOUNT),
    );

    let vec_of_payments = vec![egld_payment, esdt_payment];

    // send call
    interact
        .forward_send_async_reject_multi_transfer(vault_address, vec_of_payments)
        .await;

    // verify balance
    let fwd_account = interact
        .interactor
        .get_account(forwarder_address.as_address())
        .await;
    assert!(fwd_account.balance == ONE_EGLD.to_string());

    let esdts = interact
        .interactor
        .get_account_esdt(forwarder_address.as_address())
        .await;

    assert!(esdts.contains_key(&token_id));
    assert_eq!(
        esdts.get(&token_id).unwrap().balance,
        MOCK_AMOUNT.to_string()
    );
}

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn forward_transf_exec_reject_multi_transfer() {
    let mut interact = ContractInteract::new(Config::chain_simulator_config(), None).await;

    let alice = interact.wallet_address.clone();
    let initial_alice_balance = interact.interactor.get_account(&alice).await.balance;
    let mut gas_used = 0u64;

    // deploy fwd
    gas_used += interact.deploy().await.1;

    // deploy vault
    let (vault_address, v_gas_used) = interact.deploy_vault().await;
    gas_used += v_gas_used;

    // mint tokens
    let (token_id, t_gas_used) = interact
        .issue_fungible_token_from_wallet(MOCK_TOKEN_DISPLAY_NAME, MOCK_TOKEN_TICKER, MOCK_AMOUNT)
        .await;
    gas_used += t_gas_used;

    // build token payment
    let egld_payment = EgldOrEsdtTokenPayment::egld_payment(BigUint::from(ONE_EGLD));
    let esdt_payment = EgldOrEsdtTokenPayment::new(
        EgldOrEsdtTokenIdentifier::esdt(token_id.as_bytes()),
        0u64,
        BigUint::from(MOCK_AMOUNT),
    );

    let vec_of_payments = vec![egld_payment, esdt_payment];

    // send call
    gas_used += interact
        .transf_exec_multi_reject_funds(vault_address, vec_of_payments)
        .await;

    // verify balance in wallet
    let current_alice_balance = interact.interactor.get_account(&alice).await.balance;

    // current = initial - gas_used
    // 0.240244346985 ????
    let current_balance: u128 = current_alice_balance.parse().unwrap();
    let initial_balance: u128 = initial_alice_balance.parse().unwrap();

    assert_eq!(current_balance, initial_balance - gas_used as u128);

    let esdts = interact.interactor.get_account_esdt(&alice).await;

    assert!(esdts.contains_key(&token_id));
    assert_eq!(
        esdts.get(&token_id).unwrap().balance,
        MOCK_AMOUNT.to_string()
    );
}
