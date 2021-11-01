////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    order_book_factory::endpoints::init(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn callBack() {
    order_book_factory::endpoints::callBack(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn createPair() {
    order_book_factory::endpoints::createPair(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getPair() {
    order_book_factory::endpoints::getPair(elrond_wasm_node::arwen_api());
}
