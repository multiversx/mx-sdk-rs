#[test]
fn adder_go() {
    multiversx_sc_scenario::run_go("scenarios/adder.scen.json");
}

#[test]
fn interactor_trace_go() {
    multiversx_sc_scenario::run_go("scenarios/interactor_trace.scen.json");
}
