use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn large_vec_storage_go() {
    world().run("scenarios/large_vec_storage.scen.json");
}

#[test]
fn large_vec_storage_preload_go() {
    world().run("scenarios/large_vec_storage_preload.scen.json");
}
