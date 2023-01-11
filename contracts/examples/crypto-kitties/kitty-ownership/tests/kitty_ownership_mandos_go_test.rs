#[test]
fn approve_siring_go() {
    mx_sc_scenario::scenario_go("scenarios/approve_siring.scen.json");
}

#[test]
fn breed_ok_go() {
    mx_sc_scenario::scenario_go("scenarios/breed_ok.scen.json");
}

#[test]
fn give_birth_go() {
    mx_sc_scenario::scenario_go("scenarios/give_birth.scen.json");
}

#[test]
fn init_go() {
    mx_sc_scenario::scenario_go("scenarios/init.scen.json");
}

#[test]
fn query_go() {
    mx_sc_scenario::scenario_go("scenarios/query.scen.json");
}

#[test]
fn setup_accounts_go() {
    mx_sc_scenario::scenario_go("scenarios/setup_accounts.scen.json");
}
