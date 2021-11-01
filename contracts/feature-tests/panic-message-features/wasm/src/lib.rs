////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    panic_message_features::endpoints::init(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn callBack() {
    panic_message_features::endpoints::callBack(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn panicWithMessage() {
    panic_message_features::endpoints::panicWithMessage(elrond_wasm_node::arwen_api());
}
