////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    child::endpoints::init(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn callBack() {
    child::endpoints::callBack(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getWrappedEgldTokenIdentifier() {
    child::endpoints::getWrappedEgldTokenIdentifier(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn issueWrappedEgld() {
    child::endpoints::issueWrappedEgld(elrond_wasm_node::arwen_api());
}
