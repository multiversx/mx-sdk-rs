////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    nft_storage_prepay::endpoints::init(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn setCostPerByte() {
    nft_storage_prepay::endpoints::setCostPerByte(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn reserveFunds() {
    nft_storage_prepay::endpoints::reserveFunds(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn claim() {
    nft_storage_prepay::endpoints::claim(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn depositPaymentForStorage() {
    nft_storage_prepay::endpoints::depositPaymentForStorage(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn withdraw() {
    nft_storage_prepay::endpoints::withdraw(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getCostForSize() {
    nft_storage_prepay::endpoints::getCostForSize(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getDepositAmount() {
    nft_storage_prepay::endpoints::getDepositAmount(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getCostPerByte() {
    nft_storage_prepay::endpoints::getCostPerByte(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn callBack() {
    nft_storage_prepay::endpoints::callBack(elrond_wasm_node::arwen_api());
}
