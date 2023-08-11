use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn set_repeat_go() {
    world().run("scenarios/set_repeat.scen.json");
}

#[test]
fn set_repeat_struct_go() {
    world().run("scenarios/set_repeat_struct.scen.json");
}
