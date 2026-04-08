use std::vec;

use delegation_sc_interact::{Config, DelegateCallsInteract};
use multiversx_sc_snippets::{
    imports::{BLSSignature, RustBigUint},
    sdk::validator::Validator,
};

#[tokio::test]
#[ignore = "configurable chain-simulator is not available in CI"]
async fn cs_delegation_run_tests() {
    let mut interactor = DelegateCallsInteract::new(Config::chain_simulator_config()).await;
    let validator_1 =
        Validator::from_pem_file("./validatorKey1.pem").expect("unable to load validator key");
    let validator_2 =
        Validator::from_pem_file("./validatorKey2.pem").expect("unable to load validator key");

    let _ = interactor
        .interactor
        .add_key(validator_1.private_key.clone())
        .await
        .unwrap();
    let _ = interactor
        .interactor
        .add_key(validator_2.private_key.clone())
        .await
        .unwrap();

    interactor.set_state(&interactor.owner.to_address()).await;
    interactor
        .set_state(&interactor.delegator1.to_address())
        .await;
    interactor
        .set_state(&interactor.delegator2.to_address())
        .await;
    interactor
        .create_new_delegation_contract(0, 3745u64, 1250000000000000000000u128)
        .await;
    interactor.set_check_cap_on_redelegate_rewards(false).await;

    let addresses = interactor.get_all_contract_addresses().await;
    assert_eq!(&addresses[0], interactor.state.current_delegation_address());

    interactor
        .add_nodes(vec![
            (validator_1.public_key, BLSSignature::dummy("signed1")),
            (validator_2.public_key, BLSSignature::dummy("signed2")),
        ])
        .await;

    let state = interactor.get_all_node_states().await;
    assert_eq!(&state, "notStaked");

    let total_stake = interactor.get_total_active_stake().await;
    assert_eq!(
        total_stake,
        RustBigUint::from(1_250_000_000_000_000_000_000u128)
    );

    let user_active_stake = interactor.get_user_active_stake().await;
    assert_eq!(
        user_active_stake,
        RustBigUint::from(1_250_000_000_000_000_000_000u128)
    );

    interactor.remove_nodes(vec![validator_2.public_key]).await;

    let delegator1 = interactor.delegator1.clone();
    interactor
        .delegate(&delegator1, 1_250_000_000_000_000_000_000u128)
        .await;

    let total_stake = interactor.get_total_active_stake().await;
    assert_eq!(
        total_stake,
        RustBigUint::from(2_500_000_000_000_000_000_000u128)
    );
    let user_active_stake = interactor.get_user_active_stake().await;
    assert_eq!(
        user_active_stake,
        RustBigUint::from(1_250_000_000_000_000_000_000u128)
    );

    let delegator2 = interactor.delegator2.clone();
    interactor
        .delegate(&delegator2, 2_250_000_000_000_000_000_000u128)
        .await;

    let total_stake = interactor.get_total_active_stake().await;
    assert_eq!(
        total_stake,
        RustBigUint::from(4_750_000_000_000_000_000_000u128)
    );
    let user_active_stake = interactor.get_user_active_stake().await;
    assert_eq!(
        user_active_stake,
        RustBigUint::from(1_250_000_000_000_000_000_000u128)
    );

    let _ = interactor.interactor.generate_blocks_until_epoch(10).await;

    interactor.claim_rewards(&delegator1).await;

    interactor.stake_nodes(vec![validator_1.public_key]).await;
    let state = interactor.get_all_node_states().await;
    assert_eq!(&state, "staked");

    interactor.unstake_nodes(vec![validator_1.public_key]).await;
    interactor
        .restake_unstaked_nodes(vec![validator_1.public_key])
        .await;
    interactor.unjail_nodes(vec![validator_1.public_key]).await;

    interactor.unstake_nodes(vec![validator_1.public_key]).await;
    interactor.unbond_nodes(vec![validator_1.public_key]).await;
}
