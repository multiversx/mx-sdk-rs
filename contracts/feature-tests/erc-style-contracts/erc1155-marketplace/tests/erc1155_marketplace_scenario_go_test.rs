use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn auction_batch_go() {
    world().run("scenarios/auction_batch.scen.json");
}

#[test]
fn auction_single_token_egld_go() {
    world().run("scenarios/auction_single_token_egld.scen.json");
}

#[test]
fn bid_first_egld_go() {
    world().run("scenarios/bid_first_egld.scen.json");
}

#[test]
fn bid_second_egld_go() {
    world().run("scenarios/bid_second_egld.scen.json");
}

#[test]
fn bid_third_egld_go() {
    world().run("scenarios/bid_third_egld.scen.json");
}

#[test]
fn end_auction_go() {
    world().run("scenarios/end_auction.scen.json");
}
