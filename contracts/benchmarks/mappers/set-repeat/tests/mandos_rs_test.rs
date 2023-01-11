use mx_sc_scenario::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/benchmarks/mappers/set-repeat");

    blockchain.register_contract("file:output/set-repeat.wasm", set_repeat::ContractBuilder);
    blockchain
}

#[test]
fn set_repeat_struct_rs() {
    mx_sc_scenario::scenario_rs("scenarios/set_repeat_struct.scen.json", world());
}

#[test]
fn set_repeat_rs() {
    mx_sc_scenario::scenario_rs("scenarios/set_repeat.scen.json", world());
}
