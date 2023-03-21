use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    todo!()
}

#[test]
#[ignore = "not supported"]
fn buy_nft_rs() {
    multiversx_sc_scenario::run_rs("scenarios/buy_nft.scen.json", world());
}

#[test]
#[ignore = "not supported"]
fn create_nft_rs() {
    multiversx_sc_scenario::run_rs("scenarios/create_nft.scen.json", world());
}

#[test]
#[ignore = "not supported"]
fn init_rs() {
    multiversx_sc_scenario::run_rs("scenarios/init.scen.json", world());
}
