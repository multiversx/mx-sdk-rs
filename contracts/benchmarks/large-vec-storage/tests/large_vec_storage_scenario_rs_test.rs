use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/benchmarks/large-vec-storage");
    blockchain.register_contract(
        "mxsc:output/large-vec-storage.mxsc.json",
        large_vec_storage::ContractBuilder,
    );
    blockchain
}

#[test]
fn large_vec_storage_rs() {
    world().run("scenarios/large_vec_storage.scen.json");
}
