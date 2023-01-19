#[test]
fn auction_batch_go() {
    multiversx_sc_scenario::run_go("scenarios/auction_batch.scen.json");
}

#[test]
fn auction_single_token_egld_go() {
    multiversx_sc_scenario::run_go("scenarios/auction_single_token_egld.scen.json");
}

#[test]
fn bid_first_egld_go() {
    multiversx_sc_scenario::run_go("scenarios/bid_first_egld.scen.json");
}

#[test]
fn bid_second_egld_go() {
    multiversx_sc_scenario::run_go("scenarios/bid_second_egld.scen.json");
}

#[test]
fn bid_third_egld_go() {
    multiversx_sc_scenario::run_go("scenarios/bid_third_egld.scen.json");
}

#[test]
fn end_auction_go() {
    multiversx_sc_scenario::run_go("scenarios/end_auction.scen.json");
}
