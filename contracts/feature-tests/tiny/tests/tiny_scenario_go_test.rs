use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn tiny_go() {
    world().run("scenarios/tiny.scen.json");
}
