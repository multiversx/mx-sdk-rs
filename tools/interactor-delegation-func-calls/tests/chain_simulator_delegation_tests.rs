use std::vec;

use delegation_sc_interact::{Config, DelegateCallsInteract};
use multiversx_sc_snippets::{imports::RustBigUint, sdk::validator::Validator};

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn cs_builtin_run_tests() {
    let mut interactor = DelegateCallsInteract::new(Config::chain_simulator_config()).await;

    interactor
        .set_state(&interactor.wallet_address.to_address())
        .await;
    interactor
        .set_state(&interactor.delegator1.to_address())
        .await;
    interactor
        .set_state(&interactor.delegator2.to_address())
        .await;
    interactor
        .create_new_delegation_contract(51000_000_000_000_000_000_000u128, 3745u64)
        .await;

    let addresses = interactor.get_all_contract_addresses().await;
    assert_eq!(&addresses[0], interactor.state.current_delegation_address());

    let validator_1 =
        Validator::from_pem_file("./validatorKey_46.pem").expect("unable to load validator key");

    let _ = interactor
        .interactor
        .add_key(validator_1.private_key.clone())
        .await
        .unwrap();
    interactor
        .add_nodes(vec![validator_1.public_key.clone()], vec!["signed1"])
        .await;

    let state = interactor.get_all_node_states().await;
    assert_eq!(&state, "notStaked");

    let total_stake = interactor.get_total_active_stake().await;
    assert_eq!(
        total_stake,
        RustBigUint::from(1250_000_000_000_000_000_000u128)
    );

    let delegator_contract_address = interactor.state.current_delegation_address().clone();
    let top_up = interactor
        .get_total_staked_top_up_staked_bls_keys(&delegator_contract_address)
        .await;
    assert_eq!(top_up, RustBigUint::from(1250_000_000_000_000_000_000u128));

    let user_active_stake = interactor.get_user_active_stake().await;
    assert_eq!(
        user_active_stake,
        RustBigUint::from(1250_000_000_000_000_000_000u128)
    );

    let delegator1 = interactor.delegator1.clone();
    interactor
        .delegate(&delegator1, 1250_000_000_000_000_000_000u128)
        .await;

    let top_up = interactor
        .get_total_staked_top_up_staked_bls_keys(&delegator_contract_address)
        .await;
    assert_eq!(top_up, RustBigUint::from(2500_000_000_000_000_000_000u128));

    let total_stake = interactor.get_total_active_stake().await;
    assert_eq!(
        total_stake,
        RustBigUint::from(2500_000_000_000_000_000_000u128)
    );
    let user_active_stake = interactor.get_user_active_stake().await;
    assert_eq!(
        user_active_stake,
        RustBigUint::from(1250_000_000_000_000_000_000u128)
    );

    let delegator2 = interactor.delegator2.clone();
    interactor
        .delegate(&delegator2, 1250_000_000_000_000_000_000u128)
        .await;

    let top_up = interactor
        .get_total_staked_top_up_staked_bls_keys(&delegator_contract_address)
        .await;
    assert_eq!(top_up, RustBigUint::from(3750_000_000_000_000_000_000u128));

    let total_stake = interactor.get_total_active_stake().await;
    assert_eq!(
        total_stake,
        RustBigUint::from(3750_000_000_000_000_000_000u128)
    );
    let user_active_stake = interactor.get_user_active_stake().await;
    assert_eq!(
        user_active_stake,
        RustBigUint::from(1250_000_000_000_000_000_000u128)
    );
    interactor.stake_nodes(vec![validator_1.public_key]).await;
}
