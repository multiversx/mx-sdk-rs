use multiversx_sc_scenario::imports::*;

use adder::*;

const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const ADDER_ADDRESS: TestSCAddress = TestSCAddress::new("adder");
const CODE_PATH: MxscPath = MxscPath::new("output/adder.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new()
        .executor_config(ScenarioExecutorConfig::Experimental)
        .gas_schedule(GasScheduleVersion::V8);

    blockchain.set_current_dir_from_workspace("contracts/examples/adder");
    blockchain.register_contract(CODE_PATH, adder::ContractBuilder);
    blockchain
}

#[test]
#[cfg_attr(not(feature = "wasmer-experimental"), ignore)]
fn adder_blackbox_wasmer_experimental() {
    let mut world = world();

    world.account(OWNER_ADDRESS).nonce(1);

    let new_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .gas(1_000_000u64)
        .typed(adder_proxy::AdderProxy)
        .init(5u32)
        .code(CODE_PATH)
        .new_address(ADDER_ADDRESS)
        .returns(ReturnsNewAddress)
        .run();

    assert_eq!(new_address, ADDER_ADDRESS.to_address());
}
