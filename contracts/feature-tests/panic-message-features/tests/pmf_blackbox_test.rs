mod pmf_proxy;

use multiversx_sc_scenario::imports::*;

const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const SC_PMF: TestSCAddress = TestSCAddress::new("pmf");
const CODE_EXPR: &str = "mxsc:output/panic-message-features.mxsc.json";

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new().executor_config(ScenarioExecutorConfig::Experimental);

    blockchain.set_current_dir_from_workspace("contracts/feature-tests/panic-message-features");
    blockchain.register_contract(CODE_EXPR, panic_message_features::ContractBuilder);
    blockchain
}

fn setup() -> ScenarioWorld {
    let mut world = world();
    let code = world.code_expression(CODE_EXPR);

    world.account(OWNER_ADDRESS).nonce(1);
    world.account(SC_PMF).code(code);

    world
}

// TODO: move to basic-features a testing framework tester
#[test]
fn tx_returns_error_test() {
    let mut world = setup();

    let (status, message) = world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_PMF)
        .typed(pmf_proxy::PanicMessageFeaturesProxy)
        .sc_panic()
        .returns(ReturnsStatus)
        .returns(ReturnsMessage)
        .run();

    assert_eq!(status, 4);
    assert_eq!(message, "sc_panic! test");
}

#[test]
fn query_returns_error_test() {
    let mut world = setup();

    let (status, message) = world
        .query()
        .to(SC_PMF)
        .typed(pmf_proxy::PanicMessageFeaturesProxy)
        .sc_panic()
        .returns(ReturnsStatus)
        .returns(ReturnsMessage)
        .run();

    assert_eq!(status, 4);
    assert_eq!(message, "sc_panic! test");
}

#[test]
fn tx_expect_error_test() {
    let mut world = setup();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_PMF)
        .typed(pmf_proxy::PanicMessageFeaturesProxy)
        .sc_panic()
        .with_result(ExpectMessage("sc_panic! test"))
        .with_result(ExpectError(4, "sc_panic! test"))
        .with_result(ExpectStatus(4))
        .run();
}

#[test]
fn query_expect_error_test() {
    let mut world = setup();

    world
        .query()
        .to(SC_PMF)
        .typed(pmf_proxy::PanicMessageFeaturesProxy)
        .sc_panic()
        .with_result(ExpectStatus(4))
        .with_result(ExpectMessage("sc_panic! test"))
        .with_result(ExpectError(4, "sc_panic! test"))
        .run();
}
