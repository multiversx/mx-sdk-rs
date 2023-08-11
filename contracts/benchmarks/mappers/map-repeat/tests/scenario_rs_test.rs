use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/benchmarks/mappers/map-repeat");

    blockchain.register_contract("file:output/map-repeat.wasm", map_repeat::ContractBuilder);
    blockchain
}

#[test]
fn map_repeat_rs() {
    world().run("scenarios/map_repeat.scen.json");
}

#[test]
fn map_repeat_struct_rs() {
    world().run("scenarios/map_repeat_struct.scen.json");
}
