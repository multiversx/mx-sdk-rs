use delegation_sc_interact::DelegateCallsInteract;
use governance_sc_interact::{Config, GovernanceCallsInteract};
use multiversx_sc_snippets::imports::{BLSSignature, Validator};

#[tokio::test]
#[ignore = "configurable chain-simulator is not available in CI"]
async fn cs_governance_run_tests() {
    let mut governance_interactor =
        GovernanceCallsInteract::new(Config::chain_simulator_config()).await;
    governance_interactor
        .set_state(&governance_interactor.owner.to_address())
        .await;
    governance_interactor
        .set_state(&governance_interactor.user1.to_address())
        .await;
    governance_interactor
        .set_state(&governance_interactor.delegator.to_address())
        .await;

    let _ = governance_interactor
        .interactor
        .generate_blocks_until_epoch(8)
        .await;

    governance_interactor
        .proposal(
            &governance_interactor.owner.to_address(),
            "6db132d759482f9f3515fe3ca8f72a8d6dc61244",
            9,
            11,
        )
        .await;

    governance_interactor.view_config().await;
    governance_interactor.view_proposal(1).await;

    let mut delegation_interactor =
        DelegateCallsInteract::new(delegation_sc_interact::Config::chain_simulator_config()).await;
    let validator_1 =
        Validator::from_pem_file("../interactor-delegation-func-calls/validatorKey1.pem")
            .expect("unable to load validator key");
    let validator_2 =
        Validator::from_pem_file("../interactor-delegation-func-calls/validatorKey2.pem")
            .expect("unable to load validator key");
    let validator_3 =
        Validator::from_pem_file("./validatorKey3.pem").expect("unable to load validator key");

    let _ = delegation_interactor
        .interactor
        .add_key(validator_1.private_key.clone())
        .await
        .unwrap();
    let _ = delegation_interactor
        .interactor
        .add_key(validator_2.private_key.clone())
        .await
        .unwrap();
    let _ = delegation_interactor
        .interactor
        .add_key(validator_3.private_key.clone())
        .await
        .unwrap();

    delegation_interactor
        .set_state(&delegation_interactor.owner.to_address())
        .await;
    delegation_interactor
        .set_state(&delegation_interactor.delegator1.to_address())
        .await;
    delegation_interactor
        .set_state(&delegation_interactor.delegator2.to_address())
        .await;

    governance_interactor
        .stake(
            governance_interactor.owner.clone(),
            1,
            vec![(validator_1.public_key, BLSSignature::dummy("signed1"))],
            30_000_000_000_000_000_000_000u128,
        )
        .await;

    governance_interactor
        .stake(
            governance_interactor.user1.clone(),
            1,
            vec![(validator_2.public_key, BLSSignature::dummy("signed2"))],
            40_000_000_000_000_000_000_000u128,
        )
        .await;

    delegation_interactor
        .set_state(&delegation_interactor.owner.to_address())
        .await;

    delegation_interactor
        .create_new_delegation_contract(0u128, 0u64, 40_000_000_000_000_000_000_000_u128)
        .await;

    delegation_interactor
        .add_nodes(vec![(
            validator_3.public_key,
            BLSSignature::dummy("signed3"),
        )])
        .await;

    delegation_interactor
        .stake_nodes(vec![validator_3.public_key])
        .await;

    governance_interactor
        .delegate_vote(
            &governance_interactor.delegator.clone(),
            1,
            "yes",
            &governance_interactor.user2.clone(),
            40000,
            Some("user error"),
        )
        .await;

    delegation_interactor
        .delegate(
            &delegation_interactor.delegator1.clone(),
            50_000_000_000_000_000_000_000u128,
        )
        .await;

    let _ = governance_interactor
        .interactor
        .generate_blocks_until_epoch(10)
        .await;

    governance_interactor
        .vote(&governance_interactor.owner.clone(), 1, "yes")
        .await;
}
