#[test]
#[ignore = "builtin SC not implemented"]
fn esdt_system_sc_go() {
    multiversx_sc_scenario::run_go("scenarios/esdt_system_sc.scen.json");
}
