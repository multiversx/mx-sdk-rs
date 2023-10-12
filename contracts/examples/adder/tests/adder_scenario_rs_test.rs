use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    todo!()
}

#[test]
fn adder_rs() {
    world().run("scenarios/adder.scen.json");
}

#[test]
fn interactor_trace_rs() {
    world().run("scenarios/interactor_trace.scen.json");
}
