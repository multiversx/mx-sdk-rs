use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/adder");

    blockchain.register_contract("file:output/adder.wasm", adder::ContractBuilder);
    blockchain
}

#[test]
fn adder_rs() {
    multiversx_sc_scenario::run_rs("scenarios/adder.scen.json", world());
}

#[test]
fn interactor_trace_rs() {
    multiversx_sc_scenario::run_rs("scenarios/interactor_trace.scen.json", world());
}
