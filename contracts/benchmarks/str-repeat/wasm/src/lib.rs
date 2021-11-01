////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    str_repeat::endpoints::init(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn callBack() {
    str_repeat::endpoints::callBack(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getByteArray() {
    str_repeat::endpoints::getByteArray(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getByteArrayLength() {
    str_repeat::endpoints::getByteArrayLength(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn repeat() {
    str_repeat::endpoints::repeat(elrond_wasm_node::arwen_api());
}
