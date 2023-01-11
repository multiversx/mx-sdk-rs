#[test]
fn external_pure_go() {
    multiversx_sc_scenario::run_go("scenarios/external-pure.scen.json");
}

#[test]
fn external_get_go() {
    multiversx_sc_scenario::run_go("scenarios/external-get.scen.json");
}
