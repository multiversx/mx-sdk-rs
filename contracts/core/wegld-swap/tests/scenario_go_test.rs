use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn unwrap_egld_go() {
    world().run("scenarios/unwrap_egld.scen.json");
}

#[test]
fn wrap_egld_go() {
    world().run("scenarios/wrap_egld.scen.json");
}
