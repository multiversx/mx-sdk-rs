////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    lottery_esdt::endpoints::init(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    lottery_esdt::endpoints::callBack(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn buy_ticket() {
    lottery_esdt::endpoints::buy_ticket(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn createLotteryPool() {
    lottery_esdt::endpoints::createLotteryPool(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn determine_winner() {
    lottery_esdt::endpoints::determine_winner(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn getLotteryInfo() {
    lottery_esdt::endpoints::getLotteryInfo(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn start() {
    lottery_esdt::endpoints::start(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn status() {
    lottery_esdt::endpoints::status(elrond_wasm_node::vm_api());
}
