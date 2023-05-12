use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
#[ignore = "builtin SC not implemented"]
fn esdt_system_sc_go() {
    world().run("scenarios/esdt_system_sc.scen.json");
}
