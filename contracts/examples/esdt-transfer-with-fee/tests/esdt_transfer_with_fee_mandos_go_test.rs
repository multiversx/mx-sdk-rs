#[test]
fn deploy_go() {
    mx_sc_debug::mandos_go("scenarios/deploy.scen.json");
}

#[test]
fn setup_fees_go() {
    mx_sc_debug::mandos_go("scenarios/setup_fees_and_transfer.scen.json");
}

#[test]
fn claim_go() {
    mx_sc_debug::mandos_go("scenarios/claim.scen.json");
}
