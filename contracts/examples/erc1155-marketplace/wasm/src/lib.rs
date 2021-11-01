////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    erc1155_marketplace::endpoints::init(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    erc1155_marketplace::endpoints::callBack(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn bid() {
    erc1155_marketplace::endpoints::bid(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn claim() {
    erc1155_marketplace::endpoints::claim(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn endAuction() {
    erc1155_marketplace::endpoints::endAuction(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn getAuctionStatus() {
    erc1155_marketplace::endpoints::getAuctionStatus(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn getCurrentWinner() {
    erc1155_marketplace::endpoints::getCurrentWinner(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn getCurrentWinningBid() {
    erc1155_marketplace::endpoints::getCurrentWinningBid(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn getPercentageCut() {
    erc1155_marketplace::endpoints::getPercentageCut(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn isUpForAuction() {
    erc1155_marketplace::endpoints::isUpForAuction(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn onERC1155BatchReceived() {
    erc1155_marketplace::endpoints::onERC1155BatchReceived(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn onERC1155Received() {
    erc1155_marketplace::endpoints::onERC1155Received(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn setCutPercentage() {
    erc1155_marketplace::endpoints::setCutPercentage(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn setTokenOwnershipContractAddress() {
    erc1155_marketplace::endpoints::setTokenOwnershipContractAddress(elrond_wasm_node::vm_api());
}
