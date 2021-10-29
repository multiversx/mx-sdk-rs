////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    send_tx_repeat::endpoints::init(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn repeat() {
    send_tx_repeat::endpoints::repeat(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn callBack() {
    send_tx_repeat::endpoints::callBack(elrond_wasm_node::arwen_api());
}
