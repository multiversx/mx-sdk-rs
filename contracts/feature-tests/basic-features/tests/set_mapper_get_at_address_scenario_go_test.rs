use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn set_mapper_get_at_address_go() {
    world().run("scenarios/set_mapper_get_at_address.scen.json");
}