use delegation_sc_interact::DelegateCallsInteract;
use governance_sc_interact::{Config, GovernanceCallsInteract};
use multiversx_sc_snippets::imports::{BLSSignature, Validator};

#[tokio::test]
#[ignore = "configurable chain-simulator is not available in CI"]
async fn cs_builtin_run_tests() {
    let mut delegation_interactor =
        DelegateCallsInteract::new(delegation_sc_interact::Config::chain_simulator_config()).await;
    let validator_1 =
        Validator::from_pem_file("../interactor-delegation-func-calls/validatorKey1.pem")
            .expect("unable to load validator key");
    let validator_2 =
        Validator::from_pem_file("../interactor-delegation-func-calls/validatorKey2.pem")
            .expect("unable to load validator key");

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

    delegation_interactor
        .set_state(&delegation_interactor.owner.to_address())
        .await;
    delegation_interactor
        .set_state(&delegation_interactor.delegator1.to_address())
        .await;
    delegation_interactor
        .set_state(&delegation_interactor.delegator2.to_address())
        .await;
    delegation_interactor
        .create_new_delegation_contract(51_000_000_000_000_000_000_000_u128, 3745u64)
        .await;

    delegation_interactor
        .add_nodes(vec![(
            validator_1.public_key,
            BLSSignature::dummy("signed1"),
        )])
        .await;

    let delegator1 = delegation_interactor.delegator1.clone();
    delegation_interactor
        .delegate(&delegator1, 1_250_000_000_000_000_000_000u128)
        .await;

    let mut governance_interactor =
        GovernanceCallsInteract::new(Config::chain_simulator_config()).await;
    governance_interactor
        .set_state(&governance_interactor.owner.to_address())
        .await;

    let _ = governance_interactor
        .interactor
        .generate_blocks_until_epoch(8)
        .await;

    governance_interactor
        .proposal("6db132d759482f9f3515fe3ca8f72a8d6dc61244", 9, 11)
        .await;

    // governance_interactor
    //     .vote(&delegation_interactor.delegator1, 2, "yes")
    //     .await;
}
