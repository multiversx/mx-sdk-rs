use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn str_repeat_go() {
    world().run("scenarios/str_repeat.scen.json");
}
