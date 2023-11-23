use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn init_go() {
    world().run("scenarios/init.scen.json");
}

#[test]
fn mint_nft_go() {
    world().run("scenarios/mint_nft.scen.json");
}

#[test]
fn test_subscription_go() {
    world().run("scenarios/test_subscription.scen.json");
}
