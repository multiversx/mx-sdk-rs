#[test]
fn single_value_repeat_struct_go() {
    multiversx_sc_scenario::run_go("scenarios/single_value_repeat_struct.scen.json");
}

#[test]
fn single_value_repeat_go() {
    multiversx_sc_scenario::run_go("scenarios/single_value_repeat.scen.json");
}
