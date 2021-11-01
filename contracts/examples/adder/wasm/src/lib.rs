////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    adder::endpoints::init(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    adder::endpoints::callBack(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn add() {
    adder::endpoints::add(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn getSum() {
    adder::endpoints::getSum(elrond_wasm_node::vm_api());
}
