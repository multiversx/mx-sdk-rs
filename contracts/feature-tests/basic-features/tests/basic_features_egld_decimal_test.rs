use imports::{MxscPath, ReturnsResult, TestAddress, TestSCAddress};
use multiversx_sc::{
    typenum::U18,
    types::{BigUint, ConstDecimals, ManagedDecimal},
};
use multiversx_sc_scenario::{api::StaticApi, imports, ScenarioTxRun, ScenarioWorld};

const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const BASIC_FEATURES_ADDRESS: TestSCAddress = TestSCAddress::new("basic-features");
const BASIC_FEATURES_PATH: MxscPath = MxscPath::new("output/basic-features.mxsc.json");

struct BasicFeaturesState {
    world: ScenarioWorld,
}

impl BasicFeaturesState {
    fn new() -> Self {
        let mut world = world();
        let basic_features_code =
            world.code_expression(BASIC_FEATURES_PATH.eval_to_expr().as_str());

        world.account(OWNER_ADDRESS).nonce(1).balance(100);
        world
            .account(BASIC_FEATURES_ADDRESS)
            .nonce(1)
            .code(basic_features_code);

        Self { world }
    }
}
fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.set_current_dir_from_workspace("contracts/feature-tests/basic-features");
    blockchain.register_contract(BASIC_FEATURES_PATH, basic_features::ContractBuilder);
    blockchain
}

#[test]
fn egld_decimal_blackbox_test() {
    let mut state = BasicFeaturesState::new();

    let egld_decimal_result = state
        .world
        .tx()
        .from(OWNER_ADDRESS)
        .to(BASIC_FEATURES_ADDRESS)
        .typed(basic_features::basic_features_proxy::BasicFeaturesProxy)
        .returns_egld_decimal()
        .egld(5)
        .returns(ReturnsResult)
        .run();

    assert_eq!(
        egld_decimal_result,
        ManagedDecimal::<StaticApi, ConstDecimals<U18>>::const_decimals_from_raw(BigUint::from(
            5u64
        ))
    );
}
