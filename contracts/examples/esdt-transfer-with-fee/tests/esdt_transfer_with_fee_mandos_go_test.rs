#[test]
fn deploy_go() {
    mx_sc_debug::mandos_go("mandos/deploy.scen.json");
}

#[test]
fn setup_fees_go() {
    mx_sc_debug::mandos_go("mandos/setup_fees_and_transfer.scen.json");
}

#[test]
fn claim_go() {
    mx_sc_debug::mandos_go("mandos/claim.scen.json");
}
