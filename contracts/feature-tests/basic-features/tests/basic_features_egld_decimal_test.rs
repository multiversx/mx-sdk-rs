use multiversx_sc::types::{
    BigUint, ConstDecimals, ContractCallWithEgld, ManagedAddress, ManagedDecimal,
};
use multiversx_sc_scenario::{api::StaticApi, scenario_model::*, *};

const BASIC_FEATURES_PATH_EXPR: &str = "file:../output/basic-features.wasm";
const OWNER_ADDRESS_EXPR: &str = "address:owner";
const BASIC_FEATURES_ADDRESS_EXPR: &str = "sc:basic-features";

type BasicFeatures = ContractInfo<basic_features::Proxy<StaticApi>>;

struct BasicFeaturesState {
    world: ScenarioWorld,
    basic_features: BasicFeatures,
}

impl BasicFeaturesState {
    fn new() -> Self {
        let mut world = world();
        let basic_features_code = world.code_expression(BASIC_FEATURES_PATH_EXPR);

        world.set_state_step(
            SetStateStep::new()
                .put_account(
                    OWNER_ADDRESS_EXPR,
                    Account::new().nonce(1).balance(BigUintValue::from(100u64)),
                )
                .put_account(
                    BASIC_FEATURES_ADDRESS_EXPR,
                    Account::new().nonce(1).code(basic_features_code),
                ),
        );

        let basic_features = BasicFeatures::new(BASIC_FEATURES_ADDRESS_EXPR);

        Self {
            world,
            basic_features,
        }
    }
}
fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/basic-features");

    blockchain.register_contract(BASIC_FEATURES_PATH_EXPR, basic_features::ContractBuilder);
    blockchain
}

#[test]
fn egld_decimal_blackbox_test() {
    let mut state = BasicFeaturesState::new();

    let sc_call =
        ContractCallWithEgld::<StaticApi, ManagedDecimal<StaticApi, ConstDecimals<18>>>::new(
            ManagedAddress::from(state.basic_features.to_address()),
            "returns_egld_decimal",
            BigUint::from(5u64),
        );

    let egld_decimal: ManagedDecimal<StaticApi, ConstDecimals<18>> = state
        .world
        .sc_call_get_result(ScCallStep::new().call(sc_call).from("address:owner"));

    assert_eq!(
        egld_decimal,
        ManagedDecimal::<StaticApi, ConstDecimals<18>>::const_decimals_from_raw(BigUint::from(
            5u64
        ))
    );
}
