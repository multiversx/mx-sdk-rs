use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/adder");

    blockchain.register_contract("file:output/adder.wasm", adder::ContractBuilder);
    blockchain
}

/// Test!
#[ignore]
#[test]
fn adder_rs() {
    multiversx_sc_scenario::run_rs("scenarios/adder.scen.json", world());
}

#[test]
fn adder2_rs() {
    multiversx_sc_scenario::run_rs("scenarios/adder_____________________________________________________.scen.json", world());
}
