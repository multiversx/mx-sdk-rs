#[test]
fn buy_go() {
    multiversx_sc_scenario::run_go("scenarios/buy.scen.json");
}

#[test]
fn claim_go() {
    multiversx_sc_scenario::run_go("scenarios/claim.scen.json");
}

#[test]
fn deploy_go() {
    multiversx_sc_scenario::run_go("scenarios/deploy.scen.json");
}

#[test]
fn deposit_go() {
    multiversx_sc_scenario::run_go("scenarios/deposit.scen.json");
}

#[test]
fn deposit_more_view_go() {
    multiversx_sc_scenario::run_go("scenarios/deposit_more_view.scen.json");
}

#[test]
fn sell_go() {
    multiversx_sc_scenario::run_go("scenarios/sell.scen.json");
}

#[test]
fn set_bonding_curve_go() {
    multiversx_sc_scenario::run_go("scenarios/set_bonding_curve.scen.json");
}
