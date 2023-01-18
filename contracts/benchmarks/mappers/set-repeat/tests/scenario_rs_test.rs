use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/benchmarks/mappers/set-repeat");

    blockchain.register_contract("file:output/set-repeat.wasm", set_repeat::ContractBuilder);
    blockchain
}

#[test]
fn set_repeat_struct_rs() {
    multiversx_sc_scenario::run_rs("scenarios/set_repeat_struct.scen.json", world());
}

#[test]
fn set_repeat_rs() {
    multiversx_sc_scenario::run_rs("scenarios/set_repeat.scen.json", world());
}
