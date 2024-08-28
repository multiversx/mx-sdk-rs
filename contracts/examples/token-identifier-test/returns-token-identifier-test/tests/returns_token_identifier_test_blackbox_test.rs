use multiversx_sc_scenario::imports::*;

use returns_token_identifier_test::*;

const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const ADDER_ADDRESS: TestSCAddress = TestSCAddress::new("adder");
const CODE_PATH: MxscPath = MxscPath::new("output/returns-token-identifier-test.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(CODE_PATH, returns_token_identifier_test::ContractBuilder);
    blockchain
}

#[test]
fn adder_blackbox() {
    let mut world = world();

    world.start_trace();

    world.account(OWNER_ADDRESS).nonce(1);

    let new_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(returns_token_identifier_test_proxy::ReturnsTokenIdentifierTestProxy)
        .init(5u32)
        .code(CODE_PATH)
        .new_address(ADDER_ADDRESS)
        .returns(ReturnsNewAddress)
        .run();

    assert_eq!(new_address, ADDER_ADDRESS.to_address());

    world
        .query()
        .to(ADDER_ADDRESS)
        .typed(returns_token_identifier_test_proxy::ReturnsTokenIdentifierTestProxy)
        .sum()
        .returns(ExpectValue(5u32))
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(ADDER_ADDRESS)
        .typed(returns_token_identifier_test_proxy::ReturnsTokenIdentifierTestProxy)
        .add(1u32)
        .run();

    world
        .query()
        .to(ADDER_ADDRESS)
        .typed(returns_token_identifier_test_proxy::ReturnsTokenIdentifierTestProxy)
        .sum()
        .returns(ExpectValue(6u32))
        .run();

    world.check_account(OWNER_ADDRESS);

    world
        .check_account(ADDER_ADDRESS)
        .check_storage("str:sum", "6");

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(ADDER_ADDRESS)
        .typed(returns_token_identifier_test_proxy::ReturnsTokenIdentifierTestProxy)
        .upgrade(100u64)
        .code(CODE_PATH)
        .run();

    world
        .check_account(ADDER_ADDRESS)
        .check_storage("str:sum", "100");

    world.write_scenario_trace("trace1.scen.json");
}
