use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    todo!()
}

#[test]
#[ignore = "not supported"]
fn test_subscription_rs() {
    world().run("scenarios/test_subscription.scen.json");
}

#[test]
#[ignore = "not supported"]
fn mint_nft_rs() {
    world().run("scenarios/mint_nft.scen.json");
}

#[test]
#[ignore = "not supported"]
fn init_rs() {
    world().run("scenarios/init.scen.json");
}
