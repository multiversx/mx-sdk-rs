use multiversx_sc_scenario::imports::*;

use adder::*;

const ADDER_PATH_EXPR: &str = "mxsc:output/adder.mxsc.json";

const OWNER: AddressExpr = AddressExpr("owner");
const SC_ADDER: ScExpr = ScExpr("adder");
const CODE_EXPR: MxscExpr = MxscExpr("output/adder.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/adder");

    blockchain.register_contract(ADDER_PATH_EXPR, adder::ContractBuilder);
    blockchain
}

#[test]
fn adder_blackbox() {
    let mut world = world();

    world.start_trace();

    world.account(OWNER).nonce(1);

    world.new_address(OWNER, 1, SC_ADDER);

    let new_address = world
        .tx()
        .from(OWNER)
        .typed(adder_proxy::AdderProxy)
        .init(5u32)
        .code(CODE_EXPR)
        .returns(ReturnsNewAddress)
        .run();

    assert_eq!(new_address, SC_ADDER.to_address());

    world
        .query()
        .to(SC_ADDER)
        .typed(adder_proxy::AdderProxy)
        .sum()
        .returns(ExpectValue(5u32))
        .run();

    world
        .tx()
        .from(OWNER)
        .to(SC_ADDER)
        .typed(adder_proxy::AdderProxy)
        .add(1u32)
        .run();

    world
        .query()
        .to(SC_ADDER)
        .typed(adder_proxy::AdderProxy)
        .sum()
        .returns(ExpectValue(6u32))
        .run();

    world.check_account(OWNER);

    world.check_account(SC_ADDER).check_storage("str:sum", "6");

    world.write_scenario_trace("trace1.scen.json");
}
