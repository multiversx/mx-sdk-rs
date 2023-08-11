use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn map_repeat_go() {
    world().run("scenarios/map_repeat.scen.json");
}

#[test]
fn map_repeat_struct_go() {
    world().run("scenarios/map_repeat_struct.scen.json");
}
