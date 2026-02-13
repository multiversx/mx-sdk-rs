// Auto-generated blackbox tests from scenarios

use multiversx_sc_scenario::imports::*;

use adder::*;

const CODE_PATH: MxscPath = MxscPath::new("output/adder.mxsc.json");

const ADDRESS_HEX_1: Address = Address::from_hex("0xe32afedc904fe1939746ad973beb383563cf63642ba669b3040f9b9428a5ed60");
const ADDRESS_HEX_2: Address = Address::from_hex("0x0000000000000000050028600ceb73ac22ec0b6f257aff7bed74dffa3ebfed60");
const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const ADDER_ADDRESS: TestSCAddress = TestSCAddress::new("adder");

fn world() -> ScenarioWorld {
    todo!()
}

#[test]
fn interactor_trace_scen() {
    let mut world = world();
    interactor_trace_scen_steps(&mut world);
}

pub fn interactor_trace_scen_steps(world: &mut ScenarioWorld) {
    world.account(ADDRESS_HEX_1).nonce(ScenarioValueRaw::str("481"))
        .balance(ScenarioValueRaw::str("106274669842530000003"))
        ;


    world.tx()
        .from(ADDRESS_HEX_1)
        .typed(adder_proxy::AdderProxy)
        .init(ScenarioValueRaw::str("0x00"))
        .code(CODE_PATH)
        .new_address(ADDRESS_HEX_2)
        .run();

    world.tx()
        .from(ADDRESS_HEX_1)
        .to(ADDRESS_HEX_2)
        .typed(adder_proxy::AdderProxy)
        .add(ScenarioValueRaw::str("0x07"))
        .run();

    world.tx()
        .from(ADDRESS_HEX_1)
        .to(ADDRESS_HEX_2)
        .typed(adder_proxy::AdderProxy)
        .add(ScenarioValueRaw::str("0x05"))
        .run();

}

// add then check
#[test]
fn adder_scen() {
    let mut world = world();
    adder_scen_steps(&mut world);
}

pub fn adder_scen_steps(world: &mut ScenarioWorld) {
    world.account(OWNER_ADDRESS).nonce(ScenarioValueRaw::str("1"))
        .balance(ScenarioValueRaw::str("0"))
        ;

    world.tx()
        .id("1")
        .from(OWNER_ADDRESS)
        .typed(adder_proxy::AdderProxy)
        .init(ScenarioValueRaw::str("5"))
        .code(CODE_PATH)
        .new_address(ADDER_ADDRESS)
        .run();

    world.query()
        .id("2")
        .to(ADDER_ADDRESS)
        .typed(adder_proxy::AdderProxy)
        .sum()
        .returns(ExpectValue(ScenarioValueRaw::str("5")))
        .run();

    world.tx()
        .id("3")
        .from(OWNER_ADDRESS)
        .to(ADDER_ADDRESS)
        .typed(adder_proxy::AdderProxy)
        .add(ScenarioValueRaw::str("3"))
        .run();

    world.check_account(ADDER_ADDRESS)
        .check_storage("str:sum", "8")
        ;

}

