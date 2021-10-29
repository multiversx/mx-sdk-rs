////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    second_contract::endpoints::init(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn acceptEsdtPayment() {
    second_contract::endpoints::acceptEsdtPayment(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn rejectEsdtPayment() {
    second_contract::endpoints::rejectEsdtPayment(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getesdtTokenName() {
    second_contract::endpoints::getesdtTokenName(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn callBack() {
    second_contract::endpoints::callBack(elrond_wasm_node::arwen_api());
}
