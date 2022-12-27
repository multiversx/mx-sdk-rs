#[test]
fn deploy_rs() {
    mx_sc_debug::mandos_go("scenarios/deploy.scen.json");
}

#[test]
fn deposit_rs() {
    mx_sc_debug::mandos_go("scenarios/deposit.scen.json");
}

#[test]
fn set_bonding_curve_rs() {
    mx_sc_debug::mandos_go("scenarios/set_bonding_curve.scen.json");
}

#[test]
fn buy_rs() {
    mx_sc_debug::mandos_go("scenarios/buy.scen.json");
}

#[test]
fn sell_rs() {
    mx_sc_debug::mandos_go("scenarios/sell.scen.json");
}

#[test]
fn deposit_more_view_rs() {
    mx_sc_debug::mandos_go("scenarios/deposit_more_view.scen.json");
}

#[test]
fn claim_rs() {
    mx_sc_debug::mandos_go("scenarios/claim.scen.json");
}
