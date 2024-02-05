use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn buy_nft_go() {
    world().run("scenarios/buy_nft.scen.json");
}

#[test]
fn create_nft_go() {
    world().run("scenarios/create_nft.scen.json");
}

#[test]
fn init_go() {
    world().run("scenarios/init.scen.json");
}
