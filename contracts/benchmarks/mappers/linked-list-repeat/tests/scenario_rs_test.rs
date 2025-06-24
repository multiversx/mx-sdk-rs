use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/benchmarks/mappers/linked-list-repeat");

    blockchain.register_contract(
        "mxsc:output/linked-list-repeat.mxsc.json",
        linked_list_repeat::ContractBuilder,
    );
    blockchain
}

#[test]
fn linked_list_repeat_rs() {
    world().run("scenarios/linked_list_repeat.scen.json");
}

#[test]
fn linked_list_repeat_struct_rs() {
    world().run("scenarios/linked_list_repeat_struct.scen.json");
}
