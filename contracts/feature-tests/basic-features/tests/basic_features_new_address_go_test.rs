use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn new_address_go_test() {
    world().run("scenarios/new_address.scen.json");
}
