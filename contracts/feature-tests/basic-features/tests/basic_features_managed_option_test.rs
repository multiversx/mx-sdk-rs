use imports::{MxscPath, ReturnsResult, TestAddress, TestSCAddress};
use multiversx_sc::types::{BigUint, ManagedOption};
use multiversx_sc_scenario::{api::StaticApi, imports, ScenarioTxRun, ScenarioWorld};

const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const BASIC_FEATURES_ADDRESS: TestSCAddress = TestSCAddress::new("basic-features");
const BASIC_FEATURES_PATH: MxscPath = MxscPath::new("output/basic-features.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(BASIC_FEATURES_PATH, basic_features::ContractBuilder);

    blockchain.account(OWNER_ADDRESS).nonce(1);
    blockchain
        .account(BASIC_FEATURES_ADDRESS)
        .nonce(1)
        .code(BASIC_FEATURES_PATH);

    blockchain
}

#[test]
fn managed_option_test() {
    let mut world = world();

    let type_number: BigUint<StaticApi> = BigUint::zero();
    let expected_type_managed_option: ManagedOption<StaticApi, BigUint<StaticApi>> =
        ManagedOption::some(type_number);

    let output = world
        .tx()
        .from(OWNER_ADDRESS)
        .to(BASIC_FEATURES_ADDRESS)
        .typed(basic_features::basic_features_proxy::BasicFeaturesProxy)
        .echo_managed_option(expected_type_managed_option.clone())
        .returns(ReturnsResult)
        .run();

    assert_eq!(output, expected_type_managed_option);
}
