use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn interactor_trace_go() {
    world().run("scenarios/interactor_trace.scen.json");
}

#[test]
fn st_adder_go() {
    world().run("scenarios/st-adder.scen.json");
}
