use elrond_wasm::*;
use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();

    blockchain.register_contract(
        "file:../kitty-ownership/output/kitty-ownership.wasm",
        Box::new(|context| Box::new(kitty_ownership::contract_obj(context))),
    );
    blockchain.register_contract(
        "file:output/kitty-auction.wasm",
        Box::new(|context| Box::new(kitty_auction::contract_obj(context))),
    );

    blockchain
}
#[test]
fn bid_first_rs() {
    elrond_wasm_debug::mandos_rs("mandos/bid_first.scen.json", world());
}

#[test]
fn bid_second_max_rs() {
    elrond_wasm_debug::mandos_rs("mandos/bid_second_max.scen.json", world());
}

#[test]
fn bid_second_ok_rs() {
    elrond_wasm_debug::mandos_rs("mandos/bid_second_ok.scen.json", world());
}

#[test]
fn bid_second_too_low_rs() {
    elrond_wasm_debug::mandos_rs("mandos/bid_second_too_low.scen.json", world());
}

#[test]
fn bid_siring_auction_rs() {
    elrond_wasm_debug::mandos_rs("mandos/bid_siring_auction.scen.json", world());
}

#[test]
fn create_and_auction_gen_zero_kitty_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/create_and_auction_gen_zero_kitty.scen.json",
        world(),
    );
}

#[test]
fn create_sale_auction_not_owner_rs() {
    elrond_wasm_debug::mandos_rs("mandos/create_sale_auction_not_owner.scen.json", world());
}

#[test]
fn create_sale_auction_ok_rs() {
    elrond_wasm_debug::mandos_rs("mandos/create_sale_auction_ok.scen.json", world());
}

#[test]
fn create_siring_auction_not_owner_rs() {
    elrond_wasm_debug::mandos_rs("mandos/create_siring_auction_not_owner.scen.json", world());
}

#[test]
fn create_siring_auction_ok_rs() {
    elrond_wasm_debug::mandos_rs("mandos/create_siring_auction_ok.scen.json", world());
}

#[test]
fn end_auction_no_bids_rs() {
    elrond_wasm_debug::mandos_rs("mandos/end_auction_no_bids.scen.json", world());
}

#[test]
fn end_auction_second_bid_max_early_rs() {
    elrond_wasm_debug::mandos_rs("mandos/end_auction_second_bid_max_early.scen.json", world());
}

#[test]
fn end_auction_second_bid_ok_early_rs() {
    elrond_wasm_debug::mandos_rs("mandos/end_auction_second_bid_ok_early.scen.json", world());
}

#[test]
fn end_auction_second_bid_ok_late_rs() {
    elrond_wasm_debug::mandos_rs("mandos/end_auction_second_bid_ok_late.scen.json", world());
}

#[test]
fn end_siring_auction_rs() {
    elrond_wasm_debug::mandos_rs("mandos/end_siring_auction.scen.json", world());
}

#[test]
fn init_rs() {
    elrond_wasm_debug::mandos_rs("mandos/init.scen.json", world());
}
