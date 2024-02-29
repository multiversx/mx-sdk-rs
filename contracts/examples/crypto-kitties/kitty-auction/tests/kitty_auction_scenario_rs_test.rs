use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        "file:../kitty-ownership/output/kitty-ownership.wasm",
        kitty_ownership::ContractBuilder,
    );
    blockchain.register_contract(
        "file:output/kitty-auction.wasm",
        kitty_auction::ContractBuilder,
    );

    blockchain
}
#[test]
fn bid_first_rs() {
    world().run("scenarios/bid_first.scen.json");
}

#[test]
fn bid_second_max_rs() {
    world().run("scenarios/bid_second_max.scen.json");
}

#[test]
fn bid_second_ok_rs() {
    world().run("scenarios/bid_second_ok.scen.json");
}

#[test]
fn bid_second_too_low_rs() {
    world().run("scenarios/bid_second_too_low.scen.json");
}

#[test]
fn bid_siring_auction_rs() {
    world().run("scenarios/bid_siring_auction.scen.json");
}

#[test]
fn create_and_auction_gen_zero_kitty_rs() {
    world().run("scenarios/create_and_auction_gen_zero_kitty.scen.json");
}

#[test]
fn create_sale_auction_not_owner_rs() {
    world().run("scenarios/create_sale_auction_not_owner.scen.json");
}

#[test]
fn create_sale_auction_ok_rs() {
    world().run("scenarios/create_sale_auction_ok.scen.json");
}

#[test]
fn create_siring_auction_not_owner_rs() {
    world().run("scenarios/create_siring_auction_not_owner.scen.json");
}

#[test]
fn create_siring_auction_ok_rs() {
    world().run("scenarios/create_siring_auction_ok.scen.json");
}

#[test]
fn end_auction_no_bids_rs() {
    world().run("scenarios/end_auction_no_bids.scen.json");
}

#[test]
fn end_auction_second_bid_max_early_rs() {
    world().run("scenarios/end_auction_second_bid_max_early.scen.json");
}

#[test]
fn end_auction_second_bid_ok_early_rs() {
    world().run("scenarios/end_auction_second_bid_ok_early.scen.json");
}

#[test]
fn end_auction_second_bid_ok_late_rs() {
    world().run("scenarios/end_auction_second_bid_ok_late.scen.json");
}

#[test]
fn end_siring_auction_rs() {
    world().run("scenarios/end_siring_auction.scen.json");
}

#[test]
fn init_rs() {
    world().run("scenarios/init.scen.json");
}
