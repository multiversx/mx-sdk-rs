use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/managed-map-features");
    blockchain.register_contract(
        "file:output/managed-map-features.wasm",
        managed_map_features::ContractBuilder,
    );
    blockchain
}

#[test]
fn mmap_get_rs() {
    multiversx_sc_scenario::run_rs("scenarios/mmap_get.scen.json", world());
}

#[test]
fn mmap_remove_rs() {
    multiversx_sc_scenario::run_rs("scenarios/mmap_remove.scen.json", world());
}
