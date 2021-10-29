////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    erc20::endpoints::init(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn totalSupply() {
    erc20::endpoints::totalSupply(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn balanceOf() {
    erc20::endpoints::balanceOf(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn allowance() {
    erc20::endpoints::allowance(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn transfer() {
    erc20::endpoints::transfer(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn transferFrom() {
    erc20::endpoints::transferFrom(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn approve() {
    erc20::endpoints::approve(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn callBack() {
    erc20::endpoints::callBack(elrond_wasm_node::arwen_api());
}
