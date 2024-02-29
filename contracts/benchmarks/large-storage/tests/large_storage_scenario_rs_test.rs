use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/benchmarks/large-storage");
    blockchain.register_contract(
        "file:output/large-storage.wasm",
        large_storage::ContractBuilder,
    );
    blockchain
}

#[test]
fn large_storage_rs() {
    world().run("scenarios/large_storage.scen.json");
}
