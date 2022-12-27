#[test]
fn external_pure_go() {
    mx_sc_debug::scenario_go("scenarios/external-pure.scen.json");
}

#[test]
fn external_get_go() {
    mx_sc_debug::scenario_go("scenarios/external-get.scen.json");
}
