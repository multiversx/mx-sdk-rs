////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    proxy_test_second::endpoints::init(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn payMe() {
    proxy_test_second::endpoints::payMe(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn payMeWithResult() {
    proxy_test_second::endpoints::payMeWithResult(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn messageMe() {
    proxy_test_second::endpoints::messageMe(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn callBack() {
    proxy_test_second::endpoints::callBack(elrond_wasm_node::arwen_api());
}
