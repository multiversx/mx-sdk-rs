// Auto-generated blackbox tests from scenarios

use multiversx_sc_scenario::imports::*;

use adder::*;

const CODE_PATH: MxscPath = MxscPath::new("output/adder.mxsc.json");

fn world() -> ScenarioWorld {
    todo!()
}

#[test]
fn interactor_trace_scen() {
    let mut world = world();
    interactor_trace_scen_steps(&mut world);
}

pub fn interactor_trace_scen_steps(world: &mut ScenarioWorld) {
    world
        .account(Address::from_hex(
            "0xe32afedc904fe1939746ad973beb383563cf63642ba669b3040f9b9428a5ed60",
        ))
        .nonce(ScenarioValueRaw::str("481"))
        .balance(ScenarioValueRaw::str("106274669842530000003"));

    world
        .tx()
        .from(Address::from_hex(
            "0xe32afedc904fe1939746ad973beb383563cf63642ba669b3040f9b9428a5ed60",
        ))
        .typed(adder_proxy::AdderProxy)
        .init(ScenarioValueRaw::str("0x00"))
        .code(CODE_PATH)
        .new_address(Address::from_hex(
            "0x0000000000000000050028600ceb73ac22ec0b6f257aff7bed74dffa3ebfed60",
        ))
        .run();

    world
        .tx()
        .from(Address::from_hex(
            "0xe32afedc904fe1939746ad973beb383563cf63642ba669b3040f9b9428a5ed60",
        ))
        .to(Address::from_hex(
            "0x0000000000000000050028600ceb73ac22ec0b6f257aff7bed74dffa3ebfed60",
        ))
        .typed(adder_proxy::AdderProxy)
        .add(ScenarioValueRaw::str("0x07"))
        .run();

    world
        .tx()
        .from(Address::from_hex(
            "0xe32afedc904fe1939746ad973beb383563cf63642ba669b3040f9b9428a5ed60",
        ))
        .to(Address::from_hex(
            "0x0000000000000000050028600ceb73ac22ec0b6f257aff7bed74dffa3ebfed60",
        ))
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
    world
        .account(TestAddress::new("owner"))
        .nonce(ScenarioValueRaw::str("1"))
        .balance(ScenarioValueRaw::str("0"));

    world
        .tx()
        .id("1")
        .from(TestAddress::new("owner"))
        .typed(adder_proxy::AdderProxy)
        .init(ScenarioValueRaw::str("5"))
        .code(CODE_PATH)
        .new_address(TestSCAddress::new("adder"))
        .run();

    world
        .query()
        .id("2")
        .to(TestSCAddress::new("adder"))
        .typed(adder_proxy::AdderProxy)
        .sum()
        .returns(ExpectValue(ScenarioValueRaw::str("5")))
        .run();

    world
        .tx()
        .id("3")
        .from(TestAddress::new("owner"))
        .to(TestSCAddress::new("adder"))
        .typed(adder_proxy::AdderProxy)
        .add(ScenarioValueRaw::str("3"))
        .run();

    world
        .check_account(TestSCAddress::new("adder"))
        .check_storage("str:sum", "8");
}
