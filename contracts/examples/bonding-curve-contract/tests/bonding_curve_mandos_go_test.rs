#[test]
fn deploy_rs() {
    mx_sc_debug::mandos_go("mandos/deploy.scen.json");
}

#[test]
fn deposit_rs() {
    mx_sc_debug::mandos_go("mandos/deposit.scen.json");
}

#[test]
fn set_bonding_curve_rs() {
    mx_sc_debug::mandos_go("mandos/set_bonding_curve.scen.json");
}

#[test]
fn buy_rs() {
    mx_sc_debug::mandos_go("mandos/buy.scen.json");
}

#[test]
fn sell_rs() {
    mx_sc_debug::mandos_go("mandos/sell.scen.json");
}

#[test]
fn deposit_more_view_rs() {
    mx_sc_debug::mandos_go("mandos/deposit_more_view.scen.json");
}

#[test]
fn claim_rs() {
    mx_sc_debug::mandos_go("mandos/claim.scen.json");
}
