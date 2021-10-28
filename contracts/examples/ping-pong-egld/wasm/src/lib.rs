////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    ping_pong_egld::endpoints::init(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn ping() {
    ping_pong_egld::endpoints::ping(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn pong() {
    ping_pong_egld::endpoints::pong(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn pongAll() {
    ping_pong_egld::endpoints::pongAll(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getUserAddresses() {
    ping_pong_egld::endpoints::getUserAddresses(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getPingAmount() {
    ping_pong_egld::endpoints::getPingAmount(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getDeadline() {
    ping_pong_egld::endpoints::getDeadline(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getActivationTimestamp() {
    ping_pong_egld::endpoints::getActivationTimestamp(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getMaxFunds() {
    ping_pong_egld::endpoints::getMaxFunds(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getUserStatus() {
    ping_pong_egld::endpoints::getUserStatus(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn pongAllLastUser() {
    ping_pong_egld::endpoints::pongAllLastUser(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn callBack() {
    ping_pong_egld::endpoints::callBack(elrond_wasm_node::arwen_api());
}
