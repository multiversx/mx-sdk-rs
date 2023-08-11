use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn vec_repeat_go() {
    world().run("scenarios/vec_repeat.scen.json");
}

#[test]
fn vec_repeat_struct_go() {
    world().run("scenarios/vec_repeat_struct.scen.json");
}

#[test]
fn vec_repeat_struct_go() {
    multiversx_sc_scenario::run_go("scenarios/vec_repeat_struct.scen.json");
}
