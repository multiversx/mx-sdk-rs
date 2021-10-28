////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    crowdfunding_erc20::endpoints::init(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn fund() {
    crowdfunding_erc20::endpoints::fund(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn status() {
    crowdfunding_erc20::endpoints::status(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn claim() {
    crowdfunding_erc20::endpoints::claim(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn get_target() {
    crowdfunding_erc20::endpoints::get_target(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn get_deadline() {
    crowdfunding_erc20::endpoints::get_deadline(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn get_deposit() {
    crowdfunding_erc20::endpoints::get_deposit(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn get_erc20_contract_address() {
    crowdfunding_erc20::endpoints::get_erc20_contract_address(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn get_total_balance() {
    crowdfunding_erc20::endpoints::get_total_balance(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn callBack() {
    crowdfunding_erc20::endpoints::callBack(elrond_wasm_node::arwen_api());
}
