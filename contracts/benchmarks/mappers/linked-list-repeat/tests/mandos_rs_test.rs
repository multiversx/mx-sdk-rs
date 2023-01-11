use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/benchmarks/mappers/linked-list-repeat");

    blockchain.register_contract(
        "file:output/linked-list-repeat.wasm",
        linked_list_repeat::ContractBuilder,
    );
    blockchain
}

#[test]
fn linked_list_repeat_struct_rs() {
    multiversx_sc_scenario::run_rs("scenarios/linked_list_repeat_struct.scen.json", world());
}

#[test]
fn linked_list_repeat_rs() {
    multiversx_sc_scenario::run_rs("scenarios/linked_list_repeat.scen.json", world());
}
