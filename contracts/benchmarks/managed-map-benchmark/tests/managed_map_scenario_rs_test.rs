use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/benchmarks/managed-map-benchmark");
    blockchain.register_contract(
        "mxsc:output/managed-map-benchmark.mxsc.json",
        managed_map_benchmark::ContractBuilder,
    );
    blockchain
}

#[test]
fn mmap_contains_rs() {
    world().run("scenarios/mmap_contains.scen.json");
}

#[test]
fn mmap_get_rs() {
    world().run("scenarios/mmap_get.scen.json");
}

#[test]
fn mmap_remove_rs() {
    world().run("scenarios/mmap_remove.scen.json");
}
