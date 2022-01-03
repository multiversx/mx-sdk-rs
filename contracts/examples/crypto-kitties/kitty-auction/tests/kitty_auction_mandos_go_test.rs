#[test]
fn bid_first_go() {
    elrond_wasm_debug::mandos_go("mandos/bid_first.scen.json");
}

#[test]
fn bid_second_max_go() {
    elrond_wasm_debug::mandos_go("mandos/bid_second_max.scen.json");
}

#[test]
fn bid_second_ok_go() {
    elrond_wasm_debug::mandos_go("mandos/bid_second_ok.scen.json");
}

#[test]
fn bid_second_too_low_go() {
    elrond_wasm_debug::mandos_go("mandos/bid_second_too_low.scen.json");
}

#[test]
fn bid_siring_auction_go() {
    elrond_wasm_debug::mandos_go("mandos/bid_siring_auction.scen.json");
}

#[test]
fn create_and_auction_gen_zero_kitty_go() {
    elrond_wasm_debug::mandos_go("mandos/create_and_auction_gen_zero_kitty.scen.json");
}

#[test]
fn create_sale_auction_not_owner_go() {
    elrond_wasm_debug::mandos_go("mandos/create_sale_auction_not_owner.scen.json");
}

#[test]
fn create_sale_auction_ok_go() {
    elrond_wasm_debug::mandos_go("mandos/create_sale_auction_ok.scen.json");
}

#[test]
fn create_siring_auction_not_owner_go() {
    elrond_wasm_debug::mandos_go("mandos/create_siring_auction_not_owner.scen.json");
}

#[test]
fn create_siring_auction_ok_go() {
    elrond_wasm_debug::mandos_go("mandos/create_siring_auction_ok.scen.json");
}

#[test]
fn end_auction_no_bids_go() {
    elrond_wasm_debug::mandos_go("mandos/end_auction_no_bids.scen.json");
}

#[test]
fn end_auction_second_bid_max_early_go() {
    elrond_wasm_debug::mandos_go("mandos/end_auction_second_bid_max_early.scen.json");
}

#[test]
fn end_auction_second_bid_ok_early_go() {
    elrond_wasm_debug::mandos_go("mandos/end_auction_second_bid_ok_early.scen.json");
}

#[test]
fn end_auction_second_bid_ok_late_go() {
    elrond_wasm_debug::mandos_go("mandos/end_auction_second_bid_ok_late.scen.json");
}

#[test]
fn end_siring_auction_go() {
    elrond_wasm_debug::mandos_go("mandos/end_siring_auction.scen.json");
}

#[test]
fn init_go() {
    elrond_wasm_debug::mandos_go("mandos/init.scen.json");
}
