use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/benchmarks/large-storage");
    blockchain.register_contract(
        "mxsc:output/large-storage.mxsc.json",
        large_storage::ContractBuilder,
    );
    blockchain
}

#[test]
fn large_storage_rs() {
    world().run("scenarios/large_storage.scen.json");
}
