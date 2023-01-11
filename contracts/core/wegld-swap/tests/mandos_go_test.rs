#[test]
fn unwrap_egld_go() {
    multiversx_sc_scenario::run_go("scenarios/unwrap_egld.scen.json");
}

#[test]
fn wrap_egld_go() {
    multiversx_sc_scenario::run_go("scenarios/wrap_egld.scen.json");
}
