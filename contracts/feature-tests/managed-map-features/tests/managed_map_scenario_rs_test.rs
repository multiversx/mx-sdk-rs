use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new().executor_config(ExecutorConfig::full_suite());

    blockchain.set_current_dir_from_workspace("contracts/feature-tests/managed-map-features");
    blockchain.register_contract(
        "mxsc:output/managed-map-features.mxsc.json",
        managed_map_features::ContractBuilder,
    );
    blockchain
}

#[test]
fn mmap_get_rs() {
    world().run("scenarios/mmap_get.scen.json");
}

#[test]
fn mmap_key_mutability_rs() {
    world().run("scenarios/mmap_key_mutability.scen.json");
}

#[test]
fn mmap_mutable_input_rs() {
    world().run("scenarios/mmap_mutable_input.scen.json");
}

#[test]
fn mmap_remove_rs() {
    world().run("scenarios/mmap_remove.scen.json");
}
