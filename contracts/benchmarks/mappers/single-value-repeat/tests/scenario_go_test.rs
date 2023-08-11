use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn single_value_repeat_go() {
    world().run("scenarios/single_value_repeat.scen.json");
}

#[test]
fn single_value_repeat_struct_go() {
    world().run("scenarios/single_value_repeat_struct.scen.json");
}

#[test]
fn single_value_repeat_struct_go() {
    multiversx_sc_scenario::run_go("scenarios/single_value_repeat_struct.scen.json");
}
