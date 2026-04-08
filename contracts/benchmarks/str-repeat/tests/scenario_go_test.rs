use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
#[ignore = "gas benchmark, too brittle to include permanently"]
fn mb_builder_basic_go() {
    world().run("scenarios/mb_builder_basic.scen.json");
}

#[test]
#[ignore = "gas benchmark, too brittle to include permanently"]
fn mb_builder_cached_go() {
    world().run("scenarios/mb_builder_cached.scen.json");
}

#[test]
fn str_repeat_go() {
    world().run("scenarios/str_repeat.scen.json");
}
