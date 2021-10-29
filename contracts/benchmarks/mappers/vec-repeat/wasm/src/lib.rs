////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    vec_repeat::endpoints::init(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn add() {
    vec_repeat::endpoints::add(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn count() {
    vec_repeat::endpoints::count(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn remove() {
    vec_repeat::endpoints::remove(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getBenchmark() {
    vec_repeat::endpoints::getBenchmark(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn callBack() {
    vec_repeat::endpoints::callBack(elrond_wasm_node::arwen_api());
}
