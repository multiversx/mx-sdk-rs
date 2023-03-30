#[test]
fn test_add_group_go() {
    multiversx_sc_scenario::run_go("scenarios/test-add-group.scen.json");
}

#[test]
fn test_add_user_go() {
    multiversx_sc_scenario::run_go("scenarios/test-add-user.scen.json");
}

#[test]
fn test_change_user_go() {
    multiversx_sc_scenario::run_go("scenarios/test-change-user.scen.json");
}

#[test]
fn test_claim_go() {
    multiversx_sc_scenario::run_go("scenarios/test-claim.scen.json");
}

#[test]
fn test_end_setup_go() {
    multiversx_sc_scenario::run_go("scenarios/test-end-setup.scen.json");
}

#[test]
fn test_init_go() {
    multiversx_sc_scenario::run_go("scenarios/test-init.scen.json");
}
