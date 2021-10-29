////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    egld_esdt_swap::endpoints::init(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn issueWrappedEgld() {
    egld_esdt_swap::endpoints::issueWrappedEgld(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn mintWrappedEgld() {
    egld_esdt_swap::endpoints::mintWrappedEgld(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn wrapEgld() {
    egld_esdt_swap::endpoints::wrapEgld(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn unwrapEgld() {
    egld_esdt_swap::endpoints::unwrapEgld(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getLockedEgldBalance() {
    egld_esdt_swap::endpoints::getLockedEgldBalance(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getWrappedEgldTokenIdentifier() {
    egld_esdt_swap::endpoints::getWrappedEgldTokenIdentifier(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getUnusedWrappedEgld() {
    egld_esdt_swap::endpoints::getUnusedWrappedEgld(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn callBack() {
    egld_esdt_swap::endpoints::callBack(elrond_wasm_node::arwen_api());
}
