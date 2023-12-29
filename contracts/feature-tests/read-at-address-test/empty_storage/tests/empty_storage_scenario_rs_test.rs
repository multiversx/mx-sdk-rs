use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    // blockchain.set_current_dir_from_workspace("relative path to your workspace, if applicable");

    blockchain.register_contract("file:output/empty_storage.wasm", empty_storage::ContractBuilder);
    blockchain
}

#[test]
fn empty_rs() {
    world().run("scenarios/empty_storage.scen.json");
}
