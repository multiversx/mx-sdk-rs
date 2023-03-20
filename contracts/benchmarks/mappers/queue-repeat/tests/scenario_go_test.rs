#[test]
fn queue_repeat_go() {
    multiversx_sc_scenario::run_go("scenarios/queue_repeat.scen.json");
}

#[test]
fn queue_repeat_struct_go() {
    multiversx_sc_scenario::run_go("scenarios/queue_repeat_struct.scen.json");
}
