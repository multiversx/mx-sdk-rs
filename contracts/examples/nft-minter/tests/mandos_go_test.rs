#[test]
fn init_go() {
    multiversx_sc_scenario::run_go("scenarios/init.scen.json");
}

#[test]
fn create_nft_go() {
    multiversx_sc_scenario::run_go("scenarios/create_nft.scen.json");
}

#[test]
fn buy_nft_go() {
    multiversx_sc_scenario::run_go("scenarios/buy_nft.scen.json");
}
