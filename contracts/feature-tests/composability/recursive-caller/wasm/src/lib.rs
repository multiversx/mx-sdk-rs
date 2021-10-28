////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    recursive_caller::endpoints::init(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn recursive_send_funds() {
    recursive_caller::endpoints::recursive_send_funds(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn callBack() {
    recursive_caller::endpoints::callBack(elrond_wasm_node::arwen_api());
}
