use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn init_go() {
    world().run("scenarios/init.scen.json");
}

#[test]
fn pause_and_unpause_go() {
    world().run("scenarios/pause-and-unpause.scen.json");
}
