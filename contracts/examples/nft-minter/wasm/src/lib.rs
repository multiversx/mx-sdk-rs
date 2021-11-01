////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    nft_minter::endpoints::init(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn callBack() {
    nft_minter::endpoints::callBack(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn buyNft() {
    nft_minter::endpoints::buyNft(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn createNft() {
    nft_minter::endpoints::createNft(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getNftPrice() {
    nft_minter::endpoints::getNftPrice(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn issueToken() {
    nft_minter::endpoints::issueToken(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn setLocalRoles() {
    nft_minter::endpoints::setLocalRoles(elrond_wasm_node::arwen_api());
}
