////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    set_repeat::endpoints::init(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn add() {
    set_repeat::endpoints::add(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn count() {
    set_repeat::endpoints::count(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn remove() {
    set_repeat::endpoints::remove(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getBenchmark() {
    set_repeat::endpoints::getBenchmark(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn callBack() {
    set_repeat::endpoints::callBack(elrond_wasm_node::arwen_api());
}
