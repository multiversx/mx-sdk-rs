use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    todo!()
}

#[test]
#[ignore = "not supported"]
fn buy_nft_rs() {
    world().run("scenarios/buy_nft.scen.json");
}

#[test]
#[ignore = "not supported"]
fn create_nft_rs() {
    world().run("scenarios/create_nft.scen.json");
}

#[test]
#[ignore = "not supported"]
fn init_rs() {
    world().run("scenarios/init.scen.json");
}
