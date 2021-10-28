////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    kitty_auction::endpoints::init(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn setKittyOwnershipContractAddress() {
    kitty_auction::endpoints::setKittyOwnershipContractAddress(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn createAndAuctionGenZeroKitty() {
    kitty_auction::endpoints::createAndAuctionGenZeroKitty(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn isUpForAuction() {
    kitty_auction::endpoints::isUpForAuction(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getAuctionStatus() {
    kitty_auction::endpoints::getAuctionStatus(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getCurrentWinningBid() {
    kitty_auction::endpoints::getCurrentWinningBid(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn createSaleAuction() {
    kitty_auction::endpoints::createSaleAuction(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn createSiringAuction() {
    kitty_auction::endpoints::createSiringAuction(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn bid() {
    kitty_auction::endpoints::bid(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn endAuction() {
    kitty_auction::endpoints::endAuction(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn callBack() {
    kitty_auction::endpoints::callBack(elrond_wasm_node::arwen_api());
}
