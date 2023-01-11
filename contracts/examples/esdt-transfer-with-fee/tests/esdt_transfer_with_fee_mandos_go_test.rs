#[test]
fn deploy_go() {
    mx_sc_scenario::scenario_go("scenarios/deploy.scen.json");
}

#[test]
fn setup_fees_go() {
    mx_sc_scenario::scenario_go("scenarios/setup_fees_and_transfer.scen.json");
}

#[test]
fn claim_go() {
    mx_sc_scenario::scenario_go("scenarios/claim.scen.json");
}
