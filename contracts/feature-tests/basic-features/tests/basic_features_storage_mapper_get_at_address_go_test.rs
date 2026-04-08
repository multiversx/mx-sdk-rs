use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn storage_mapper_get_at_address_go() {
    world().run("scenarios/storage_mapper_get_at_address.scen.json");
}
