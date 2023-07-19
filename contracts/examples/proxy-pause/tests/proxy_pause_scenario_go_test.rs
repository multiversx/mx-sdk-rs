#[test]
fn init_go() {
    multiversx_sc_scenario::run_go("scenarios/init.scen.json");
}

#[test]
fn pause_and_unpause_go() {
    multiversx_sc_scenario::run_go("scenarios/pause-and-unpause.scen.json");
}
