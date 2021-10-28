////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    queue_repeat::endpoints::init(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn add() {
    queue_repeat::endpoints::add(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn count() {
    queue_repeat::endpoints::count(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn remove() {
    queue_repeat::endpoints::remove(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getBenchmark() {
    queue_repeat::endpoints::getBenchmark(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn callBack() {
    queue_repeat::endpoints::callBack(elrond_wasm_node::arwen_api());
}
