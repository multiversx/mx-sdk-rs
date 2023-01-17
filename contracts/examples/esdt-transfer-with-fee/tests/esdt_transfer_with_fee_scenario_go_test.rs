#[test]
fn deploy_go() {
    multiversx_sc_scenario::run_go("scenarios/deploy.scen.json");
}

#[test]
fn setup_fees_go() {
    multiversx_sc_scenario::run_go("scenarios/setup_fees_and_transfer.scen.json");
}

#[test]
fn claim_go() {
    multiversx_sc_scenario::run_go("scenarios/claim.scen.json");
}
