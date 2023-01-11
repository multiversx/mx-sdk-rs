#[test]
fn init_go() {
    mx_sc_scenario::scenario_go("scenarios/init.scen.json");
}

#[test]
fn create_nft_go() {
    mx_sc_scenario::scenario_go("scenarios/create_nft.scen.json");
}

#[test]
fn buy_nft_go() {
    mx_sc_scenario::scenario_go("scenarios/buy_nft.scen.json");
}
