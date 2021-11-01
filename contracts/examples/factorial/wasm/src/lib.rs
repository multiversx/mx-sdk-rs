////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    factorial::endpoints::init(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn callBack() {
    factorial::endpoints::callBack(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn factorial() {
    factorial::endpoints::factorial(elrond_wasm_node::arwen_api());
}
