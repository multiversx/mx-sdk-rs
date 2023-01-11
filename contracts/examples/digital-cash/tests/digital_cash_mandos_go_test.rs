#[test]
fn claim_egld_go() {
    mx_sc_scenario::scenario_go("scenarios/claim-egld.scen.json");
}

#[test]
fn claim_esdt_go() {
    mx_sc_scenario::scenario_go("scenarios/claim-esdt.scen.json");
}

#[test]
fn fund_egld_and_esdt_go() {
    mx_sc_scenario::scenario_go("scenarios/fund-egld-and-esdt.scen.json");
}

#[test]
fn set_accounts_go() {
    mx_sc_scenario::scenario_go("scenarios/set-accounts.scen.json");
}

#[test]
fn withdraw_egld_go() {
    mx_sc_scenario::scenario_go("scenarios/withdraw-egld.scen.json");
}

#[test]
fn withdraw_esdt_go() {
    mx_sc_scenario::scenario_go("scenarios/withdraw-esdt.scen.json");
}
