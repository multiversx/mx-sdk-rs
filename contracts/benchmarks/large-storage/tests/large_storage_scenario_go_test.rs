use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn large_storage_go() {
    world().run("scenarios/large_storage.scen.json");
}
