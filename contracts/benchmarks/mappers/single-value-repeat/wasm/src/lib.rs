////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    single_value_repeat::endpoints::init(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn add() {
    single_value_repeat::endpoints::add(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn count() {
    single_value_repeat::endpoints::count(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn remove() {
    single_value_repeat::endpoints::remove(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn callBack() {
    single_value_repeat::endpoints::callBack(elrond_wasm_node::arwen_api());
}
