use multiversx_sc_scenario::imports::*;

use adder::*;

const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const ADDER_ADDRESS: TestSCAddress = TestSCAddress::new("adder");
const CODE_PATH: MxscPath = MxscPath::new("output/adder.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new().executor_config(ExecutorConfig::full_suite());

    blockchain.set_current_dir_from_workspace("contracts/examples/adder");
    blockchain.register_contract(CODE_PATH, adder::ContractBuilder);
    blockchain
}

#[test]
fn adder_blackbox() {
    let mut world = world();

    world.start_trace();

    world.account(OWNER_ADDRESS).nonce(1);

    let new_address = world
        .tx()
        .id("adder deploy")
        .from(OWNER_ADDRESS)
        .typed(adder_proxy::AdderProxy)
        .init(5u32)
        .code(CODE_PATH)
        .new_address(ADDER_ADDRESS)
        .returns(ReturnsNewAddress)
        .run();

    assert_eq!(new_address, ADDER_ADDRESS);

    world
        .query()
        .to(ADDER_ADDRESS)
        .typed(adder_proxy::AdderProxy)
        .sum()
        .returns(ExpectValue(5u32))
        .run();

    world
        .tx()
        .id("add 1")
        .from(OWNER_ADDRESS)
        .to(ADDER_ADDRESS)
        .typed(adder_proxy::AdderProxy)
        .add(1u32)
        .run();

    world
        .query()
        .to(ADDER_ADDRESS)
        .typed(adder_proxy::AdderProxy)
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
        .typed(adder_proxy::AdderProxy)
        .upgrade(100u64)
        .code(CODE_PATH)
        .run();

    world
        .check_account(ADDER_ADDRESS)
        .check_storage("str:sum", "100");

    world.write_scenario_trace("adder_trace1.scen.json");
}
