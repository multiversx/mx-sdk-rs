mod af_proxy;
use multiversx_sc::types::{TestAddress, TestSCAddress};
use multiversx_sc_scenario::{
    imports::ScenarioExecutorConfig, ExpectMessage, ScenarioTxRun, ScenarioWorld,
};

const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const CODE_EXPR: &str = "mxsc:output/alloc-features.mxsc.json";
const SC_AF: TestSCAddress = TestSCAddress::new("alloc-features");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new().executor_config(
        ScenarioExecutorConfig::Debugger.then(ScenarioExecutorConfig::Experimental),
    );

    blockchain.set_current_dir_from_workspace("contracts/feature-tests/alloc-features");
    blockchain.register_contract(CODE_EXPR, alloc_features::ContractBuilder);
    blockchain
}

/// Likely to be removed soon.
#[test]
fn test_sc_error() {
    let mut world = world();
    let code = world.code_expression(CODE_EXPR);

    world.account(OWNER_ADDRESS).nonce(1);
    world.account(SC_AF).code(code);
    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_AF)
        .typed(af_proxy::AllocFeaturesProxy)
        .return_sc_error()
        .returns(ExpectMessage("return_sc_error"))
        .run();
}
