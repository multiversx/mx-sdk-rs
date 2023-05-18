use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn adder_go() {
    world().run("scenarios/adder.scen.json");
}

#[test]
fn interactor_trace_go() {
    multiversx_sc_scenario::run_go("scenarios/interactor_trace.scen.json");
}
