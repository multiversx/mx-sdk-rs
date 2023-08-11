use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn queue_repeat_go() {
    world().run("scenarios/queue_repeat.scen.json");
}

#[test]
fn queue_repeat_struct_go() {
    world().run("scenarios/queue_repeat_struct.scen.json");
}
