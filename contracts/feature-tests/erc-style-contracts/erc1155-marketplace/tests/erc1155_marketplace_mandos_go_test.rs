#[test]
fn auction_batch_go() {
    elrond_wasm_debug::mandos_go("mandos/auction_batch.scen.json");
}

#[test]
fn auction_single_token_egld_go() {
    elrond_wasm_debug::mandos_go("mandos/auction_single_token_egld.scen.json");
}

#[test]
fn bid_first_egld_go() {
    elrond_wasm_debug::mandos_go("mandos/bid_first_egld.scen.json");
}

#[test]
fn bid_second_egld_go() {
    elrond_wasm_debug::mandos_go("mandos/bid_second_egld.scen.json");
}

#[test]
fn bid_third_egld_go() {
    elrond_wasm_debug::mandos_go("mandos/bid_third_egld.scen.json");
}

#[test]
fn end_auction_go() {
    elrond_wasm_debug::mandos_go("mandos/end_auction.scen.json");
}
