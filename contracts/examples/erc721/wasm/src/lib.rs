////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    erc721::endpoints::init(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn mint() {
    erc721::endpoints::mint(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn approve() {
    erc721::endpoints::approve(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn revoke() {
    erc721::endpoints::revoke(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn transfer() {
    erc721::endpoints::transfer(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn totalMinted() {
    erc721::endpoints::totalMinted(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn tokenOwner() {
    erc721::endpoints::tokenOwner(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn tokenCount() {
    erc721::endpoints::tokenCount(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn approval() {
    erc721::endpoints::approval(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn callBack() {
    erc721::endpoints::callBack(elrond_wasm_node::arwen_api());
}
